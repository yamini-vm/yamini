use yamini::binread::read_from_file;
use yamini::instructions::InstructionSet;

#[test]
fn test_read_from_file() {
    let program = read_from_file("tests/data/calc.bin");

    assert_eq!(program.len(), 10);

    let expected_program = vec![
        InstructionSet::LOAD(3),
        InstructionSet::LOAD(4),
        InstructionSet::ADD,
        InstructionSet::LOAD(2),
        InstructionSet::SUB,
        InstructionSet::LOAD(3),
        InstructionSet::MUL,
        InstructionSet::LOAD(5),
        InstructionSet::DIV,
        InstructionSet::RET,
    ];

    assert_eq!(program, expected_program);
}