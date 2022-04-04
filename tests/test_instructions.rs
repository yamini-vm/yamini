use yamini::instructions::InstructionSet;

#[test]
fn test_instruction_equality() {
    let instruction = InstructionSet::LOAD(3);
    assert_eq!(instruction, InstructionSet::LOAD(3));

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

    let instruction = InstructionSet::LOADLABEL;
    assert_eq!(instruction, InstructionSet::LOADLABEL);

    let instruction = InstructionSet::JMP(3);
    assert_eq!(instruction, InstructionSet::JMP(3));

    let instruction = InstructionSet::LOADREGISTER(3);
    assert_eq!(instruction, InstructionSet::LOADREGISTER(3));

    let instruction = InstructionSet::POPREGISTER(2);
    assert_eq!(instruction, InstructionSet::POPREGISTER(2));
}

#[test]
fn test_instruction_from_int() {
    let instruction = InstructionSet::from_int(0, Some(2));
    assert_eq!(instruction, InstructionSet::LOAD(2));

    let instruction = InstructionSet::from_int(1, None);
    assert_eq!(instruction, InstructionSet::ADD);

    let instruction = InstructionSet::from_int(2, None);
    assert_eq!(instruction, InstructionSet::SUB);

    let instruction = InstructionSet::from_int(3, None);
    assert_eq!(instruction, InstructionSet::MUL);

    let instruction = InstructionSet::from_int(4, None);
    assert_eq!(instruction, InstructionSet::DIV);

    let instruction = InstructionSet::from_int(5, None);
    assert_eq!(instruction, InstructionSet::RET);

    let instruction = InstructionSet::from_int(6, None);
    assert_eq!(instruction, InstructionSet::MOD);

    let instruction = InstructionSet::from_int(7, None);
    assert_eq!(instruction, InstructionSet::LOADLABEL);

    let instruction = InstructionSet::from_int(8, Some(2));
    assert_eq!(instruction, InstructionSet::JMP(2));

    let instruction = InstructionSet::from_int(9, Some(2));
    assert_eq!(instruction, InstructionSet::LOADREGISTER(2));

    let instruction = InstructionSet::from_int(10, Some(2));
    assert_eq!(instruction, InstructionSet::POPREGISTER(2));
}