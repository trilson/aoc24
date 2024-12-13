use crate::{utils::files::lines_from_file, Solution, SolutionPair};

#[derive(Debug)]
struct GameInput {
    a: (i64, i64),
    b: (i64, i64),
    p: (i64, i64),
}

impl GameInput {
    fn extract_leap(input: &String) -> (i64, i64) {
        let mut digits = input
            .split(|c: char| !c.is_ascii_digit())
            .filter_map(|s| s.parse::<i64>().ok());
        (
            digits.next().expect("Must have X"),
            digits.next().expect("Must have Y"),
        )
    }

    pub fn from(lines: Vec<String>) -> Option<Self> {
        let mut it = lines.iter();
        Some(GameInput {
            a: Self::extract_leap(it.next().expect("Must have A")),
            b: Self::extract_leap(it.next().expect("Must have B")),
            p: Self::extract_leap(it.next().expect("Must have Prize")),
        })
    }
}

pub fn solve() -> SolutionPair {
    let costs: Vec<_> = lines_from_file("input/day13.txt")
        .chunks(4)
        .filter_map(|ch| GameInput::from(ch.iter().take(3).cloned().collect()))
        .map(|g| calculate_costs(g))
        .collect();

    (
        Solution::from(costs.iter().filter_map(|c| c.0).sum::<i64>()),
        Solution::from(costs.iter().filter_map(|c| c.1).sum::<i64>()),
    )
}

fn calculate_costs(g: GameInput) -> (Option<i64>, Option<i64>) {
    let sol2 = 10000000000000;
    (
        calculate_from(g.p.0, g.p.1, g.a.0, g.a.1, g.b.0, g.b.1),
        calculate_from(g.p.0 + sol2, g.p.1 + sol2, g.a.0, g.a.1, g.b.0, g.b.1),
    )
}

fn calculate_from(t_x: i64, t_y: i64, a_x: i64, a_y: i64, b_x: i64, b_y: i64) -> Option<i64> {
    let numerator = (b_y * t_x) - (b_x * t_y);
    let denominator = (b_x * a_y) - (b_y * a_x);
    if numerator % denominator != 0 {
        return None;
    }
    let a = -1 * (numerator / denominator);
    let numerator_2 = t_x - (a_x * a);
    if numerator_2 % b_x != 0 {
        return None;
    }
    Some((a * 3) + numerator_2 / b_x)
}
