#[derive(Debug)]
pub enum InstructionSet {
    LOAD(u8),
    ADD,
    RET,
}

impl PartialEq for InstructionSet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (InstructionSet::LOAD(a), InstructionSet::LOAD(b)) => a == b,
            (InstructionSet::ADD, InstructionSet::ADD) => true,
            (InstructionSet::RET, InstructionSet::RET) => true,
            _ => false,
        }
    }
}