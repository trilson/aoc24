use crate::{utils::files::lines_from_file, Solution, SolutionPair};
use std::collections::{HashMap, HashSet, VecDeque};

const DIR: &[(i32, i32)] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];

#[derive(PartialEq, Eq, Copy, Clone, Debug, Ord, PartialOrd, Hash)]
enum FenceDirection {
    Vertical = 0,
    Horizontal = 1,
}

#[derive(Debug, Clone, Copy)]
struct Fence {
    direction: FenceDirection,
    axis: i32,
    loc: i32,
    inside: bool,
}

impl Fence {
    fn from(before: (usize, usize), after: (i32, i32), dir: &(i32, i32)) -> Self {
        if dir.0 == 0 {
            return Fence {
                direction: FenceDirection::Vertical,
                axis: ((after.1 + before.1 as i32) as f32 / 2_f32).floor() as i32,
                loc: after.0,
                inside: after.1 > (before.1 as i32),
            };
        }
        Fence {
            direction: FenceDirection::Horizontal,
            axis: ((after.0 + before.0 as i32) as f32 / 2_f32).floor() as i32,
            loc: after.1,
            inside: after.0 > (before.0 as i32),
        }
    }
}

pub fn solve() -> SolutionPair {
    let grid: Vec<String> = lines_from_file("input/day12.txt");
    let (areas, fences) = get_fences(&grid);

    let mut sol1 = 0;
    let mut sol2 = 0;

    for (plot, area) in &areas {
        let fences: &Vec<Fence> = fences.get(&plot).expect("Must have fences for plot");
        let mut fence_map = HashMap::new();
        sol1 += fences.len() as u32 * area;

        let mut bounds = 0;
        for fence in fences {
            fence_map
                .entry((fence.direction, fence.axis, fence.inside))
                .or_insert(Vec::new())
                .push(fence.loc);
        }
        for mut border in fence_map {
            border.1.sort();
            bounds += contiguous(&border.1);
        }
        sol2 += bounds as u32 * area;
    }

    (Solution::from(sol1), Solution::from(sol2))
}

fn get_fences(
    lines: &Vec<String>,
) -> (
    HashMap<(usize, usize), u32>,
    HashMap<(usize, usize), Vec<Fence>>,
) {
    let grid_len = lines.len() as i32;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut areas: HashMap<(usize, usize), u32> = HashMap::new();
    let mut fences: HashMap<(usize, usize), Vec<Fence>> = HashMap::new();

    for (row, line) in lines.iter().enumerate() {
        for (col, start) in line.chars().enumerate() {
            let point = (row, col);
            if visited.contains(&point) {
                continue;
            }
            let mut queue: VecDeque<(usize, usize)> = vec![point].into();
            while let Some(current_point) = queue.pop_front() {
                if visited.contains(&current_point) {
                    continue;
                }
                visited.insert(current_point);
                *areas.entry(point).or_insert(0) += 1;

                for dir in DIR {
                    let row = dir.0 + current_point.0 as i32;
                    let col = dir.1 + current_point.1 as i32;
                    if row < 0
                        || row >= grid_len
                        || col < 0
                        || col >= grid_len
                        || !lines
                            .get(row as usize)
                            .and_then(|c| c.chars().nth(col as usize))
                            .is_some_and(|ch| ch == start)
                    {
                        fences
                            .entry(point)
                            .or_insert_with(Vec::new)
                            .push(Fence::from(current_point, (row, col), dir));
                    } else {
                        queue.push_back((row as usize, col as usize));
                    }
                }
            }
        }
    }
    (areas, fences)
}

fn contiguous(fences: &[i32]) -> i32 {
    if fences.is_empty() {
        return 0;
    }

    let mut blocks = 1;
    for window in fences.windows(2) {
        if window[1] != window[0] + 1 {
            blocks += 1;
        }
    }
    blocks
}
