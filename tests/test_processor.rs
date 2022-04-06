use yamini::processor::Processor;
use yamini::memory::{Stack, Memory};
use yamini::instructions::InstructionSet;


#[test]
fn test_execute_load() {
    let mut stack = Stack::new();

    let mut processor = Processor::new();

    processor.execute(&InstructionSet::LOAD(3), &mut stack, &mut Vec::new());

    assert_eq!(stack.data(), &[3]);
    assert_eq!(stack.head(), 1);
}

#[test]
fn test_execute_add() {
    let mut stack = Stack::new();
    stack.push(3);
    stack.push(4);

    let mut processor = Processor::new();

    processor.execute(&InstructionSet::ADD, &mut stack, &mut Vec::new());

    assert_eq!(stack.data(), &[7]);
    assert_eq!(stack.head(), 1);
}

#[test]
fn test_execute_sub() {
    let mut stack = Stack::new();
    stack.push(3);
    stack.push(4);

    let mut processor = Processor::new();

    processor.execute(&InstructionSet::SUB, &mut stack, &mut Vec::new());

    assert_eq!(stack.data(), &[-1]);
    assert_eq!(stack.head(), 1);
}

#[test]
fn test_execute_mul() {
    let mut stack = Stack::new();
    stack.push(3);
    stack.push(4);

    let mut processor = Processor::new();

    processor.execute(&InstructionSet::MUL, &mut stack, &mut Vec::new());

    assert_eq!(stack.data(), &[12]);
    assert_eq!(stack.head(), 1);
}

#[test]
fn test_execute_div() {
    let mut stack = Stack::new();
    stack.push(12);
    stack.push(4);

    let mut processor = Processor::new();

    processor.execute(&InstructionSet::DIV, &mut stack, &mut Vec::new());

    assert_eq!(stack.data(), &[3]);
    assert_eq!(stack.head(), 1);
}

#[test]
fn test_execute_ret() {
    let mut stack = Stack::new();
    stack.push(3);

    let mut processor = Processor::new();

    let mut stdout = Vec::new();

    processor.execute(&InstructionSet::RET, &mut stack, &mut stdout);

    assert_eq!(stack.data(), &[]);
    assert_eq!(stack.head(), 0);

    assert_eq!(String::from_utf8(stdout).unwrap(), "3\n");
}

#[test]
fn test_execute_mod() {
    let mut stack = Stack::new();
    stack.push(12);
    stack.push(5);

    let mut processor = Processor::new();

    processor.execute(&InstructionSet::DIV, &mut stack, &mut Vec::new());

    assert_eq!(stack.data(), &[2]);
    assert_eq!(stack.head(), 1);
}

#[test]
fn test_execute_loadlabel() {
    let mut stack = Stack::new();
    let mut processor = Processor::new();

    processor.execute(&InstructionSet::LOADLABEL, &mut stack, &mut Vec::new());

    assert_eq!(stack.data(), &[]);
    assert_eq!(stack.head(), 0);
}

#[test]
fn test_execute_jmp() {
    let mut stack = Stack::new();
    let mut processor = Processor::new();

    processor.execute(&InstructionSet::JMP(2), &mut stack, &mut Vec::new());

    assert_eq!(stack.data(), &[]);
    assert_eq!(stack.head(), 0);
}

#[test]
fn test_execute_loadregister() {
    let mut stack = Stack::new();
    let mut processor = Processor::new();

    processor.execute(&InstructionSet::LOADREGISTER(2), &mut stack, &mut Vec::new());

    assert_eq!(stack.data(), &[0]);
    assert_eq!(stack.head(), 1);
}

#[test]
fn test_execute_popregister() {
    let mut stack = Stack::new();
    let mut processor = Processor::new();

    processor.execute(&InstructionSet::LOAD(2), &mut stack, &mut Vec::new());
    processor.execute(&InstructionSet::POPREGISTER(2), &mut stack, &mut Vec::new());

    assert_eq!(stack.data(), &[]);
    assert_eq!(stack.head(), 0);
}

#[test]
fn test_execute_jz() {
    let mut stack = Stack::new();
    let mut processor = Processor::new();

    processor.execute(&InstructionSet::JZ(2), &mut stack, &mut Vec::new());

    assert_eq!(stack.data(), &[]);
    assert_eq!(stack.head(), 0);
}

#[test]
fn test_execute_program() {
    let mut program = Vec::new();

    program.push(InstructionSet::LOAD(3));
    program.push(InstructionSet::LOAD(4));
    program.push(InstructionSet::MUL);
    program.push(InstructionSet::RET);

    let mut stack = Stack::new();

    let mut memory = Memory::new();
    memory.load_program(program);

    let mut processor = Processor::new();

    let mut stdout = Vec::new();

    processor.execute_program(memory, &mut stack, &mut stdout);

    assert_eq!(stack.data(), &[]);
    assert_eq!(stack.head(), 0);

    assert_eq!(String::from_utf8(stdout).unwrap(), "12\n");
}