extern crate sargparse;

use std::io;
use sargparse::{ArgumentParser, ArgumentType, InnerData};

use yamini::memory::DataMemory;
use yamini::memory::Stack;
use yamini::memory::{ProgramMemory};
use yamini::processor::Processor;
use yamini::binread::read_from_file;

fn main() {
    let mut parser = ArgumentParser::new(Some("YamASM - Assembler for YaminiVM"));

    parser.add_argument("f", "file_path", "File path to executable binary", 
                        true, None, ArgumentType::STR);
    parser.add_argument("-i", "--instructions", "Flag to print compiled instructions",
                        false, Some(InnerData::BOOL(false)), ArgumentType::BOOL);

    let args = parser.parse_args().unwrap();

    let filepath = &args.get("file_path").unwrap().get_str();
    let instructions_flag = args.get("instructions").unwrap().get_bool();

    let program = read_from_file(filepath);

    if instructions_flag {
        println!("--------------------------------------------");
        println!("Instructions:");
        for instruction in &program {
            println!("{:?}", instruction);
        }
        println!("--------------------------------------------");
    }

    let mut stack = Stack::new();
    let mut call_stack = Stack::new();

    let mut program_memory = ProgramMemory::new();
    program_memory.load_program(program);

    let mut data_memory = DataMemory::new();

    let mut processor = Processor::new();
    processor.execute_program(program_memory, &mut data_memory, &mut stack, &mut call_stack, &mut io::stdout());
}
