use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
    ops::Div,
};

use crate::{Solution, SolutionPair};
use nom::{
    character::complete::{char, i32 as parse_i32, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

static DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
static ROWS: i32 = 70;
static COLS: i32 = 70;

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day18.txt").unwrap();
    let (_, coords) = parse_input(&input).expect("Error parsing input");

    (
        Solution::from(solve_pt1(&coords)),
        Solution::from(solve_pt2(&coords)),
    )
}

fn solve_pt1(coords: &Vec<(i32, i32)>) -> i32 {
    min_path(coords, 1024).expect("Must have a solution")
}

fn solve_pt2(coords: &Vec<(i32, i32)>) -> String {
    let (mut low, mut hi) = (0, coords.len() - 1);
    while low <= hi {
        let mid = (low + hi).div(2);
        if min_path(coords, mid).is_some() {
            low = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    format!("{},{}", coords[hi].0, coords[hi].1)
}

fn min_path(fallen: &Vec<(i32, i32)>, fallen_count: usize) -> Option<i32> {
    let mut visited = fallen
        .iter()
        .take(fallen_count)
        .cloned()
        .collect::<HashSet<(i32, i32)>>();

    let mut q: VecDeque<(i32, i32, i32)> = VecDeque::from([(0, 0, 0)]);
    while let Some((col, row, dist)) = q.pop_front() {
        let path = (col, row);
        if visited.contains(&path) {
            continue;
        }
        visited.insert(path);
        if row < 0 || col < 0 || row > ROWS || col > COLS {
            continue;
        }
        if row == ROWS && col == COLS {
            return Some(dist);
        }
        q.extend(DIRS.map(|d| (col + d.0, row + d.1, 1 + dist)));
    }
    None
}

fn parse_line(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(parse_i32, char(','), parse_i32)(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    separated_list1(line_ending, parse_line)(input)
}
