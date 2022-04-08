use std::io;
use crate::instructions::InstructionSet;
use crate::memory::stack::Stack;
use crate::memory::{Memory, InnerData};

use super::constants::{REGISTER_OFFSET, STACK_OFFSET};


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

    pub fn execute(&mut self, instruction: &InstructionSet, stack: &mut Stack, stdout: &mut dyn io::Write) {
        match instruction {
            InstructionSet::LOAD(value, offset) => {
                if offset == &REGISTER_OFFSET {
                    if value.get_i8() < 0 || value.get_i8() as usize > self.registers.len() {
                        panic!("Register index out of bounds!");
                    }

                    stack.push(InnerData::INT(self.registers[value.get_u8() as usize]));
                } else if offset == &STACK_OFFSET {
                    stack.push(value.clone());
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
            InstructionSet::RET => {
                if let Some(value) = stack.pop() {
                    match writeln!(stdout, "{}", value) {
                        Ok(_) => (),
                        Err(error) => panic!("{}", error),
                    }
                } else {
                    panic!("Stack is empty!")
                };
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
            InstructionSet::POP(register_idx, offset) => {
                if offset == &REGISTER_OFFSET {
                    if register_idx.get_i8() < 0 || register_idx.get_i8() as usize > self.registers.len() {
                        panic!("Register index out of bounds!");
                    }

                    self.registers[register_idx.get_u8() as usize] = match stack.pop() {
                        Some(value) => value.get_i8(),
                        None => panic!("Stack is empty!"),
                    };
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

    pub fn execute_program(&mut self, memory: Memory, stack: &mut Stack, stdout: &mut dyn io::Write) {
        loop {
            let instruction = memory.get_value(self.pc);
            self.execute(instruction, stack, stdout);

            self.pc += 1;

            if *instruction == InstructionSet::RET {
                break;
            }
        }   
    }
}