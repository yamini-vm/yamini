use crate::memory::InnerData;

#[derive(Debug)]
pub enum InstructionSet {
    LOAD(InnerData),
    ADD,
    SUB,
    MUL,
    DIV,
    RET,
}

impl PartialEq for InstructionSet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (InstructionSet::LOAD(a), InstructionSet::LOAD(b)) => a == b,
            (InstructionSet::ADD, InstructionSet::ADD) => true,
            (InstructionSet::SUB, InstructionSet::SUB) => true,
            (InstructionSet::MUL, InstructionSet::MUL) => true,
            (InstructionSet::DIV, InstructionSet::DIV) => true,
            (InstructionSet::RET, InstructionSet::RET) => true,
            _ => false,
        }
    }
}