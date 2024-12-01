use std::collections::HashMap;

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let lines: Vec<String> = lines_from_file("input/day01.txt");
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut r_count: HashMap<i32, i32> = HashMap::new();

    for l in lines {
        let left_right: Vec<&str> = l.split_whitespace().collect();
        if let (Ok(l), Ok(r)) = (left_right[0].parse::<i32>(), left_right[1].parse::<i32>()) {
            left.push(l);
            right.push(r);
            *r_count.entry(r).or_insert(0) += 1;
        }
    }
    left.sort();
    right.sort();

    let sol1 = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, (left, right)| acc + (right - left).abs());

    left.dedup();
    let sol2 = left
        .iter()
        .fold(0, |acc, i| acc + (i * r_count.get(i).unwrap_or(&0)));

    (Solution::from(sol1), Solution::from(sol2))
}
