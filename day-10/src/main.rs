// Advent of Code: Day 10
//
// We have a complicated situation with various power adapters on a
// plane, but essentially we have a list of numbers and we want to
// sort the numbers and then count up the number of distinct differences
// between the numbers. For example, how many numbers when sorted have
// a difference of 1 from each other, how many have a difference of 3.
// The answer is the count of diffs of 1 multiplied by the count of diffs
// of 3.
//
// For the small input the answer is 7 * 5 = 35, for the test input the
// answer is 22 * 10 = 220.
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
    let mut lines: Vec<usize> = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .map(|s| s.parse().expect("must be a positive integer"))
        .collect();

    lines.sort();

    let mut differences: HashMap<usize, usize> = HashMap::new();

    for i in 0..(lines.len() - 1) {
        let diff = lines[i + 1] - lines[i];
        *differences.entry(diff).or_insert(0) += 1;
    }

    // adding one for the outlet to the first adapter
    let diffs_of_1 = differences.get(&1).unwrap() + 1;
    // adding one from the last adapter to the device
    let diffs_of_3 = differences.get(&3).unwrap() + 1;

    println!("Differences of 1: {}", diffs_of_1);
    println!("Differences of 3: {}", diffs_of_3);
    println!("The product of those is {}", diffs_of_1 * diffs_of_3);
}
