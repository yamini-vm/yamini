use yamini::memory::{Stack, Memory};
use yamini::instructions::InstructionSet;

#[test]
fn test_stack_push() {
    let mut stack = Stack::new();
    stack.push(3);
    stack.push(4);

    assert_eq!(stack.data(), &[3, 4]);
    assert_eq!(stack.head(), 2);
}

#[test]
fn test_stack_pop() {
    let mut stack = Stack::new();
    stack.push(3);
    stack.push(4);

    assert_eq!(stack.pop(), Some(4));
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), None);
}

#[test]
fn test_stack_top() {
    let mut stack = Stack::new();
    stack.push(3);
    stack.push(4);

    assert_eq!(*stack.top(), 4);
}

#[test]
fn test_memory_get_value() {
    let mut memory = Memory::new();
    memory.add_value(InstructionSet::LOAD(3, 100));

    let instruction = memory.get_value(0);

    assert_eq!(*instruction, InstructionSet::LOAD(3, 100));
}

#[test]
fn test_memory_set_value() {
    let mut memory = Memory::new();
    memory.add_value(InstructionSet::LOAD(3, 100));

    memory.set_value(0, InstructionSet::LOAD(4, 100));

    let instruction = memory.get_value(0);

    assert_eq!(*instruction, InstructionSet::LOAD(4, 100));
}

#[test]
fn test_memory_add_value() {
    let mut memory = Memory::new();

    let idx = memory.add_value(InstructionSet::LOAD(3, 100));

    assert_eq!(idx, 0);
}

#[test]
fn test_memory_load_program() {
    let mut memory = Memory::new();
    let mut program = Vec::new();

    program.push(InstructionSet::LOAD(3, 100));
    program.push(InstructionSet::LOAD(4, 100));

    memory.load_program(program);

    assert_eq!(memory.data(), vec![InstructionSet::LOAD(3, 100), InstructionSet::LOAD(4, 100)].as_slice());
}