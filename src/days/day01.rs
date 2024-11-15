use std::fs::read_to_string;

use crate::{Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let file_path = "input/1".to_string();

    let contents = read_to_string(file_path).expect("Should have been able to read the file");

    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
