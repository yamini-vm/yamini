use std::io;
use crate::instructions::InstructionSet;
use crate::memory::stack::Stack;
use crate::memory::Memory;

pub struct Processor {
    pc: usize,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            pc: 0,
        }
    }

    pub fn execute(&self, instruction: &InstructionSet, stack: &mut Stack, stdout: &mut dyn io::Write) {
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