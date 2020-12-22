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

use std::{collections::HashMap, env, fs::File, io::BufRead, io::BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <input-file>");
        return;
    }

    let input_file = &args[1];

    let file = File::open(input_file).expect("no such file");
    let buf = BufReader::new(file);
    let instructions: Vec<String> = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .collect();

    let mut ip: usize = 0;
    let mut acc: isize = 0;
    let mut visited_instructions: HashMap<usize, bool> = HashMap::new();
    let instructions_len: usize = instructions.len();

    while ip <= instructions_len && !visited_instructions.contains_key(&ip) {
        visited_instructions.insert(ip, true);
        let (opcode, arg) = parse(&instructions[ip]);

        match &opcode[..] {
            "nop" => {
                ip = ip + 1;
            }
            "acc" => {
                acc = acc + arg;
                ip = ip + 1;
            }
            "jmp" => {
                if arg < 0 {
                    ip = ip - arg.abs() as usize;
                } else {
                    ip = ip + arg as usize;
                }
            }
            _ => {
                panic!("Incompatible opcode found!");
            }
        }
    }

    println!("The value of the accumulator was {}", acc);
}

fn parse(instruction: &str) -> (String, isize) {
    let parts: Vec<&str> = instruction.split(" ").collect();
    let opcode: String = parts[0].to_string();
    let arg: isize = parts[1].parse().unwrap();

    return (opcode, arg);
}
