use std::io;
use std::env;

use yamini::memory::DataMemory;
use yamini::memory::Stack;
use yamini::memory::{ProgramMemory};
use yamini::processor::Processor;
use yamini::binread::read_from_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <filepath>", args[0]);
        return;
    }

    let filepath = &args[1];

    let program = read_from_file(filepath);

    let mut stack = Stack::new();
    let mut call_stack = Stack::new();

    let mut program_memory = ProgramMemory::new();
    program_memory.load_program(program);

    let mut data_memory = DataMemory::new();

    let mut processor = Processor::new();
    processor.execute_program(program_memory, &mut data_memory, &mut stack, &mut call_stack, &mut io::stdout());
}
