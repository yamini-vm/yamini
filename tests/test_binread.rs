use yamini::binread::read_from_file;
use yamini::instructions::InstructionSet;
use yamini::memory::InnerData;

#[test]
fn test_read_from_file() {
    let program = read_from_file("tests/data/a.out");

    assert_eq!(program.len(), 4);

    let expected_program = vec![
        InstructionSet::LOAD(InnerData::INT(3), 200),
        InstructionSet::LOAD(InnerData::INT(4), 200),
        InstructionSet::ADD,
        InstructionSet::RET,
    ];

    assert_eq!(program, expected_program);
}