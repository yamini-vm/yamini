use yamini::binread::read_from_file;
use yamini::instructions::InstructionSet;

#[test]
fn test_read_from_file() {
    let program = read_from_file("tests/data/a.out");

    assert_eq!(program.len(), 4);

    let expected_program = vec![
        InstructionSet::LOAD(3),
        InstructionSet::LOAD(4),
        InstructionSet::ADD,
        InstructionSet::RET,
    ];

    assert_eq!(program, expected_program);
}