use std::collections::HashMap;

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let input = lines_from_file("input/day19.txt");
    let available_towels: Vec<Vec<char>> =
        input[0].split(", ").map(|c| c.chars().collect()).collect();
    let target_towels: Vec<Vec<char>> = input[2..].iter().map(|c| c.chars().collect()).collect();

    let mut sol1: u64 = 0;
    let mut sol2: u64 = 0;
    let mut memo: HashMap<Vec<char>, u64> = HashMap::new();
    for towel in target_towels {
        let combs = combs(towel, &available_towels, &mut memo);
        sol2 += combs;
        if combs > 0 {
            sol1 += 1;
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}

fn combs(rem: Vec<char>, avail: &[Vec<char>], memo: &mut HashMap<Vec<char>, u64>) -> u64 {
    if rem.is_empty() {
        return 1;
    }
    if let Some(&v) = memo.get(&rem) {
        return v;
    }
    let sum = avail
        .iter()
        .filter_map(|t| {
            rem.starts_with(t)
                .then(|| combs(rem[t.len()..].to_vec(), avail, memo))
        })
        .sum();
    *memo.entry(rem).or_insert(sum)
}
