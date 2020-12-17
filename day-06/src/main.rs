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
// Day 2 update: different rules. We only want the list of questions that all
// members of the group answered yes to. So for the test input example, instead
// of 11 we would get: 3 + 0 + 1 + 1 + 1 = 6.
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

    let groups_lines = split_into_groups(lines);
    let groups_common_questions = find_common_questions(groups_lines);

    let answer: usize = groups_common_questions
        .iter()
        .map(|common_questions| common_questions.len())
        .sum();

    println!(
        "The sum of counts of yes answers for each group is {}",
        answer
    );
}

fn split_into_groups(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut groups_lines: Vec<Vec<String>> = vec![];
    let mut group_lines: Vec<String> = vec![];

    for line in lines.iter() {
        if line.trim() == "" {
            group_lines.sort_unstable_by_key(|line| line.len());
            groups_lines.push(group_lines);
            group_lines = vec![];
        } else {
            group_lines.push(line.clone());
        }
    }
    group_lines.sort_unstable_by_key(|line| line.len());
    groups_lines.push(group_lines);

    return groups_lines;
}

fn find_common_questions(groups_lines: Vec<Vec<String>>) -> Vec<HashSet<char>> {
    return groups_lines
        .iter()
        .map(|group_lines| {
            let mut common_questions: HashSet<char> = group_lines[0].chars().collect();
            let num_lines = group_lines.len();
            for i in 1..num_lines {
                let line_questions: HashSet<char> = group_lines[i].chars().collect();
                common_questions.retain(|question| line_questions.contains(&question));
            }
            return common_questions;
        })
        .collect();
}
