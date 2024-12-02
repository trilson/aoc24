use crate::{utils::files::lines_from_file, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let lines: Vec<String> = lines_from_file("input/day02.txt");
    let mut sol1 = 0;
    let mut sol2 = 0;

    for line in lines {
        let report: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        if is_valid(&report) {
            sol1 += 1;
            continue;
        }
        for (idx, _) in report.iter().enumerate() {
            let excluded: Vec<i32> = report[..idx]
                .iter()
                .chain(&report[idx + 1..])
                .copied()
                .collect();
            if is_valid(&excluded) {
                sol2 += 1;
                break;
            }
        }
    }

    (Solution::from(sol1), Solution::from(sol1 + sol2))
}

pub fn is_valid(input: &Vec<i32>) -> bool {
    let mut prev = input[0];

    let increment = if input[1] > prev { 1 } else { -1 };
    let mut success = true;

    for element in input[1..].iter() {
        let delta = increment * (element - prev);
        if delta < 1 || delta > 3 {
            success = false;
            break;
        }
        prev = *element;
    }
    success
}
