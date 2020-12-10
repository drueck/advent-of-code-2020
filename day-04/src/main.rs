// Advent of Code 2020: Day 4
//
// We have a file containing purported passport details, some of which are not
// valid. The following fields are possibilites, and all must be present except
// cid which is optional. Passports are separated in the file by blank lines.
// For part 1, as long as all the required fields are present, the passport is
// considered valid. The goal is to print out the number of valid passports.
//
// byr (Birth Year)
// iyr (Issue Year)
// eyr (Expiration Year)
// hgt (Height)
// hcl (Hair Color)
// ecl (Eye Color)
// pid (Passport ID)
// cid (Country ID)
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
    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .collect();

    let required_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut passports: Vec<HashMap<String, String>> = vec![];
    let mut passport: HashMap<String, String> = HashMap::new();

    for line in lines {
        if line.trim() == "" {
            passports.push(passport);
            passport = HashMap::new();
            continue;
        }

        let fields: Vec<&str> = line.split(" ").collect();
        for field in fields {
            let parts: Vec<&str> = field.split(":").collect();
            passport.insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    passports.push(passport);

    let total_passports = passports.len();

    let valid_passports = passports
        .iter()
        .filter(|&passport| required_keys.iter().all(|&key| passport.contains_key(key)))
        .count();

    println!(
        "Found {} valid passports out of {} total according to our rules",
        valid_passports, total_passports
    );
}
