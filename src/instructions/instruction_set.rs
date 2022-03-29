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

impl InstructionSet {
    pub fn to_int(self) -> u8 {
        match self {
            InstructionSet::LOAD(_) => 0,
            InstructionSet::ADD => 1,
            InstructionSet::SUB => 2,
            InstructionSet::MUL => 3,
            InstructionSet::DIV => 4,
            InstructionSet::RET => 5,
        }
    }

    pub fn from_int(value: u8, arg: Option<InnerData>) -> Self {
        match value {
            0 => {
                match arg {
                    Some(arg) => InstructionSet::LOAD(arg),
                    None => panic!("InstructionSet::LOAD: arg is None"),
                }
            },
            1 => InstructionSet::ADD,
            2 => InstructionSet::SUB,
            3 => InstructionSet::MUL,
            4 => InstructionSet::DIV,
            5 => InstructionSet::RET,
            _ => panic!("Invalid instruction set value: {}", value),
        }
    }
}