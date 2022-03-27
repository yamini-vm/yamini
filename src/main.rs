use std::io;

use yamini::instructions::InstructionSet;
use yamini::memory::Stack;
use yamini::processor::Processor;

fn main() {
    let mut program = Vec::new();

    program.push(InstructionSet::LOAD(3));
    program.push(InstructionSet::LOAD(4));
    program.push(InstructionSet::ADD);
    program.push(InstructionSet::RET);

    let mut stack = Stack::new();

    let processor = Processor::new();

    for instruction in &program {
        processor.execute(&instruction, &mut stack, &mut io::stdout());
    }
}
