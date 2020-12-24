// Advent of Code 2020: Day 8
//
// We have a set of instructions for a handheld game console that have an infinite
// loop in them. Our task is to find the instruction that is executed twice, thus
// causing the loop, and output the value of the accumulator just before that
// instruction is executed the second time.
//
// For the test input the value of the accumulator is 5.
//
// Part 2 update:
//
// There's a bug in the code that is causing the infinite loop, and we know what it's
// either a jmp that should be a nop or a nop that should be a jmp. Our task is to
// find and fix the bug and then see what the acc value is at the end of the proper
// execution of the program.
//
// Usage: cargo run <input-file>

use std::{env, fs::File, io::BufRead, io::BufReader};

mod vm;
use crate::vm::{execute, parse, ExitCode, Instruction, OpCode};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <input-file>");
        return;
    }

    let input_file = &args[1];

    let file = File::open(input_file).expect("no such file");
    let buf = BufReader::new(file);
    let original_instructions: Vec<Instruction> = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .map(|s| parse(&s))
        .collect();

    let (_, acc) = execute(&original_instructions);

    println!("The acc just before the infinite loop was {}", acc);

    let instructions_len = original_instructions.len();
    let indices_to_check: Vec<usize> = (0..instructions_len)
        .filter(|&i| {
            let instruction = &original_instructions[i];
            return (instruction.opcode == OpCode::Nop && instruction.arg != 0)
                || instruction.opcode == OpCode::Jmp;
        })
        .collect();

    let mut debug_instructions = original_instructions.clone();
    let mut result: (ExitCode, isize) = (ExitCode::InfiniteLoop, 0);

    for i in indices_to_check.iter() {
        let original_instruction = &original_instructions[*i];
        let test_instruction = Instruction {
            opcode: if original_instruction.opcode == OpCode::Nop {
                OpCode::Jmp
            } else {
                OpCode::Nop
            },
            arg: original_instruction.arg,
        };
        debug_instructions[*i] = test_instruction;

        result = execute(&debug_instructions);

        if result.0 == ExitCode::Ok {
            println!("The buggy instruction was found at index {}", i);
            break;
        }

        debug_instructions[*i] = original_instructions[*i].clone();
    }

    if result.0 == ExitCode::Ok {
        println!("The correct acc after fixing the bug was {}", result.1);
    } else {
        println!("Could not find the bug! :(");
    }
}
