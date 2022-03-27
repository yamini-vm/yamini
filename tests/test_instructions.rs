use yamini::instructions::InstructionSet;

#[test]
fn test_instruction_equality() {
    let instruction = InstructionSet::LOAD(3);
    assert_eq!(instruction, InstructionSet::LOAD(3));

    let instruction = InstructionSet::ADD;
    assert_eq!(instruction, InstructionSet::ADD);

    let instruction = InstructionSet::RET;
    assert_eq!(instruction, InstructionSet::RET);
}