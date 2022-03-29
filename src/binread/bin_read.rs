use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::instructions::InstructionSet;
use crate::memory::InnerData;


fn read_file_line_by_line(filepath: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut file_lines = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => file_lines.push(line),
            Err(e) => println!("Error: {}", e),
        }
    }

    Ok(file_lines)
}

pub fn read_from_file(filepath: &str) -> Vec<InstructionSet> {
    let mut buffer = Vec::new();
    match read_file_line_by_line(filepath) {
        Ok(lines) => {
            buffer = lines;
        },
        Err(e) => println!("Error: {}", e),
    }

    let mut program = Vec::new();
    let mut is_reading_arg = false;
    let mut arg: InnerData;
    let mut int_byte: InnerData;

    for byte in buffer {
        if is_reading_arg {
            arg = byte.parse::<InnerData>().unwrap() as InnerData;
            let instruction = InstructionSet::from_int(0, Some(arg));
            program.push(instruction);
            is_reading_arg = false;
            continue;
        }

        int_byte = byte.parse::<InnerData>().unwrap() as InnerData;

        if int_byte == 0 {
            is_reading_arg = true;
        } else {
            let instruction = InstructionSet::from_int(int_byte as u8, None);
            program.push(instruction);
        }
    }
    program
}