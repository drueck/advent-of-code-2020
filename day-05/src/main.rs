use std::{env, fs::File, io::BufRead, io::BufReader};

mod airplane;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <input-file>");
        return;
    }

    let input_file = &args[1];

    let file = File::open(input_file).expect("no such file");
    let buf = BufReader::new(file);
    let seat_codes: Vec<String> = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .collect();

    let seat_ids: Vec<usize> = seat_codes
        .iter()
        .map(|code| airplane::seat_id(code))
        .collect();

    println!("The highest seat id is {}", seat_ids.iter().max().unwrap());
}
