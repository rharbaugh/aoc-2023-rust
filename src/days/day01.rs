use std::fs::read_to_string;

use crate::{Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let file_path = "input/1";
    println!("In file {file_path}");

    let mut lines = read_to_string(file_path).expect("Should have been able to read the file.");

    //this is stupid brute forcing but otherwise we're doing regex or something horrible
    lines = str::replace(&lines, "one", "one1one");
    lines = str::replace(&lines, "two", "two2two");
    lines = str::replace(&lines, "three", "three3three");
    lines = str::replace(&lines, "four", "four4four");
    lines = str::replace(&lines, "five", "five5five");
    lines = str::replace(&lines, "six", "six6six");
    lines = str::replace(&lines, "seven", "seven7seven");
    lines = str::replace(&lines, "eight", "eight8eight");
    lines = str::replace(&lines, "nine", "nine9nine");

    let mut total = 0;
    for line in lines.split('\n') {
        println!("{}", line);
        let mut digits = line.chars().filter_map(|x| x.to_digit(10));

        let left = digits.next().unwrap_or(0);
        let right = digits.next_back().unwrap_or(left);

        let combination = (left * 10) + right;

        println!(
            "Left {}, Right {}, Combination {}",
            left, right, combination
        );

        //brane: if we 10x the left one we can add them
        total += combination;
    }

    let sol1: u32 = total;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
