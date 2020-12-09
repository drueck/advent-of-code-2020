// Advent of Code 2020: Day 3
//
// Read in a file that contains a grid of . and # characters that represents a
// map where dots are empty spaces and hash marks are trees. This map has a
// fixed height, but is assumed to repeat forever to the right.
//
// The goal is to determine how many trees we would hit if we went down from
// the top left to the bottom of the map using a given slope. For part one at
// least the slope is right 3, down 1.
//
// Part 2 update:
//
// We want to check a set of slopes to see which one hits the fewest trees.
// For some reason we want to collect the number of trees we hit with each
// slope and multiply those together and output that answer. We suspect this
// is to make it easier to enter the answer on the website. ;)
//
// The slopes to check are:
//
// right 1, down 1
// right 3, down 1
// right 5, down 1
// right 7, down 1
// right 1, down 2
//
// Usage: cargo run <input-file>

use std::{env, fs::File, io::BufRead, io::BufReader};

mod slope;
use crate::slope::Slope;

mod map;
use crate::map::Map;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <input-file>");
        return;
    }

    let input_file = &args[1];

    let file = File::open(input_file).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<Vec<char>> = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .map(|s| s.chars().collect())
        .collect();

    let map = Map::new(&lines);

    let slopes = vec![
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];

    let tree_counts: Vec<usize> = slopes.iter().map(|slope| map.trees(slope)).collect();

    println!();
    println!("Here are the tree counts for each slope:");
    println!();

    for (i, count) in tree_counts.iter().enumerate() {
        println!("{}: {}", &slopes[i], count);
    }

    println!();

    let min_tree_count = tree_counts.iter().min().unwrap();
    let min_tree_count_index = tree_counts
        .iter()
        .position(|&tree_count| tree_count == *min_tree_count)
        .unwrap();
    let best_slope = &slopes[min_tree_count_index];

    println!(
        "The slope with the fewest trees is {} with {} trees",
        best_slope, min_tree_count
    );
    println!(
        "The product of the tree counts is {}",
        tree_counts.iter().product::<usize>()
    );
}
