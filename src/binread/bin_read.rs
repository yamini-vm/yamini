use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::instructions::InstructionSet;
use crate::memory::InnerData;

#[derive(Debug)]
enum State {
    ReadOneArg,
    ReadTwoArgs,
    GenInstruction,
    ReadInstruction,
    ReadObject,
}

fn read_file_line_by_line(filepath: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);
    let mut file_bytes = Vec::new();

    reader.read_to_end(&mut file_bytes)?;

    Ok(file_bytes)
}

fn in_range_or_promote(data_arg: &String, range: &[u8],
                       in_range_type: &str, out_range_type: &str) -> InnerData {
    let mut value_bytes = Vec::new();

    for ch in data_arg.chars() {
        value_bytes.push(ch as u8 - '0' as u8);
    }

    let res = value_bytes.len() == range.len() && value_bytes.iter().zip(range).all(|(a, b)| a <= b);

    if res {
        InnerData::from(&data_arg, in_range_type)
    } else {
        InnerData::from(&data_arg, out_range_type)
    }
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
        8, // JMP
        10, // JZ
        11, // JN
        16, // CALL
    ];

    let instruction_with_two_args = vec![
        0,  // LOAD
        9,  // POP
    ];

    let mut object_instructions_with_end = HashMap::new();
    object_instructions_with_end.insert(12, 13); // 12 = STARTSTR, 13 = ENDSTR

    let object_offsets = vec![2, 3]; // 2 = STACK_OFFSET, 3 = STACK_OFFSET_STR

    let mut program = Vec::new();
    let mut state = State::ReadInstruction;

    let mut instruction_stack: Vec<u8> = Vec::new();
    let mut arg_stack: Vec<InnerData> = Vec::new();

    let mut instruction: u8;
    let mut arg;

    let mut i = 0;
    while i < buffer.len() {
        match state {
            State::ReadOneArg => {
                arg = buffer[i] as i8;
                arg_stack.push(InnerData::INT(arg));

                state = State::GenInstruction;
                i += 1;
            },
            State::ReadTwoArgs => {
                arg = buffer[i] as i8;
                arg_stack.push(InnerData::INT(arg));

                i += 1;
                arg = buffer[i] as i8;
                arg_stack.push(InnerData::INT(arg));

                state = State::GenInstruction;
                i += 1;
            },
            State::GenInstruction => {
                instruction = match instruction_stack.pop() {
                    Some(instruction) => instruction as u8,
                    None => panic!("Instruction stack is empty"),
                };

                program.push(InstructionSet::from_int(
                    instruction as u8, 
                    arg_stack.pop(),
                    arg_stack.pop(),
                ));

                state = State::ReadInstruction;
            },
            State::ReadInstruction => {
                instruction = buffer[i] as u8;
                instruction_stack.push(instruction);

                if instruction_with_arg.contains(&instruction) {
                    state = State::ReadOneArg;
                } else if instruction_with_two_args.contains(&instruction) {
                    if object_offsets.contains(&buffer[i+1]) && object_instructions_with_end.contains_key(&buffer[i + 2]) {
                        state = State::ReadObject;
                    } else {
                        state = State::ReadTwoArgs;
                    }
                } 
                else {
                    state = State::GenInstruction;
                }

                i += 1;
            },
            State::ReadObject => {
                let mut j = i + 2;
                let mut data_arg = String::new();

                let offset = buffer[i] as i8;
                arg_stack.push(InnerData::INT(offset));
                i += 1;

                while buffer[j] != object_instructions_with_end[&buffer[i]] {
                    data_arg.push(buffer[j] as char);
                    j += 1;
                }
                j += 1; // Skip ENDSTR

                match offset {
                    2 => {
                        match data_arg.len() {
                            1 | 2 => {
                                arg_stack.push(InnerData::from(&data_arg, "INT"));
                            },
                            3 => {
                                let range = vec![1, 2, 8];
                                arg_stack.push(in_range_or_promote(&data_arg, &range, 
                                                                   "INT", "INT16"));
                            },
                            4 => {
                                arg_stack.push(InnerData::from(&data_arg, "INT16"));
                            },
                            5 => {
                                let range = vec![3, 2, 7, 6, 7];
                                arg_stack.push(in_range_or_promote(&data_arg, &range, 
                                                                   "INT16", "INT32"));
                            },
                            6 | 7 | 8 | 9 => {
                                arg_stack.push(InnerData::from(&data_arg, "INT32"));
                            }
                            10 => {
                                let range = vec![2, 1, 4, 7, 4, 8, 3, 6, 4, 7];
                                arg_stack.push(in_range_or_promote(&data_arg, &range, 
                                                                   "INT32", "ERR"));
                            }
                            _ => {

                            }
                        }
                    },
                    3 => {
                        arg_stack.push(InnerData::STR(data_arg));
                    },
                    _ => {
                        panic!("Unknown object offset");
                    }
                }

                i = j;
                state = State::GenInstruction;
            },
        }
    }

    // Read RET instruction
    program.push(InstructionSet::from_int(
        instruction_stack.pop().unwrap() as u8,
        None, 
        None
    ));

    program
}