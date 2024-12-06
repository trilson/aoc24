use std::collections::HashSet;

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let lines: Vec<String> = lines_from_file("input/day06.txt");
    let start = find_start(&lines).expect("We've got to have a start");
    let found = path(start, &lines, None).expect("Pt1 must have a valid path");
    let unique_path = found
        .iter()
        .map(|((x, y), _)| (x, y))
        .collect::<HashSet<_>>();

    let sol2 = unique_path
        .iter()
        .filter(|p| path(start, &lines, Some((*p.0, *p.1))).is_none())
        .count();

    (Solution::from(unique_path.len()), Solution::from(sol2))
}

fn find_start(grid: &Vec<String>) -> Option<(i32, i32)> {
    for (idx, l) in grid.iter().enumerate() {
        for (idy, v) in l.chars().enumerate() {
            if v == '^' {
                return Some((idx as i32, idy as i32));
            }
        }
    }
    None
}

fn path(
    start: (i32, i32),
    grid: &Vec<String>,
    obstacle: Option<(i32, i32)>,
) -> Option<HashSet<((i32, i32), usize)>> {
    let dir_order = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut pathfinder: HashSet<((i32, i32), usize)> = HashSet::new();
    let mut current_pos = start;
    let mut dir_idx: usize = 0;

    while current_pos.0 >= 0
        && current_pos.0 < grid.len() as i32
        && current_pos.1 >= 0
        && current_pos.1 < grid.len() as i32
    {
        if pathfinder.contains(&(current_pos, dir_idx)) {
            return None;
        }
        if grid
            .get(current_pos.0 as usize)
            .expect("Must be in bounds")
            .chars()
            .nth(current_pos.1 as usize)
            .expect("Must be in bounds")
            != '#'
            && current_pos != obstacle.unwrap_or_default()
        {
            pathfinder.insert((current_pos, dir_idx));
        } else {
            current_pos.0 -= dir_order[dir_idx].0;
            current_pos.1 -= dir_order[dir_idx].1;
            dir_idx = (dir_idx + 1) % 4
        }
        current_pos.0 += dir_order[dir_idx].0;
        current_pos.1 += dir_order[dir_idx].1;
    }
    Some(pathfinder)
}
