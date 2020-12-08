// Advent of Code 2020: Day 2
//
// Read in a file with a list of passwords and their associated bizarre
// requirements. So many things wrong with this. Output the number of
// valid passwords from the list.
//
// The requirements and passwords look like this:
// 1-3 a: abcde
//
// which means that the password must have 1 to 3 a characters in it
// and then there's the password that was created supposedly with these
// requirements. In this case the password is valid
//
// Usage: cargo run <input-file>

use std::{env, fs::File, io::BufRead, io::BufReader};

mod password;
use crate::password::PasswordEntry;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    let file = File::open(input_file).expect("no such file");
    let buf = BufReader::new(file);
    let entries: Vec<PasswordEntry> = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .map(|s| PasswordEntry::from_string(s))
        .collect();

    let valid_entries = entries.iter().filter(|&entry| entry.valid()).count();

    println!("The numer of valid entries was: {}", valid_entries);
}
