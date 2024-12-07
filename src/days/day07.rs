use crate::{utils::files::lines_from_file, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let lines: Vec<String> = lines_from_file("input/day07.txt");
    let mut sol1: i64 = 0;
    let mut sol2: i64 = 0;

    for line in lines.iter().map(|l| l.split_once(':')) {
        let (target_str, numbers_str) = line.expect("Must have a target and components");
        let target: i64 = target_str.parse().expect("Target must be a number");
        let numbers: Vec<i64> = numbers_str
            .split_whitespace()
            .map(|el| el.parse().expect("Component must be a number"))
            .collect();

        if is_valid(numbers.get(0).unwrap(), &target, &numbers[1..], false) {
            sol1 += target;
            sol2 += target;
        } else if is_valid(numbers.get(0).unwrap(), &target, &numbers[1..], true) {
            sol2 += target;
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}

fn is_valid(cur: &i64, goal: &i64, rem: &[i64], sol2: bool) -> bool {
    if cur > goal {
        return false;
    }
    if rem.is_empty() {
        return cur == goal;
    }
    let concat = cur * 10_i64.pow(((rem[0] as f64).log10().floor() as u32) + 1) + rem[0];
    return is_valid(&(cur * rem[0]), goal, &rem[1..], sol2)
        || is_valid(&(cur + rem[0]), goal, &rem[1..], sol2)
        || (sol2 && is_valid(&concat, goal, &rem[1..], sol2));
}
