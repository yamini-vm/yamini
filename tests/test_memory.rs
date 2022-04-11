use yamini::memory::{Stack, ProgramMemory, InnerData, DataMemory};
use yamini::instructions::InstructionSet;

#[test]
fn test_stack_push() {
    let mut stack = Stack::new();
    stack.push(InnerData::INT(3));
    stack.push(InnerData::INT(4));

    assert_eq!(stack.data(), &[InnerData::INT(3), InnerData::INT(4)]);
    assert_eq!(stack.head(), 2);
}

#[test]
fn test_stack_pop() {
    let mut stack = Stack::new();
    stack.push(InnerData::INT(3));
    stack.push(InnerData::INT(4));

    assert_eq!(stack.pop(), Some(InnerData::INT(4)));
    assert_eq!(stack.pop(), Some(InnerData::INT(3)));
    assert_eq!(stack.pop(), None);
}

#[test]
fn test_stack_top() {
    let mut stack = Stack::new();
    stack.push(InnerData::INT(3));
    stack.push(InnerData::INT(4));

    assert_eq!(*stack.top(), InnerData::INT(4));
}

#[test]
fn test_program_memory_get_value() {
    let mut memory = ProgramMemory::new();
    memory.add_instruction(InstructionSet::LOAD(InnerData::INT(3), 1));

    let instruction = memory.get_instruction(0);

    assert_eq!(*instruction, InstructionSet::LOAD(InnerData::INT(3), 1));
}

#[test]
fn test_program_memory_set_value() {
    let mut memory = ProgramMemory::new();
    memory.add_instruction(InstructionSet::LOAD(InnerData::INT(3), 1));

    memory.set_instruction(0, InstructionSet::LOAD(InnerData::INT(4), 1));

    let instruction = memory.get_instruction(0);

    assert_eq!(*instruction, InstructionSet::LOAD(InnerData::INT(4), 1));
}

#[test]
fn test_program_memory_add_value() {
    let mut memory = ProgramMemory::new();

    let idx = memory.add_instruction(InstructionSet::LOAD(InnerData::INT(3), 1));

    assert_eq!(idx, 0);
}

#[test]
fn test_program_memory_load_program() {
    let mut memory = ProgramMemory::new();
    let mut program = Vec::new();

    program.push(InstructionSet::LOAD(InnerData::INT(3), 1));
    program.push(InstructionSet::LOAD(InnerData::INT(4), 1));

    memory.load_program(program);

    assert_eq!(memory.program(), vec![
        InstructionSet::LOAD(InnerData::INT(3), 1), 
        InstructionSet::LOAD(InnerData::INT(4), 1)
    ].as_slice());
}

#[test]
fn test_data_memory_get_var_value_existing() {
    let mut memory = DataMemory::new();

    memory.set_var_value(0, InnerData::INT(3));

    assert_eq!(memory.get_var_value(0), &InnerData::INT(3));
}

#[test]
fn test_data_memory_get_var_value_non_existing() {
    let memory = DataMemory::new();

    assert_eq!(memory.get_var_value(0), &InnerData::INT(0));
}

#[test]
fn test_data_memory_set_var_value_non_existing() {
    let mut memory = DataMemory::new();

    memory.set_var_value(0, InnerData::INT(3));

    assert_eq!(memory.get_var_value(0), &InnerData::INT(3));
}

#[test]
fn test_data_memory_set_var_value_existing() {
    let mut memory = DataMemory::new();

    memory.set_var_value(0, InnerData::INT(3));
    memory.set_var_value(0, InnerData::INT(4));

    assert_eq!(memory.get_var_value(0), &InnerData::INT(4));
}