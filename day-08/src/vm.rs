use std::collections::HashMap;

#[derive(PartialEq, Clone)]
pub enum OpCode {
    Nop,
    Jmp,
    Acc,
}

#[derive(Clone)]
pub struct Instruction {
    pub opcode: OpCode,
    pub arg: isize,
}

#[derive(PartialEq)]
pub enum ExitCode {
    Ok,
    InfiniteLoop,
}

pub fn execute(instructions: &Vec<Instruction>) -> (ExitCode, isize) {
    let mut ip: usize = 0;
    let mut acc: isize = 0;
    let mut visited_instructions: HashMap<usize, bool> = HashMap::new();
    let instructions_len: usize = instructions.len();

    while ip < instructions_len && !visited_instructions.contains_key(&ip) {
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

    let exit_code = if ip >= instructions_len {
        ExitCode::Ok
    } else {
        ExitCode::InfiniteLoop
    };

    return (exit_code, acc);
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
