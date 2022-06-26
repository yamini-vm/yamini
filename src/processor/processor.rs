extern crate serde_json;
extern crate serde;

use std::collections::HashMap;
use std::{io, fs, path, collections};
use crate::instructions::InstructionSet;
use crate::memory::stack::Stack;
use crate::memory::{ProgramMemory, DataMemory, InnerData};
use serde::{Serialize, Deserialize};

use super::constants::{REGISTER_OFFSET, STACK_OFFSET, STACK_OFFSET_STR, DATA_MEMORY_OFFSET};
use super::constants::{ADDR_OFFSET, PTR_OFFSET};
use super::constants::{DEBUG_DIR, DEBUGGING_JSON_FILE, CODE_LINES_JSON_FILE};


#[derive(Serialize, Deserialize, Clone)]
#[allow(dead_code)]
struct FlagRegister {
    zero: bool,
    negative: bool,
    unused_3: bool,
    unused_4: bool,
    unused_5: bool,
    unused_6: bool,
    unused_7: bool,
    unused_8: bool,
}

impl FlagRegister {
    fn new() -> FlagRegister {
        FlagRegister {
            zero: false,
            negative: false,
            unused_3: false,
            unused_4: false,
            unused_5: false,
            unused_6: false,
            unused_7: false,
            unused_8: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum JsonValue {
    STRING(String),
    NUMBER(i64),
    ARRAY([i8; 10]),
    STACK { data: Vec<InnerData>, head: usize },
    MAP(HashMap<u8, InnerData>),
    FLAGREGISTER(FlagRegister),
    INSTRUCTION { instruction: String, arg: String },
    VECTOR(Vec<JsonValue>),
}

#[allow(dead_code)]
pub struct Processor {
    pc: usize,
    registers: [i8; 10],
    flag_register: FlagRegister,
    debug: bool,
    debug_vec: Vec<collections::HashMap<String, JsonValue>>,
}

impl Processor {
    pub fn new(debug: bool) -> Processor {
        Processor {
            pc: 0,
            registers: [0; 10],
            flag_register: FlagRegister::new(),
            debug,
            debug_vec: Vec::new(),
        }
    }

    fn get_stack_instruction(&self, stack: &Stack, instruction: &str) -> JsonValue {
        let arg = match instruction {
            "PUSH" => stack.top().to_string(),
            "POP" => "".to_string(),
            _ => "".to_string(),
        };

       JsonValue::INSTRUCTION { 
            instruction: instruction.to_string(), 
            arg,
        }
    }

    pub fn execute(&mut self, instruction: &InstructionSet, data_memory: &mut DataMemory,
                   stack: &mut Stack, call_stack: &mut Stack,
                   stdout: &mut dyn io::Write) {
        let mut instruction_map = HashMap::new();
        if self.debug {
            instruction_map.insert("instruction".to_string(), JsonValue::STRING(format!("{:?}", instruction)));
            instruction_map.insert("pc".to_string(), JsonValue::NUMBER(self.pc as i64));
            instruction_map.insert("b_registers".to_string(), JsonValue::ARRAY(self.registers));
            instruction_map.insert("b_data_memory".to_string(), JsonValue::MAP(data_memory.data.clone()));
            instruction_map.insert("b_flag_register".to_string(), JsonValue::FLAGREGISTER(self.flag_register.clone()));
        }

        match instruction {
            InstructionSet::LOAD(value, offset) => {
                if offset == &REGISTER_OFFSET {
                    if value.get_i8() < 0 || value.get_i8() as usize > self.registers.len() {
                        panic!("Register index out of bounds!");
                    }

                    stack.push(InnerData::INT(self.registers[value.get_u8() as usize]));
                } else if offset == &STACK_OFFSET || offset == &STACK_OFFSET_STR {
                    stack.push(value.clone());
                } else if offset == &DATA_MEMORY_OFFSET {
                    stack.push(data_memory.get_var_value(value.get_u8()).clone());
                } else if offset == &ADDR_OFFSET {
                    stack.push(InnerData::INT(value.get_u8() as i8 * 8));
                } else {
                    panic!("Invalid offset!");
                }
            },
            InstructionSet::ADD => {
                let b = match stack.pop() {
                    Some(value) => value,
                    None => panic!("Stack is empty!"),
                };
                let a = match stack.pop() {
                    Some(value) => value,
                    None => panic!("Stack is empty!"),
                };

                stack.push(a + b);
            },
            InstructionSet::SUB => {
                let b = match stack.pop() {
                    Some(value) => value,
                    None => panic!("Stack is empty!"),
                };
                let a = match stack.pop() {
                    Some(value) => value,
                    None => panic!("Stack is empty!"),
                };

                stack.push(a - b);
            },
            InstructionSet::MUL => {
                let b = match stack.pop() {
                    Some(value) => value,
                    None => panic!("Stack is empty!"),
                };
                let a = match stack.pop() {
                    Some(value) => value,
                    None => panic!("Stack is empty!"),
                };

                stack.push(a * b);
            },
            InstructionSet::DIV => {
                let b = match stack.pop() {
                    Some(value) => value,
                    None => panic!("Stack is empty!"),
                };
                let a = match stack.pop() {
                    Some(value) => value,
                    None => panic!("Stack is empty!"),
                };

                stack.push(a / b);
            },
            InstructionSet::HALT  => {},
            InstructionSet::SHOW => {
                if let Some(value) = stack.pop() {
                    match writeln!(stdout, "{}", value) {
                        Ok(_) => (),
                        Err(error) => panic!("{}", error),
                    }
                }
            },
            InstructionSet::MOD => {
                let b = match stack.pop() {
                    Some(value) => value,
                    None => panic!("Stack is empty!"),
                };
                let a = match stack.pop() {
                    Some(value) => value,
                    None => panic!("Stack is empty!"),
                };

                stack.push(a % b);
            }
            InstructionSet::LABEL => {},
            InstructionSet::JMP(label) => {
                self.pc = label.get_u8() as usize;
            },
            InstructionSet::POP(value, offset) => {
                if offset == &REGISTER_OFFSET {
                    if value.get_i8() < 0 || value.get_i8() as usize > self.registers.len() {
                        panic!("Register index out of bounds!");
                    }

                    self.registers[value.get_u8() as usize] = match stack.pop() {
                        Some(value) => value.get_i8(),
                        None => panic!("Stack is empty!"),
                    };
                } else if offset == &DATA_MEMORY_OFFSET {
                    let data_val = match stack.pop() {
                        Some(value) => value,
                        None => panic!("Stack is empty!"),
                    };

                    data_memory.set_var_value(value.get_u8(), data_val);
                } else if offset == &PTR_OFFSET {
                    let address = data_memory.get_var_value(value.get_u8()).clone();

                    let data_val = match stack.pop() {
                        Some(value) => value,
                        None => panic!("Stack is empty!"),
                    };

                    data_memory.set_var_value(address.get_u8() / 8, data_val);
                } else {
                    panic!("Invalid offset!");
                }
            },
            InstructionSet::JZ(label) => {
                if self.flag_register.zero {
                    self.pc = label.get_i8() as usize;
                }
            },
            InstructionSet::JN(label) => {
                if self.flag_register.negative {
                    self.pc = label.get_i8() as usize;
                }
            },
            InstructionSet::STARTSTR => {},
            InstructionSet::ENDSTR => {},
            InstructionSet::RET => {
                if let Some(value) = call_stack.pop() {
                    self.pc = value.get_u8() as usize;
                }
            },
            InstructionSet::CALL(label) => {
                call_stack.push(InnerData::INT(self.pc as i8));
                self.pc = label.get_u8() as usize;
            },
            InstructionSet::EQU => {
                let b = match stack.pop() {
                    Some(value) => value,
                    None => panic!("Stack is empty!"),
                };
                let a = match stack.pop() {
                    Some(value) => value,
                    None => panic!("Stack is empty!"),
                };

                match (a, b) {
                    (InnerData::INT(a), InnerData::INT(b)) => {
                        stack.push(InnerData::INT(if a == b { 1 } else { 0 }));
                    },
                    (InnerData::STR(a), InnerData::STR(b)) => {
                        stack.push(InnerData::INT(if a == b { 1 } else { 0 }));
                    },
                    _ => {
                        stack.push(InnerData::INT(0));
                    }
                }
            },
            InstructionSet::NEG => {
                let value = match stack.pop() {
                    Some(value) => value,
                    None => panic!("Stack is empty!"),
                };

                match value {
                    InnerData::INT(value) => stack.push(InnerData::INT(-value)),
                    _ => panic!("Invalid type!"),
                }
            },
            InstructionSet::DEREF => {
                let value = match stack.pop() {
                    Some(value) => value,
                    None => panic!("Stack is empty!"),
                };

                match value {
                    InnerData::INT(value) => stack.push(
                        InnerData::INT(data_memory.get_var_value(value as u8 / 8).get_i8())
                    ),
                    _ => panic!("Invalid type!"),
                }
            },
        }

        if stack.data.len() > 0 {
            match stack.top() {
                InnerData::INT(top_val) => {
                    if stack.data().len() > 0 && *top_val == 0 {
                        self.flag_register.zero = true;
                    } else if stack.data.len() > 0 && *top_val < 0 {
                        self.flag_register.negative = true;
                    } else {
                        self.flag_register.zero = false;
                        self.flag_register.negative = false;
                    }
                },
                _ => {},
            }
        }

        if self.debug {
            instruction_map.insert("a_registers".to_string(), JsonValue::ARRAY(self.registers));
            instruction_map.insert("a_data_memory".to_string(), JsonValue::MAP(data_memory.data.clone()));
            instruction_map.insert("a_flag_register".to_string(), JsonValue::FLAGREGISTER(self.flag_register.clone()));

            let mut stack_instructions = Vec::new();
            let mut call_stack_instructions = Vec::new();

            match instruction {
                InstructionSet::LOAD(_, _) => {
                    stack_instructions.push(self.get_stack_instruction(stack, "PUSH"));
                },
                InstructionSet::ADD | InstructionSet::SUB | InstructionSet::MUL |
                InstructionSet::DIV | InstructionSet::MOD | InstructionSet::EQU => {
                    stack_instructions.push(self.get_stack_instruction(stack, "POP"));
                    stack_instructions.push(self.get_stack_instruction(stack, "POP"));
                    stack_instructions.push(self.get_stack_instruction(stack, "PUSH"));
                }
                InstructionSet::SHOW | InstructionSet::POP(_, _) => {
                    stack_instructions.push(self.get_stack_instruction(stack, "POP"));
                }
                InstructionSet::NEG | InstructionSet::DEREF => {
                    stack_instructions.push(self.get_stack_instruction(stack, "POP"));
                    stack_instructions.push(self.get_stack_instruction(stack, "PUSH"));
                }

                InstructionSet::CALL(_) => {
                    call_stack_instructions.push(self.get_stack_instruction(call_stack, "PUSH"));
                }
                InstructionSet::RET => {
                    call_stack_instructions.push(self.get_stack_instruction(call_stack, "POP"));
                }
                _ => {},
            }
            instruction_map.insert("stack".to_string(), JsonValue::VECTOR(stack_instructions));
            instruction_map.insert("call_stack".to_string(), JsonValue::VECTOR(call_stack_instructions));

            self.debug_vec.push(instruction_map);
        }
    }

    pub fn execute_program(&mut self, program_memory: ProgramMemory, data_memory: &mut DataMemory,
                           stack: &mut Stack, call_stack: &mut Stack,
                           stdout: &mut dyn io::Write) {
        if self.debug {
            match fs::create_dir_all(DEBUG_DIR) {
                Ok(_) => {
                    let debug_file_path = path::Path::new(DEBUG_DIR).join(DEBUGGING_JSON_FILE);

                    match fs::File::create(debug_file_path) {
                        Ok(_) => (),
                        Err(error) => panic!("Could not create file: {}", error),
                    }
                },
                Err(error) => panic!("Could not create directory: {}", error),
            }

            let mut code_lines = Vec::new();
            for instruction in &program_memory.program {
                code_lines.push(format!("{:?}", instruction));
            }

            let json = serde_json::to_string_pretty(&code_lines).unwrap();

            let code_lines_path = path::Path::new(DEBUG_DIR).join(CODE_LINES_JSON_FILE);
            match fs::write(code_lines_path, json) {
                Ok(_) => (),
                Err(error) => panic!("Could not write to file: {}", error),
            }
        }

        loop {
            let instruction = program_memory.get_instruction(self.pc);
            self.execute(instruction, data_memory, stack, call_stack, stdout);

            self.pc += 1;

            if *instruction == InstructionSet::HALT {
                break;
            }
        }   

        if self.debug {
            let json = serde_json::to_string_pretty(&self.debug_vec).unwrap();

            let debug_file_path = path::Path::new(DEBUG_DIR).join(DEBUGGING_JSON_FILE);
            match fs::write(debug_file_path, json) {
                Ok(_) => (),
                Err(error) => panic!("Could not write to file: {}", error),
            }
        }
    }
}