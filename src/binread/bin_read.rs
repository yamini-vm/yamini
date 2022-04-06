use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::instructions::InstructionSet;
use crate::memory::InnerData;


fn read_file_line_by_line(filepath: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);
    let mut file_bytes = Vec::new();

    reader.read_to_end(&mut file_bytes)?;

    Ok(file_bytes)
}

pub fn read_from_file(filepath: &str) -> Vec<InstructionSet> {
    let mut buffer = Vec::new();
    match read_file_line_by_line(filepath) {
        Ok(lines) => {
            buffer = lines;
        },
        Err(e) => println!("Error: {}", e),
    }

    let instruction_with_arg = vec![
        0, // LOAD
        8, // JMP
        9, // LOADREGISTER
        10, // POPREGISTER
        11, // JZ
    ];

    let mut program = Vec::new();
    let mut is_reading_arg = false;
    let mut current_arg_instruction = 0;
    let mut arg: InnerData;

    for byte in buffer {
        if is_reading_arg {
            arg = byte as InnerData;
            let instruction = InstructionSet::from_int(current_arg_instruction, Some(arg));
            program.push(instruction);
            is_reading_arg = false;
            continue;
        }

        if instruction_with_arg.contains(&byte) {
            is_reading_arg = true;
            current_arg_instruction = byte;
        } else {
            let instruction = InstructionSet::from_int(byte as u8, None);
            program.push(instruction);
        }
    }
    program
}