use yamini::processor::Processor;
use yamini::memory::Stack;
use yamini::instructions::InstructionSet;

#[test]
fn test_execute_load() {
    let mut stack = Stack::new();

    let processor = Processor::new();

    processor.execute(&InstructionSet::LOAD(3), &mut stack, &mut Vec::new());

    assert_eq!(stack.data(), &[3]);
    assert_eq!(stack.head(), 1);
}

#[test]
fn test_execute_add() {
    let mut stack = Stack::new();
    stack.push(3);
    stack.push(4);

    let processor = Processor::new();

    processor.execute(&InstructionSet::ADD, &mut stack, &mut Vec::new());

    assert_eq!(stack.data(), &[7]);
    assert_eq!(stack.head(), 1);
}

#[test]
fn test_execute_ret() {
    let mut stack = Stack::new();
    stack.push(3);

    let processor = Processor::new();

    let mut stdout = Vec::new();

    processor.execute(&InstructionSet::RET, &mut stack, &mut stdout);

    assert_eq!(stack.data(), &[]);
    assert_eq!(stack.head(), 0);

    assert_eq!(String::from_utf8(stdout).unwrap(), "3\n");
}