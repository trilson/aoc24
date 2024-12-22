use std::{collections::HashMap, iter::successors};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let sequences: Vec<i64> = lines_from_file("input/day22.txt")
        .iter()
        .filter_map(|c| c.parse().ok())
        .collect();

    let mut sol1: i64 = 0;
    let mut optimal_bananas = HashMap::new();
    for sequence in sequences {
        let secrets: Vec<i64> = successors(Some(sequence), |cur| Some(next_sequence(*cur)))
            .take(2001)
            .collect();
        sol1 += secrets.last().expect("Must have a 2000th sequence");

        let bananas: Vec<i64> = secrets.iter().map(|x| x % 10).collect();
        let deltas: Vec<i64> = bananas.windows(2).map(|x| x[1] - x[0]).collect();

        let mut price_changes = HashMap::new();
        for (id, window) in deltas.windows(4).enumerate() {
            let window_vector = window.to_vec();
            if !price_changes.contains_key(&window_vector) {
                price_changes.insert(window_vector, bananas[id + 4]);
            }
        }
        for (key, val) in price_changes {
            *optimal_bananas.entry(key).or_insert(0_i64) += val;
        }
    }
    let sol2: i64 = *optimal_bananas
        .values()
        .max()
        .expect("Must have optimal number of bananas to sell");

    (Solution::from(sol1), Solution::from(sol2))
}

fn next_sequence(sequence: i64) -> i64 {
    let mut next = (sequence ^ (sequence << 6)) & 16777215;
    next = (next ^ (next >> 5)) & 16777215;
    (next ^ (next << 11)) & 16777215
}
