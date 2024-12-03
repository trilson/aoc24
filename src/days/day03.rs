use regex::Regex;
use std::fs::read_to_string;

use crate::{Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("input/day03.txt").unwrap();
    let mul_binding: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_binding: Regex = Regex::new(r"(do)\(\)").unwrap();
    let dont_binding: Regex = Regex::new(r"(don't)\(\)").unwrap();

    let dos: Vec<usize> = do_binding.find_iter(&input).map(|s| s.start()).collect();
    let donts: Vec<usize> = dont_binding.find_iter(&input).map(|s| s.start()).collect();

    let mut sol1 = 0;
    let mut sol2 = 0;

    let mut inc_idx = 0_usize;
    let mut exc_idx = 0_usize;

    let mut do_val = 0;
    let mut dont_val = 0;

    for m in mul_binding.captures_iter(&input) {
        let a: i32 = m.get(1).unwrap().as_str().parse().unwrap();
        let b: i32 = m.get(2).unwrap().as_str().parse().unwrap();
        sol1 += a * b;

        let idx = m.get(1).unwrap().start();
        while inc_idx < dos.len() && dos[inc_idx] < idx {
            do_val = dos[inc_idx];
            inc_idx += 1;
        }

        while exc_idx < donts.len() && donts[exc_idx] < idx {
            dont_val = donts[exc_idx];
            exc_idx += 1;
        }

        if do_val >= dont_val {
            sol2 += a * b;
        }
    }
    (Solution::from(sol1), Solution::from(sol2))
}
