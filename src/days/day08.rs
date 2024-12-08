use crate::{utils::files::lines_from_file, Solution, SolutionPair};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve() -> SolutionPair {
    let lines: Vec<String> = lines_from_file("input/day08.txt");
    let mut sol1_locations: HashSet<(i32, i32)> = HashSet::new();
    let mut sol2_locations: HashSet<(i32, i32)> = HashSet::new();

    let mut antenna_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch != '.' {
                antenna_map
                    .entry(ch)
                    .or_insert(Vec::new())
                    .push((row as i32, col as i32));
            }
        }
    }

    for group in antenna_map.values() {
        for combination in group.iter().combinations(2) {
            let before = combination.get(0).expect("Must have one half of pair");
            let after = combination.get(1).expect("Must have second half of pair");

            sol2_locations.insert(**before);
            sol2_locations.insert(**after);

            let dr = after.0 - before.0;
            let dc = after.1 - before.1;

            let mut antinode_before = (before.0 - dr, before.1 - dc);
            let mut antinode_after = (after.0 + dr, after.1 + dc);

            if in_bounds(antinode_before, lines.len()) {
                sol1_locations.insert(antinode_before);
            }
            if in_bounds(antinode_after, lines.len()) {
                sol1_locations.insert(antinode_after);
            }
            while in_bounds(antinode_before, lines.len()) {
                sol2_locations.insert(antinode_before);
                antinode_before = (antinode_before.0 - dr, antinode_before.1 - dc);
            }
            while in_bounds(antinode_after, lines.len()) {
                sol2_locations.insert(antinode_after);
                antinode_after = (antinode_after.0 + dr, antinode_after.1 + dc);
            }
        }
    }
    (
        Solution::from(sol1_locations.len()),
        Solution::from(sol2_locations.len()),
    )
}

fn in_bounds(node: (i32, i32), grid_len: usize) -> bool {
    node.0 >= 0 && node.0 < grid_len as i32 && node.1 >= 0 && node.1 < grid_len as i32
}
