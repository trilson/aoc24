use std::collections::VecDeque;

use indexmap::IndexSet;

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let track: Vec<Vec<char>> = lines_from_file("input/day20.txt")
        .iter()
        .map(|l| l.chars().collect())
        .collect();

    let s = find('S', &track).expect("Track must have a start");
    let path = find_path(s.0, s.1, &track, track.len() as i32, track[0].len() as i32)
        .expect("Must have a vald path from S to E");
    let sol1 = run_solver(&path, track.len() as i32, track[0].len() as i32, |p| {
        generate_skip(*p)
    });
    let sol2 = run_solver(&path, track.len() as i32, track[0].len() as i32, |p| {
        generate_manhattan(*p)
    });
    (Solution::from(sol1), Solution::from(sol2))
}

fn generate_skip(p: (i32, i32)) -> Vec<(i32, i32)> {
    [
        (p.0 + 2, p.1),
        (p.0 - 2, p.1),
        (p.0, p.1 + 2),
        (p.0, p.1 - 2),
    ]
    .into_iter()
    .collect()
}

fn generate_manhattan(p: (i32, i32)) -> Vec<(i32, i32)> {
    let mut cheats = Vec::new();
    for xr in -20..21 {
        let nr = p.0 + xr;
        for xc in -20 + xr.abs()..21 - xr.abs() {
            let nc = p.1 + xc;
            cheats.push((nr, nc));
        }
    }
    cheats
}

fn run_solver<F>(path: &IndexSet<(i32, i32)>, rows: i32, cols: i32, neighbour_generator: F) -> i64
where
    F: Fn(&(i32, i32)) -> Vec<(i32, i32)>,
{
    path.iter()
        .enumerate()
        .map(|(cur_step, &current_point)| {
            neighbour_generator(&current_point)
                .iter()
                .filter(|&&(r, c)| r >= 0 && r < rows && c >= 0 && c < cols)
                .filter_map(|cheat| path.get_full(cheat))
                .filter(|&(cheat_idx, &cheat_point)| {
                    let cost = (cheat_point.0 - current_point.0).abs()
                        + (cheat_point.1 - current_point.1).abs()
                        - 1;
                    cheat_idx > cur_step && (cheat_idx - cur_step) as i32 - cost > 100
                })
                .count() as i64
        })
        .sum()
}

fn find(target: char, track: &Vec<Vec<char>>) -> Option<(i32, i32)> {
    for (row, line) in track.iter().enumerate() {
        for (col, ch) in line.iter().enumerate() {
            if ch == &target {
                return Some((row as i32, col as i32));
            }
        }
    }
    None
}

fn find_path(
    row: i32,
    col: i32,
    track: &Vec<Vec<char>>,
    rows: i32,
    cols: i32,
) -> Option<IndexSet<(i32, i32)>> {
    let mut q = VecDeque::from([(row, col)]);
    let mut seen = IndexSet::new();

    while let Some((row, col)) = q.pop_front() {
        seen.insert((row, col));
        if track[row as usize][col as usize] == 'E' {
            return Some(seen);
        }
        for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (next_row, next_col) = (dr + row, dc + col);
            if next_row < 0 || next_col < 0 || next_row >= rows || next_col >= cols {
                continue;
            }
            if track[next_row as usize][next_col as usize] == '#' {
                continue;
            }
            if seen.contains(&(next_row, next_col)) {
                continue;
            }
            q.push_back((next_row, next_col))
        }
    }
    None
}
