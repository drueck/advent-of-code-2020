// Advent of Code 2020: Day 7
//
// We have a list of rules for bags at the airport, specifically a list
// of bag types that must contain a specific number of other bag types.
// Our challenge for part 1 is to find the number of bags that could
// contain a shiny gold bag. Some bags will contain it directly, and other
// bags will contain a bag that contains a shiny gold bag. For our test
// input the answer is 4, because a shiny gold bag could be contained
// directly or nested in a bright white, muted yellow, dark orange, or
// light red bag.
//
// Part 2:
//
// For part two we are curious how many bags would have to be inside our
// shiny gold bag. Sooooo many.
//
// Usage cargo run <input-file>

use std::{env, fs::File, io::BufRead, io::BufReader};

mod bags;
use crate::bags::{Bag, BagRules};

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

    let mut bag_rules = BagRules::new();

    for line in lines {
        bag_rules.add_rule(&line);
    }

    let shiny_gold_bag = Bag::new("shiny gold");
    let bags_containing_shiny_gold = bag_rules.bags_that_could_contain(&shiny_gold_bag).len();
    let bags_inside_shiny_gold = bag_rules.num_bags_inside(&shiny_gold_bag);

    println!(
        "The number of bags that could contain a shiny gold bag is: {}",
        bags_containing_shiny_gold
    );

    println!(
        "The number of bags inside a shiny gold bag is: {}",
        bags_inside_shiny_gold
    );
}
