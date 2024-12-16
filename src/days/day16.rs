use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Trajectory {
    loc: (i32, i32),
    direction: (i32, i32),
}

impl Trajectory {
    fn from(row: i32, col: i32, dir_row: i32, dir_col: i32) -> Self {
        Trajectory {
            loc: (row, col),
            direction: (dir_row, dir_col),
        }
    }
    fn with(&self, dir: &(i32, i32)) -> Self {
        Trajectory {
            loc: self.loc,
            direction: *dir,
        }
    }
    fn step(&self) -> Self {
        Trajectory {
            loc: (self.loc.0 + self.direction.0, self.loc.1 + self.direction.1),
            direction: self.direction,
        }
    }
}

#[derive(Eq, PartialEq)]
struct Path {
    cost: i32,
    traj: Trajectory,
    journey: Vec<Trajectory>,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Path {
    fn from(
        cost: i32,
        row: i32,
        col: i32,
        dir_row: i32,
        dir_col: i32,
        journey: Vec<Trajectory>,
    ) -> Self {
        Path {
            cost,
            traj: Trajectory::from(row, col, dir_row, dir_col),
            journey,
        }
    }

    fn from_dir(&self, dir: &(i32, i32)) -> Self {
        let cost = if self.traj.direction == *dir {
            self.cost + 1
        } else {
            self.cost + 1000
        };
        let trajectory = if self.traj.direction == *dir {
            self.traj.step()
        } else {
            self.traj.with(dir)
        };
        Path {
            cost,
            traj: trajectory,
            journey: self.journey.to_vec(),
        }
    }
}

static DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub fn solve() -> SolutionPair {
    let grid: Vec<Vec<char>> = lines_from_file("input/day16.txt")
        .iter()
        .map(|c| c.chars().collect())
        .collect();

    let (start_row, start_col) = grid
        .iter()
        .enumerate()
        .find_map(|(row, line)| {
            line.iter()
                .enumerate()
                .find(|&(_, ch)| ch == &'S')
                .map(|(col, _)| (row, col))
        })
        .expect("Must have a start");

    let mut hist: BinaryHeap<Path> = BinaryHeap::new();
    hist.push(Path::from(
        0,
        start_row as i32,
        start_col as i32,
        0,
        1,
        Vec::new(),
    ));

    let mut visited: HashSet<Trajectory> = HashSet::new();
    let mut best: Option<i32> = None;
    let mut journey: HashSet<(i32, i32)> = HashSet::new();

    while let Some(path) = hist.pop() {
        visited.insert(path.traj);
        if grid[path.traj.loc.0 as usize][path.traj.loc.1 as usize] == 'E' {
            if let Some(best) = best {
                if path.cost > best {
                    break;
                }
                journey.extend(path.journey.iter().map(|c| c.loc));
            } else {
                best = Some(path.cost);
            }
        }
        for mut next in DIRS.iter().map(|c| path.from_dir(c)) {
            if grid[next.traj.loc.0 as usize][next.traj.loc.1 as usize] == '#' {
                continue;
            }
            if visited.contains(&next.traj) {
                continue;
            }
            next.journey.push(path.traj);
            hist.push(next);
        }
    }

    (
        Solution::from(best.unwrap()),
        Solution::from(journey.len() + 1),
    )
}
