// Advent of Code 2020: Day 8
//
// We have a set of instructions for a handheld game console that have an infinite
// loop in them. Our task is to find the instruction that is executed twice, thus
// causing the loop, and output the value of the accumulator just before that
// instruction is executed the second time.
//
// For the test input the value of the accumulator is 5.
//
// Usage: cargo run <input-file>

use std::{env, fs::File, io::BufRead, io::BufReader};

mod vm;
use crate::vm::{execute, parse, Instruction};

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

    let acc = execute(original_instructions);

    println!("The acc just before the infinite loop was {}", acc);
}
