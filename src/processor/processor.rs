use std::io;
use crate::instructions::InstructionSet;
use crate::memory::stack::Stack;
use crate::memory::{ProgramMemory, DataMemory, InnerData};

use super::constants::{REGISTER_OFFSET, STACK_OFFSET, STACK_OFFSET_STR, DATA_MEMORY_OFFSET};
use super::constants::{DATA_MEMORY_OFFSET_STR, ADDR_OFFSET, PTR_OFFSET};


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
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            pc: 0,
            registers: [0; 10],
            flag_register: FlagRegister::new(),
        }
    }

    pub fn execute(&mut self, instruction: &InstructionSet, data_memory: &mut DataMemory,
                   stack: &mut Stack, call_stack: &mut Stack,
                   stdout: &mut dyn io::Write) {
        match instruction {
            InstructionSet::LOAD(value, offset) => {
                if offset == &REGISTER_OFFSET {
                    if value.get_i8() < 0 || value.get_i8() as usize > self.registers.len() {
                        panic!("Register index out of bounds!");
                    }

                    stack.push(InnerData::INT(self.registers[value.get_u8() as usize]));
                } else if offset == &STACK_OFFSET || offset == &STACK_OFFSET_STR {
                    stack.push(value.clone());
                } else if offset == &DATA_MEMORY_OFFSET || offset == &DATA_MEMORY_OFFSET_STR {
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
    }

    pub fn execute_program(&mut self, program_memory: ProgramMemory, data_memory: &mut DataMemory,
                           stack: &mut Stack, call_stack: &mut Stack,
                           stdout: &mut dyn io::Write) {
        loop {
            let instruction = program_memory.get_instruction(self.pc);
            self.execute(instruction, data_memory, stack, call_stack, stdout);

            self.pc += 1;

            if *instruction == InstructionSet::HALT {
                break;
            }
        }   
    }
}