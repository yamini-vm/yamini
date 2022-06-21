extern crate serde_json;

use std::collections::HashMap;
use std::{io, fs, path, collections};
use crate::instructions::InstructionSet;
use crate::memory::stack::Stack;
use crate::memory::{ProgramMemory, DataMemory, InnerData};

use super::constants::{REGISTER_OFFSET, STACK_OFFSET, STACK_OFFSET_STR, DATA_MEMORY_OFFSET};
use super::constants::{ADDR_OFFSET, PTR_OFFSET};
use super::constants::{DEBUG_DIR, DEBUGGING_JSON_FILE};


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


#[allow(dead_code)]
pub struct Processor {
    pc: usize,
    registers: [i8; 10],
    flag_register: FlagRegister,
    debug: bool,
    debug_vec: Vec<collections::HashMap<String, String>>,
}

impl Processor {
    pub fn new(debug: bool) -> Processor {
        Processor {
            pc: 0,
            registers: [0; 10],
            flag_register: FlagRegister::new(),
            debug: debug,
            debug_vec: Vec::new(),
        }
    }

    pub fn execute(&mut self, instruction: &InstructionSet, data_memory: &mut DataMemory,
                   stack: &mut Stack, call_stack: &mut Stack,
                   stdout: &mut dyn io::Write) {
        let mut instruction_map = HashMap::new();
        if self.debug {
            instruction_map.insert("instruction".to_string(), format!("{:?}", instruction));
            instruction_map.insert("b_pc".to_string(), self.pc.to_string());
            instruction_map.insert("b_registers".to_string(), format!("{:?}", self.registers));
            instruction_map.insert("b_data_memory".to_string(), format!("{:?}", data_memory.data));
            instruction_map.insert("b_stack".to_string(), format!("{:?}", stack));
            instruction_map.insert("b_call_stack".to_string(), format!("{:?}", call_stack));
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
            instruction_map.insert("a_pc".to_string(), self.pc.to_string());
            instruction_map.insert("a_registers".to_string(), format!("{:?}", self.registers));
            instruction_map.insert("a_data_memory".to_string(), format!("{:?}", data_memory.data));
            instruction_map.insert("a_stack".to_string(), format!("{:?}", stack));
            instruction_map.insert("a_call_stack".to_string(), format!("{:?}", call_stack));

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