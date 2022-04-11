use yamini::processor::Processor;
use yamini::memory::{Stack, ProgramMemory, InnerData, DataMemory};
use yamini::instructions::InstructionSet;


#[test]
fn test_execute_load() {
    let mut stack = Stack::new();

    let mut processor = Processor::new();

    processor.execute(
        &InstructionSet::LOAD(InnerData::INT(3), 2),
        &mut DataMemory::new(),
        &mut stack, 
        &mut Stack::new(),
        &mut Vec::new()
    );

    assert_eq!(stack.data(), &[InnerData::INT(3)]);
    assert_eq!(stack.head(), 1);
}

#[test]
fn test_execute_add() {
    let mut stack = Stack::new();
    stack.push(InnerData::INT(3));
    stack.push(InnerData::INT(4));

    let mut processor = Processor::new();

    processor.execute(
        &InstructionSet::ADD, 
        &mut DataMemory::new(),
        &mut stack, 
        &mut Stack::new(), 
        &mut Vec::new()
    );

    assert_eq!(stack.data(), &[InnerData::INT(7)]);
    assert_eq!(stack.head(), 1);
}

#[test]
fn test_execute_sub() {
    let mut stack = Stack::new();
    stack.push(InnerData::INT(3));
    stack.push(InnerData::INT(4));

    let mut processor = Processor::new();

    processor.execute(
        &InstructionSet::SUB, 
        &mut DataMemory::new(),
        &mut stack, 
        &mut Stack::new(), 
        &mut Vec::new()
    );

    assert_eq!(stack.data(), &[InnerData::INT(-1)]);
    assert_eq!(stack.head(), 1);
}

#[test]
fn test_execute_mul() {
    let mut stack = Stack::new();
    stack.push(InnerData::INT(3));
    stack.push(InnerData::INT(4));

    let mut processor = Processor::new();

    processor.execute(
        &InstructionSet::MUL, 
        &mut DataMemory::new(),
        &mut stack, 
        &mut Stack::new(), 
        &mut Vec::new()
    );

    assert_eq!(stack.data(), &[InnerData::INT(12)]);
    assert_eq!(stack.head(), 1);
}

#[test]
fn test_execute_div() {
    let mut stack = Stack::new();
    stack.push(InnerData::INT(12));
    stack.push(InnerData::INT(4));

    let mut processor = Processor::new();

    processor.execute(
        &InstructionSet::DIV, 
        &mut DataMemory::new(),
        &mut stack, 
        &mut Stack::new(), 
        &mut Vec::new()
    );

    assert_eq!(stack.data(), &[InnerData::INT(3)]);
    assert_eq!(stack.head(), 1);
}

#[test]
fn test_execute_halt() {
    let mut stack = Stack::new();
    stack.push(InnerData::INT(3));

    let mut processor = Processor::new();

    let mut stdout = Vec::new();

    processor.execute(
        &InstructionSet::HALT, 
        &mut DataMemory::new(),
        &mut stack, 
        &mut Stack::new(), 
        &mut stdout
    );

    assert_eq!(stack.data(), &[InnerData::INT(3)]);
    assert_eq!(stack.head(), 1);
}

#[test]
fn test_execute_mod() {
    let mut stack = Stack::new();
    stack.push(InnerData::INT(12));
    stack.push(InnerData::INT(5));

    let mut processor = Processor::new();

    processor.execute(
        &InstructionSet::DIV,
        &mut DataMemory::new(),
        &mut stack, 
        &mut Stack::new(), 
        &mut Vec::new()
    );

    assert_eq!(stack.data(), &[InnerData::INT(2)]);
    assert_eq!(stack.head(), 1);
}

#[test]
fn test_execute_loadlabel() {
    let mut stack = Stack::new();
    let mut processor = Processor::new();

    processor.execute(
        &InstructionSet::LABEL,
        &mut DataMemory::new(),
        &mut stack, 
        &mut Stack::new(), 
        &mut Vec::new()
    );

    assert_eq!(stack.data(), &[]);
    assert_eq!(stack.head(), 0);
}

#[test]
fn test_execute_jmp() {
    let mut stack = Stack::new();
    let mut processor = Processor::new();

    processor.execute(
        &InstructionSet::JMP(InnerData::INT(2)),
        &mut DataMemory::new(),
        &mut stack, 
        &mut Stack::new(),  
        &mut Vec::new()
    );

    assert_eq!(stack.data(), &[]);
    assert_eq!(stack.head(), 0);
}

