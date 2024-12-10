use std::collections::{HashMap, HashSet};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
struct Point {
    row: i32,
    col: i32,
    val: i32,
}

impl Point {
    fn from(row: i32, col: i32, grid: &[String]) -> Option<Self> {
        if row >= 0_i32 && row < grid.len() as i32 && col >= 0_i32 && col < grid.len() as i32 {
            grid.get(row as usize)
                .and_then(|r| r.chars().nth(col as usize))
                .and_then(|ch| ch.to_digit(10))
                .and_then(|digit| {
                    Some(Self {
                        row: row as i32,
                        col: col as i32,
                        val: digit as i32,
                    })
                })
        } else {
            None
        }
    }
}

pub fn solve() -> SolutionPair {
    let lines: Vec<String> = lines_from_file("input/day10.txt");

    let mut sol1 = 0;
    let mut sol2 = 0;

    for (row, l) in lines.iter().enumerate() {
        for (col, ch) in l.chars().enumerate() {
            if ch == '0' {
                let nines = find_paths(row as i32, col as i32, &lines, &mut HashMap::new(), -1);
                sol1 += nines.iter().collect::<HashSet<_>>().len();
                sol2 += nines.len();
            }
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}

fn find_paths(
    row: i32,
    col: i32,
    grid: &[String],
    visited: &mut HashMap<(Point, i32), Vec<Point>>,
    previous: i32,
) -> Vec<Point> {
    if let Some(point) = Point::from(row, col, grid) {
        if let Some(nines) = visited.get(&(point, previous)) {
            return nines.to_vec();
        } else {
            if point.val - previous != 1 {
                return Vec::new();
            } else if point.val == 9 {
                return vec![point];
            } else {
                let mut this_nines: Vec<Point> = Vec::new();
                for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    this_nines.extend(find_paths(row + dr, col + dc, &grid, visited, point.val));
                }
                visited.insert((point, previous), this_nines.clone());
                return this_nines;
            }
        }
    } else {
        Vec::new()
    }
}
