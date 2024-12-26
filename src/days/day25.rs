use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day25.txt").expect("Invalid input");
    let mut blocks = input.split("\r\n\r\n");

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    while let Some(block) = blocks.next() {
        let block_lines: Vec<Vec<char>> =
            block.split("\r\n").map(|c| c.chars().collect()).collect();

        if block_lines[0][0] == '.' {
            locks.push(parse_block(block_lines, '.'));
        } else {
            keys.push(parse_block(block_lines, '#'));
        }
    }

    let sol1: u64 = locks
        .iter()
        .flat_map(|lock| {
            keys.iter()
                .filter(|key| key.iter().zip(lock.iter()).take(5).all(|(k, l)| k <= l))
        })
        .count() as u64;

    (Solution::from(sol1), Solution::from(0))
}

fn parse_block(block: Vec<Vec<char>>, b_type: char) -> Vec<usize> {
    (0..block[0].len())
        .filter_map(|i| block.iter().position(|row| row[i] != b_type))
        .collect()
}