#[test]
fn test_execute_popregister() {
    let mut stack = Stack::new();
    let mut processor = Processor::new();

    processor.execute(
        &InstructionSet::LOAD(InnerData::INT(2), 1), 
        &mut DataMemory::new(),
        &mut stack, 
        &mut Stack::new(),
        &mut Vec::new()
    );

    processor.execute(
        &InstructionSet::POP(InnerData::INT(2), 1), 
        &mut DataMemory::new(),
        &mut stack, 
        &mut Stack::new(),
        &mut Vec::new()
    );

    assert_eq!(stack.data(), &[]);
    assert_eq!(stack.head(), 0);
}

#[test]
fn test_execute_jz() {
    let mut stack = Stack::new();
    let mut processor = Processor::new();

    processor.execute(
        &InstructionSet::JZ(InnerData::INT(2)),
        &mut DataMemory::new(),
        &mut stack, 
        &mut Stack::new(),
        &mut Vec::new()
    );

    assert_eq!(stack.data(), &[]);
    assert_eq!(stack.head(), 0);
}

#[test]
fn test_execute_jn() {
    let mut stack = Stack::new();
    let mut processor = Processor::new();

    processor.execute(
        &InstructionSet::JN(InnerData::INT(2)), 
        &mut DataMemory::new(),
        &mut stack, 
        &mut Stack::new(),
        &mut Vec::new()
    );

    assert_eq!(stack.data(), &[]);
    assert_eq!(stack.head(), 0);
}

#[test]
fn test_execute_show() {
    let mut stack = Stack::new();
    stack.push(InnerData::INT(3));

    let mut processor = Processor::new();

    let mut stdout = Vec::new();

    processor.execute(
        &InstructionSet::SHOW, 
        &mut DataMemory::new(),
        &mut stack, 
        &mut Stack::new(), 
        &mut stdout
    );

    assert_eq!(stack.data(), &[]);
    assert_eq!(stack.head(), 0);

    assert_eq!(String::from_utf8(stdout).unwrap(), "3\n");
}

#[test]
fn test_execute_ret() {
    let mut stack = Stack::new();

    let mut processor = Processor::new();

    let mut stdout = Vec::new();

    processor.execute(
        &InstructionSet::RET,
        &mut DataMemory::new(),
        &mut stack, 
        &mut Stack::new(), 
        &mut stdout
    );

    assert_eq!(stack.data(), &[]);
    assert_eq!(stack.head(), 0);
}

#[test]
fn test_execute_call() {
    let mut stack = Stack::new();

    let mut processor = Processor::new();

    let mut stdout = Vec::new();

    processor.execute(
        &InstructionSet::CALL(InnerData::INT(2)), 
        &mut DataMemory::new(),
        &mut stack, 
        &mut Stack::new(),
        &mut stdout
    );

    assert_eq!(stack.data(), &[]);
    assert_eq!(stack.head(), 0);
}

#[test]
fn test_execute_equ() {
    let mut stack = Stack::new();
    stack.push(InnerData::INT(3));

    let mut processor = Processor::new();

    processor.execute(
        &InstructionSet::EQU(InnerData::INT(3), 2),
        &mut DataMemory::new(),
        &mut stack, 
        &mut Stack::new(), 
        &mut Vec::new()
    );

    assert_eq!(stack.data(), &[InnerData::INT(3), InnerData::INT(1)]);
    assert_eq!(stack.head(), 2);
}

#[test]
fn test_execute_program() {
    let mut program = Vec::new();

    program.push(InstructionSet::LOAD(InnerData::INT(3), 2));
    program.push(InstructionSet::LOAD(InnerData::INT(4), 2));
    program.push(InstructionSet::MUL);
    program.push(InstructionSet::HALT);

    let mut stack = Stack::new();

    let mut memory = ProgramMemory::new();
    memory.load_program(program);

    let mut processor = Processor::new();

    let mut stdout = Vec::new();

    processor.execute_program(
        memory, 
        &mut DataMemory::new(),
        &mut stack, 
        &mut Stack::new(), 
        &mut stdout
    );

    assert_eq!(stack.data(), &[InnerData::INT(12)]);
    assert_eq!(stack.head(), 1);
}