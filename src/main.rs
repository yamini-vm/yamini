use std::io;

use yamini::instructions::InstructionSet;
use yamini::memory::Stack;
use yamini::memory::Memory;
use yamini::processor::Processor;

fn main() {
    let mut program = Vec::new();

    program.push(InstructionSet::LOAD(3));
    program.push(InstructionSet::LOAD(4));
    program.push(InstructionSet::ADD);
    program.push(InstructionSet::LOAD(2));
    program.push(InstructionSet::SUB);
    program.push(InstructionSet::LOAD(3));
    program.push(InstructionSet::MUL);
    program.push(InstructionSet::LOAD(5));
    program.push(InstructionSet::DIV);
    program.push(InstructionSet::RET);

    let mut stack = Stack::new();

    let mut memory = Memory::new();
    memory.load_program(program);

    let mut processor = Processor::new();
    processor.execute_program(memory, &mut stack, &mut io::stdout());
}
