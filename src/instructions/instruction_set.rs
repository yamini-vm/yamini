use crate::memory::InnerData;

#[derive(Debug)]
pub enum InstructionSet {
    LOAD(InnerData, u8),
    ADD,
    SUB,
    MUL,
    DIV,
    RET,
    MOD,
    LABEL,
    JMP(InnerData),
    POP(InnerData, u8),
    JZ(InnerData),
    JN(InnerData),
}

impl PartialEq for InstructionSet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (InstructionSet::LOAD(a, b), InstructionSet::LOAD(c, d)) => a == c && b == d,
            (InstructionSet::ADD, InstructionSet::ADD) => true,
            (InstructionSet::SUB, InstructionSet::SUB) => true,
            (InstructionSet::MUL, InstructionSet::MUL) => true,
            (InstructionSet::DIV, InstructionSet::DIV) => true,
            (InstructionSet::RET, InstructionSet::RET) => true,
            (InstructionSet::MOD, InstructionSet::MOD) => true,
            (InstructionSet::LABEL, InstructionSet::LABEL) => true,
            (InstructionSet::JMP(a), InstructionSet::JMP(b)) => a == b,
            (InstructionSet::POP(a, b), InstructionSet::POP(c, d)) => a == c && b == d,
            (InstructionSet::JZ(a), InstructionSet::JZ(b)) => a == b,
            (InstructionSet::JN(a), InstructionSet::JN(b)) => a == b,
            _ => false,
        }
    }
}

impl InstructionSet {
    pub fn from_int(value: u8, arg: Option<InnerData>, arg1: Option<InnerData>) -> Self {
        match value {
            0 => {
                let first_arg = match arg {
                    Some(arg) => arg,
                    None => panic!("InstructionSet::LOAD: arg is None"),
                };

                let second_arg = match arg1 {
                    Some(arg) => arg,
                    None => panic!("InstructionSet::LOAD: arg1 is None"),
                };

                InstructionSet::LOAD(first_arg, second_arg as u8)
            },
            1 => InstructionSet::ADD,
            2 => InstructionSet::SUB,
            3 => InstructionSet::MUL,
            4 => InstructionSet::DIV,
            5 => InstructionSet::RET,
            6 => InstructionSet::MOD,
            7 => InstructionSet::LABEL,
            8 => {
                match arg {
                    Some(arg) => InstructionSet::JMP(arg),
                    None => panic!("InstructionSet::JMP: arg is None"),
                }
            },
            9 => {
                let first_arg = match arg {
                    Some(arg) => arg,
                    None => panic!("InstructionSet::LOAD: arg is None"),
                };

                let second_arg = match arg1 {
                    Some(arg) => arg,
                    None => panic!("InstructionSet::LOAD: arg1 is None"),
                };

                InstructionSet::POP(first_arg, second_arg as u8)
            },
            10 => {
                match arg {
                    Some(arg) => InstructionSet::JZ(arg),
                    None => panic!("InstructionSet::JZ: arg is None"),
                }
            },
            11 => {
                match arg {
                    Some(arg) => InstructionSet::JN(arg),
                    None => panic!("InstructionSet::JN: arg is None"),
                }
            },
            _ => panic!("Invalid instruction set value: {}", value),
        }
    }
}