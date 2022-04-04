use std::io;
use crate::instructions::InstructionSet;
use crate::memory::stack::Stack;
use crate::memory::{Memory, InnerData};

pub struct Processor {
    pc: usize,
    registers: [InnerData; 10],
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            pc: 0,
            registers: [0; 10],
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