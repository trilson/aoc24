use crate::{
    utils::{casting::i_u, files::lines_from_file},
    Solution, SolutionPair,
};
use std::collections::HashSet;
pub fn solve() -> SolutionPair {
    let lines: Vec<String> = lines_from_file("input/day04.txt");

    let sol1 = sol1(&lines);
    let sol2 = sol2(&lines);

    (Solution::from(sol1), Solution::from(sol2))
}

fn sol1(lines: &Vec<String>) -> i32 {
    lines
        .iter()
        .enumerate()
        .flat_map(|(idx, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter_map(move |(idy, &ch)| (ch == b'X').then(|| count_xmas(idx, idy, lines)))
        })
        .sum()
}

fn sol2(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .enumerate()
        .flat_map(|(idx, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter(move |(idy, &ch)| is_valid(&ch, idx, *idy, lines))
        })
        .count()
}

fn is_valid(ch: &u8, idx: usize, idy: usize, grid: &Vec<String>) -> bool {
    *ch == b'A'
        && ms_check(
            idx as i32 - 1,
            idy as i32 - 1,
            idx as i32 + 1,
            idy as i32 + 1,
            grid,
        )
        && ms_check(
            idx as i32 - 1,
            idy as i32 + 1,
            idx as i32 + 1,
            idy as i32 - 1,
            grid,
        )
}

fn ms_check(cx1: i32, cy1: i32, cx2: i32, cy2: i32, grid: &Vec<String>) -> bool {
    let mut c: HashSet<char> = HashSet::from(['M', 'S']);
    if cx1 >= 0 && cx1 < grid.len() as i32 && cy1 >= 0 && cy1 < grid.len() as i32 {
        c.remove(
            &grid
                .get(cx1 as usize)
                .and_then(|x| x.chars().nth(cy1 as usize))
                .unwrap(),
        );
    }
    if c.len() == 1 && cx2 >= 0 && cx2 < grid.len() as i32 && cy2 >= 0 && cy2 < grid.len() as i32 {
        c.remove(
            &grid
                .get(cx2 as usize)
                .and_then(|x| x.chars().nth(cy2 as usize))
                .unwrap(),
        );
    }
    c.is_empty()
}

fn count_xmas(idx: usize, idy: usize, grid: &Vec<String>) -> i32 {
    let xmas: Vec<char> = vec!['M', 'A', 'S'];
    let dir = vec![
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (1, 0),
        (-1, 1),
        (-1, -1),
        (-1, 0),
    ];

    let mut count = 0;
    for dir in dir {
        let (mut x_nxt, mut y_nxt) = (idx as i32 + dir.0, idy as i32 + dir.1);
        let mut xmas_idx = 0;
        while let Some(true) = i_u(x_nxt)
            .and_then(|l| grid.get(l))
            .and_then(|v| i_u(y_nxt).and_then(|y| v.chars().nth(y)))
            .and_then(|ch| xmas.get(xmas_idx).map(|&c| c == ch))
        {
            xmas_idx += 1;
            x_nxt += dir.0;
            y_nxt += dir.1;
        }
        if xmas_idx == 3 {
            count += 1;
        }
    }
    count
}
