use crate::instructions::InstructionSet;

pub struct Memory {
    pub data: Vec<InstructionSet>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            data: Vec::new(),
        }
    }

    pub fn get_value(&self, idx: usize) -> &InstructionSet {
        &self.data[idx]
    }

    pub fn set_value(&mut self, idx: usize, value: InstructionSet) {
        self.data[idx] = value;
    }

    pub fn add_value(&mut self, value: InstructionSet) -> u8 {
        self.data.push(value);
        self.data.len() as u8 - 1
    }

    pub fn load_program(&mut self, program: Vec<InstructionSet>) {
        self.data = program;
    }
}