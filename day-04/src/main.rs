// Advent of Code 2020: Day 4
//
// We have a file containing purported passport details, some of which are not
// valid. The following fields are possibilites, and all must be present except
// cid which is optional. Passports are separated in the file by blank lines.
// For part 1, as long as all the required fields are present, the passport is
// considered valid. The goal is to print out the number of valid passports.
//
// Part 2 Update: Validating field values
//
// Each field must validate according to these rules:
//
// byr (Birth Year) - four digits; at least 1920 and at most 2020
// iyr (Issue Year) - four digits; at least 2010 and at most 2020
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030
// hgt (Height) - a number followed by either cm or in
//  - if cm, the number must be at least 150 and at most 293
//  - if in, the number must be at least 59 and at most 76
// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f
// ecl (Eye Color) - exactly one of amb blu brn gry grn hzl oth
// pid (Passport ID) - a nine-digit number, including leading zeros
// cid (Country ID) - optional, so no validation
//
// Usage: cargo run <input-file>

use std::{collections::HashMap, env, fs::File, io::BufRead, io::BufReader};

mod passport;
use crate::passport::Passport;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <input-file>");
        return;
    }

    let input_file = &args[1];

    let file = File::open(input_file).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .collect();

    let mut passports: Vec<Passport> = vec![];
    let mut fields: HashMap<String, String> = HashMap::new();

    for line in lines {
        if line.trim() == "" {
            passports.push(Passport::new(&fields));
            fields = HashMap::new();
            continue;
        }

        let entries: Vec<&str> = line.split(" ").collect();
        for entry in entries {
            let parts: Vec<&str> = entry.split(":").collect();
            fields.insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    passports.push(Passport::new(&fields));

    let total_passports = passports.len();

    let valid_passports = passports
        .iter()
        .filter(|&passport| passport.valid())
        .count();

    println!(
        "Found {} valid passports out of {} total according to our rules",
        valid_passports, total_passports
    );
}
