use std::{env, fs::File, io::BufRead, io::BufReader};

// read in a file with a list of integers one per line
// find two numbers in the list that add up to 2020
// display the product of those two numbers as the answer
//
// the answer when using the provided test-input.txt
// should be 1721 * 299 = 514579
//
// usage `cargo run <input-file>`

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let file = File::open(input).expect("no such file");
    let buf = BufReader::new(file);
    let entries: Vec<i64> = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .map(|s| s.parse::<i64>().expect("entry was not an int"))
        .collect();

    for first in entries.iter() {
        for second in entries.iter() {
            if first != second && first + second == 2020 {
                println!("The answer is: {}", first * second);
                return;
            }
        }
    }

    println!("Could not find an answer in the given input");
}
