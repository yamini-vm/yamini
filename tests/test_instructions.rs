use yamini::{instructions::InstructionSet, memory::InnerData};

#[test]
fn test_instruction_equality() {
    let instruction = InstructionSet::LOAD(InnerData::INT(3), 200);
    assert_eq!(instruction, InstructionSet::LOAD(InnerData::INT(3), 200));

    let instruction = InstructionSet::ADD;
    assert_eq!(instruction, InstructionSet::ADD);

    let instruction = InstructionSet::SUB;
    assert_eq!(instruction, InstructionSet::SUB);

    let instruction = InstructionSet::MUL;
    assert_eq!(instruction, InstructionSet::MUL);

    let instruction = InstructionSet::DIV;
    assert_eq!(instruction, InstructionSet::DIV);

    let instruction = InstructionSet::RET;
    assert_eq!(instruction, InstructionSet::RET);

    let instruction = InstructionSet::MOD;
    assert_eq!(instruction, InstructionSet::MOD);

    let instruction = InstructionSet::LABEL;
    assert_eq!(instruction, InstructionSet::LABEL);

    let instruction = InstructionSet::JMP(InnerData::INT(3));
    assert_eq!(instruction, InstructionSet::JMP(InnerData::INT(3)));

    let instruction = InstructionSet::POP(InnerData::INT(2), 100);
    assert_eq!(instruction, InstructionSet::POP(InnerData::INT(2), 100));

    let instruction = InstructionSet::JZ(InnerData::INT(2));
    assert_eq!(instruction, InstructionSet::JZ(InnerData::INT(2)));

    let instruction = InstructionSet::JN(InnerData::INT(2));
    assert_eq!(instruction, InstructionSet::JN(InnerData::INT(2)));

    let instruction = InstructionSet::STARTSTR;
    assert_eq!(instruction, InstructionSet::STARTSTR);

    let instruction = InstructionSet::ENDSTR;
    assert_eq!(instruction, InstructionSet::ENDSTR);
}

#[test]
fn test_instruction_from_int() {
    let instruction = InstructionSet::from_int(0, Some(InnerData::INT(2)), Some(InnerData::INT(100)));
    assert_eq!(instruction, InstructionSet::LOAD(InnerData::INT(2), 100));

    let instruction = InstructionSet::from_int(1, None, None);
    assert_eq!(instruction, InstructionSet::ADD);

    let instruction = InstructionSet::from_int(2, None, None);
    assert_eq!(instruction, InstructionSet::SUB);

    let instruction = InstructionSet::from_int(3, None, None);
    assert_eq!(instruction, InstructionSet::MUL);

    let instruction = InstructionSet::from_int(4, None, None);
    assert_eq!(instruction, InstructionSet::DIV);

    let instruction = InstructionSet::from_int(5, None, None);
    assert_eq!(instruction, InstructionSet::RET);

    let instruction = InstructionSet::from_int(6, None, None);
    assert_eq!(instruction, InstructionSet::MOD);

    let instruction = InstructionSet::from_int(7, None, None);
    assert_eq!(instruction, InstructionSet::LABEL);

    let instruction = InstructionSet::from_int(8, Some(InnerData::INT(2)), None);
    assert_eq!(instruction, InstructionSet::JMP(InnerData::INT(2)));

    let instruction = InstructionSet::from_int(9, Some(InnerData::INT(2)), Some(InnerData::INT(100)));
    assert_eq!(instruction, InstructionSet::POP(InnerData::INT(2), 100));

    let instruction = InstructionSet::from_int(10, Some(InnerData::INT(2)), None);
    assert_eq!(instruction, InstructionSet::JZ(InnerData::INT(2)));

    let instruction = InstructionSet::from_int(11, Some(InnerData::INT(2)), None);
    assert_eq!(instruction, InstructionSet::JN(InnerData::INT(2)));

    let instruction = InstructionSet::from_int(12, None, None);
    assert_eq!(instruction, InstructionSet::STARTSTR);

    let instruction = InstructionSet::from_int(13, None, None);
    assert_eq!(instruction, InstructionSet::ENDSTR);
}