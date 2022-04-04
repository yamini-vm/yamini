use std::io;
use crate::instructions::InstructionSet;
use crate::memory::stack::Stack;
use crate::memory::{Memory, InnerData};


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


pub struct Processor {
    pc: usize,
    registers: [InnerData; 10],
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
            InstructionSet::LOAD(value) => {
                stack.push(*value);
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
            InstructionSet::LOADLABEL => {},
            InstructionSet::JMP(value) => {
                self.pc = *value as usize;
            },
            InstructionSet::LOADREGISTER(register_idx) => {
                if *register_idx < 0 || (*register_idx as usize) > self.registers.len() {
                    panic!("Register index out of bounds!");
                }

                stack.push(self.registers[*register_idx as usize]);
            },
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