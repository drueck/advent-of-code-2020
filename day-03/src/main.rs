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
// Usage: cargo run <input-file>

use std::{env, fs::File, io::BufRead, io::BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    let file = File::open(input_file).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<Vec<char>> = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .map(|s| s.chars().collect())
        .collect();

    let width = lines.iter().nth(0).unwrap().len();
    let height = lines.len();

    let right = 3;
    let down = 1;

    let mut x = 0;
    let mut y = 0;

    let mut tree_count = 0;

    while y < height {
        if lines[y][x] == '#' {
            tree_count += 1;
        }

        x = (x + right) % width;
        y = y + down;
    }

    println!("The number of trees encountered was {}", tree_count);
}
