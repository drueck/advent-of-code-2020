// Advent of Code 2020: Day 6
//
// This one is about counting the answers to which groups of passengers on a
// flight said yes on a customs declaration form. The input file lists answers
// that each person said yes to, and each group of passengers is separated by
// a blank line. The requested final output is the sum of counts for each
// group of questions where at least one of the passengers in the group answered
// yes. So, for the example in test-input.txt the answer should be
// 3 + 3 + 3 + 1 + 1 = 11.
//
// Usage: cargo run <input-file>

use std::{collections::HashSet, env, fs::File, io::BufRead, io::BufReader};

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

    let mut questions_answered_yes: HashSet<char> = HashSet::new();
    let mut groups_yes_answers: Vec<HashSet<char>> = vec![];

    for line in lines {
        if line.trim() == "" {
            groups_yes_answers.push(questions_answered_yes);
            questions_answered_yes = HashSet::new();
        } else {
            for question_letter in line.chars() {
                questions_answered_yes.insert(question_letter);
            }
        }
    }
    groups_yes_answers.push(questions_answered_yes);

    let answer: usize = groups_yes_answers.iter().map(|group| group.len()).sum();

    println!(
        "The sum of counts of yes answers for each group is {}",
        answer
    );
}
