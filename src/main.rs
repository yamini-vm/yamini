use std::io;

use yamini::memory::Stack;
use yamini::memory::Memory;
use yamini::processor::Processor;
use yamini::binread::read_from_file;

fn main() {
    let program = read_from_file("calc.bin");

    let mut stack = Stack::new();

    let mut memory = Memory::new();
    memory.load_program(program);

    let mut processor = Processor::new();
    processor.execute_program(memory, &mut stack, &mut io::stdout());
}
