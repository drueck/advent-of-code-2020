// Advent of Code 2020: Day 5
//
// The input file has a list of seat codes for a unique airplane seating scheme,
// where the characters in the code basically implement a binary search for the
// seat. The codes are 10 characters long, where the first 7 are either F or B
// for "Front" or "Back" and the last 3 are either R or L for "Right" or "Left".
// The plane has 128 rows and 8 seats per row. You dropped your ticket, so you
// don't know which one is yours, so you scan all the other tickets (it's a full
// plane) and figure out the seat id for each seat, and then by process of
// elimination determine which one is yours. The seat id is calculated by the
// formula seat_id = row_id * 8 + column_id.
//
// Usage `cargo run <input-file>`

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

    let mut seat_ids: Vec<usize> = seat_codes
        .iter()
        .map(|code| airplane::seat_id(code))
        .collect();

    seat_ids.sort();

    let max_seat_id = seat_ids[seat_ids.len() - 1];
    let seat_ids_range = seat_ids[0]..max_seat_id;
    let mut missing_seat_id = 0;

    for (i, seat_id) in seat_ids_range.into_iter().enumerate() {
        if seat_id != seat_ids[i] {
            missing_seat_id = seat_id;
            break;
        }
    }

    println!("The highest seat id is {}", max_seat_id);
    println!("The missing seat id is {}", missing_seat_id);
}
