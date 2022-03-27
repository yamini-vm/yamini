use std::io;

use crate::instructions::InstructionSet;
use crate::memory::stack::Stack;

pub struct Processor {
}

impl Processor {
    pub fn new() -> Processor {
        Processor {

        }
    }

    pub fn execute(&self, instruction: &InstructionSet, stack: &mut Stack, stdout: &mut dyn io::Write) {
        match instruction {
            InstructionSet::LOAD(value) => {
                stack.push(*value);
            },
            InstructionSet::ADD => {
                let a = match stack.pop() {
                    Some(value) => value,
                    None => panic!("Stack is empty!"),
                };
                let b = match stack.pop() {
                    Some(value) => value,
                    None => panic!("Stack is empty!"),
                };

                stack.push(a + b);
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
            }
        }
    }
}