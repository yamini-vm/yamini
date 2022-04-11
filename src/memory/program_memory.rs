use crate::instructions::InstructionSet;

pub struct ProgramMemory {
    pub program: Vec<InstructionSet>,
}

impl ProgramMemory {
    pub fn new() -> ProgramMemory {
        ProgramMemory {
            program: Vec::new(),
        }
    }

    pub fn get_instruction(&self, idx: usize) -> &InstructionSet {
        &self.program[idx]
    }

    pub fn add_instruction(&mut self, value: InstructionSet) -> u8 {
        self.program.push(value);
        self.program.len() as u8 - 1
    }

    pub fn set_instruction(&mut self, idx: usize, value: InstructionSet) {
        self.program[idx] = value;
    }

    pub fn load_program(&mut self, program: Vec<InstructionSet>) {
        self.program = program;
    }

    pub fn program(&self) -> &Vec<InstructionSet> {
        &self.program
    }
}