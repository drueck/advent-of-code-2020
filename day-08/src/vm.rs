use std::collections::HashMap;

pub enum OpCode {
    Nop,
    Jmp,
    Acc,
}

pub struct Instruction {
    opcode: OpCode,
    arg: isize,
}

pub fn execute(instructions: Vec<Instruction>) -> isize {
    let mut ip: usize = 0;
    let mut acc: isize = 0;
    let mut visited_instructions: HashMap<usize, bool> = HashMap::new();
    let instructions_len: usize = instructions.len();

    while ip <= instructions_len && !visited_instructions.contains_key(&ip) {
        visited_instructions.insert(ip, true);
        let instruction = &instructions[ip];

        match instruction.opcode {
            OpCode::Nop => {
                ip = ip + 1;
            }
            OpCode::Acc => {
                acc = acc + instruction.arg;
                ip = ip + 1;
            }
            OpCode::Jmp => {
                if instruction.arg < 0 {
                    ip = ip - instruction.arg.abs() as usize;
                } else {
                    ip = ip + instruction.arg as usize;
                }
            }
        }
    }

    return acc;
}

pub fn parse(instruction: &str) -> Instruction {
    let parts: Vec<&str> = instruction.split(" ").collect();
    let opcode: OpCode = match parts[0] {
        "nop" => OpCode::Nop,
        "jmp" => OpCode::Jmp,
        "acc" => OpCode::Acc,
        _ => {
            panic!("Invalid opcode!");
        }
    };
    let arg: isize = parts[1].parse().unwrap();

    return Instruction { opcode, arg };
}
