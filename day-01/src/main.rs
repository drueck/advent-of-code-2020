// Advent of Code 2020: Day 1
//
// Read in a file with a list of integers one per line and
// find a combination of <group-size> numbers in the list
// that add up to 2020.
//
// Display the product of those numbers as the answer.
//
// The answers when using the provided test-input.txt
// for group sizes of two and three, respectively, are:
//
// 1721 * 299 = 514579
// 979 * 366 * 675 = 241861950
//
// usage `cargo run <input-file> <group-size>`
//
// <group-size> is optional and defaults to 2

mod combinations;

pub use crate::combinations::Combinations;
use std::{env, fs::File, io::BufRead, io::BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file: &String = &args[1];
    let group_size: usize = if args.len() > 2 {
        args[2]
            .parse::<usize>()
            .expect("the second argument was not an integer")
    } else {
        2
    };

    let file = File::open(input_file).expect("no such file");
    let buf = BufReader::new(file);
    let entries: Vec<u32> = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .map(|s| s.parse::<u32>().expect("entry was not an int"))
        .collect();

    let combinations = Combinations::new(entries, group_size);
    let mut iterations = 0;

    for combination in combinations {
        iterations += 1;
        if combination.iter().sum::<u32>() == 2020 {
            display_success_message(combination, iterations);
            return;
        }
    }

    display_failure_message(group_size, iterations);
}

fn display_success_message(combination: Vec<u32>, iterations: usize) {
    let numbers = combination
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>();

    let product = combination.iter().product::<u32>();
    println!();
    println!("{} = 2020", numbers.join(" + "));
    println!("{} = {}", numbers.join(" * "), product);
    println!();
    println!("Iterations: {}", iterations);
}

fn display_failure_message(group_size: usize, iterations: usize) {
    println!();
    println!(
        "Did not find any combinations of {} entries that totalled 2020",
        group_size
    );
    println!("Iterations: {}", iterations);
}
