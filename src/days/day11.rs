use cached::proc_macro::cached;

use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

pub fn solve() -> SolutionPair {
    let input: Vec<u64> = read_to_string("input/day11.txt")
        .expect("Input file unavailable")
        .split_whitespace()
        .flat_map(str::parse)
        .collect();
    let sol1: u64 = input.iter().map(|c| pebble_blink(*c, 25)).sum();
    let sol2: u64 = input.iter().map(|c| pebble_blink(*c, 75)).sum();
    (Solution::from(sol1), Solution::from(sol2))
}

#[cached]
fn pebble_blink(pebble: u64, blinks: u32) -> u64 {
    if blinks == 0 {
        return 1;
    }
    if pebble == 0 {
        return pebble_blink(1, blinks - 1);
    }
    let digits = pebble.ilog10() + 1;
    if pebble > 9 && digits % 2 == 0 {
        let pow_ten = 10_u64.pow(digits / 2);
        return pebble_blink(pebble / pow_ten, blinks - 1)
            + pebble_blink(pebble % pow_ten, blinks - 1);
    }
    pebble_blink(pebble * 2024, blinks - 1)
}
