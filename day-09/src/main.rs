// Advent of Code: Day 9
//
// We have an encrypted file that's a sequence of numbers. We're trying
// to break the encryption and in order to do so, we have to find the
// first number in the sequence (after the preamble) that is not the sum
// of two of the values in the previous preamble-length window. For the
// test input, the preamble length is 5, and we're trying to find the
// first number after the first 5 for which the number is not the sum
// of two of the numbers in the previous group of 5. For the test input
// the answer is 127.
//
// Part 2:
//
// We now want to find a group of consecutive number entries in the list
// which add up to our number that we found in part 1. Then we want to
// get the minimum and maximum numbers from that group and add them
// together for our answer. For the test input the answer is 62.
//
// Usage cargo run <input-file> <preamble-length>

use std::{env, fs::File, io::BufRead, io::BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: cargo run <input-file> <preamble-length>");
        return;
    }

    let input_file = &args[1];
    let preamble_length: usize = args[2]
        .parse()
        .expect("preamble length must be a positive integer");

    let file = File::open(input_file).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<usize> = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .map(|s| s.parse().expect("must be a positive integer"))
        .collect();

    let mut n: &usize = &0;

    for i in preamble_length..lines.len() {
        let window = &lines[(i - preamble_length)..i];
        n = &lines[i];

        if !sum_of_two_in_window(n, window) {
            println!("The number that doesn't follow the pattern is {}", n);
            break;
        }
    }

    let mut start: usize = 0;
    let mut end: usize = 0;

    for i in 0..lines.len() {
        let mut sum: usize = lines[i];
        for j in (i + 1)..lines.len() {
            sum += lines[j];
            if sum >= *n {
                start = i;
                end = j;
                break;
            }
        }
        if sum == *n {
            let range = &lines[start..=end];
            let min = range.iter().min().unwrap();
            let max = range.iter().max().unwrap();
            println!("The answer for part 2 is {}", min + max);
            break;
        }
    }
}

fn sum_of_two_in_window(n: &usize, window: &[usize]) -> bool {
    for i in 0..window.len() {
        let a = &window[i];
        for j in 0..window.len() {
            if j == i {
                continue;
            }
            let b = &window[j];
            if a + b == *n {
                return true;
            }
        }
    }
    return false;
}
