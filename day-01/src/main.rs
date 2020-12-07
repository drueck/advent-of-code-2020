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

use crate::combinations::Combinations;
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
    let mut entries: Vec<u32> = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .map(|s| s.parse::<u32>().expect("entry was not an int"))
        .collect::<Vec<u32>>();

    display_initial_entries_message(&entries, group_size);

    let initial_entries_count = entries.len();
    entries.sort();
    remove_impossible_entries(&mut entries, group_size);

    let combinations = Combinations::new(&entries, group_size);
    let mut iterations = 0;

    display_filtered_entries_mesage(initial_entries_count, &entries, &combinations);

    for combination in combinations {
        iterations += 1;
        if combination.iter().sum::<u32>() == 2020 {
            display_success_message(combination, iterations);
            return;
        }
    }

    display_failure_message(group_size, iterations);
}

fn display_initial_entries_message(entries: &Vec<u32>, group_size: usize) {
    let num_entries = entries.len();
    println!();
    println!("Found {} entries", num_entries);
    println!(
        "{} entries would have {} possible combinations of {}",
        num_entries,
        Combinations::new(&entries, group_size).len(),
        group_size
    );
}

fn remove_impossible_entries(entries: &mut Vec<u32>, group_size: usize) {
    let smallest_entries = entries[0..(group_size - 1)].to_vec();
    let sum_of_smallest = smallest_entries.iter().sum::<u32>();

    entries.retain(|&entry| entry + sum_of_smallest <= 2020)
}

fn display_filtered_entries_mesage(
    initial_entries_count: usize,
    entries: &Vec<u32>,
    combinations: &Combinations,
) {
    println!();
    println!(
        "Eliminated {} entries that could not be part of the solution",
        initial_entries_count - entries.len()
    );
    println!(
        "Now we have {} possible combinations to consider",
        combinations.len()
    );
}

fn display_success_message(combination: Vec<u32>, iterations: usize) {
    let numbers = combination
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>();

    let product = combination.iter().product::<u32>();
    println!();
    println!("Success!");
    println!("{} = 2020", numbers.join(" + "));
    println!("{} = {}", numbers.join(" * "), product);
    println!();
    println!("Iterations: {}", iterations);
}

fn display_failure_message(group_size: usize, iterations: usize) {
    println!();
    println!("Bummer!");
    println!(
        "Did not find any combinations of {} entries that totalled 2020",
        group_size
    );
    println!("Iterations: {}", iterations);
}
