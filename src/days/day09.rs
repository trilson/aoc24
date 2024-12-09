use crate::{Solution, SolutionPair};
use std::{fs::read_to_string, iter::repeat};

pub fn solve() -> SolutionPair {
    let lines = read_to_string("input/day09.txt").unwrap();
    let sol1 = sol1(&lines);
    let sol2 = sol2(&lines);
    (Solution::from(sol1), Solution::from(sol2))
}

fn sol1(lines: &str) -> i64 {
    let mut exploded: Vec<i64> = Vec::new();
    for (idx, ch) in lines.chars().enumerate() {
        if idx % 2 == 0 {
            exploded.extend(repeat(idx as i64 / 2).take(ch.to_digit(10).unwrap() as usize));
        } else {
            exploded.extend(repeat(-1).take(ch.to_digit(10).unwrap() as usize));
        }
    }
    let (mut fwd, mut bwd) = (0_usize, exploded.len() - 1);
    while fwd < bwd {
        if exploded[fwd] == -1 {
            while exploded[bwd] == -1 && fwd < bwd {
                bwd -= 1;
            }
            exploded[fwd] = exploded[bwd];
            exploded[bwd] = -1
        }
        fwd += 1;
    }
    exploded
        .iter()
        .enumerate()
        .filter(|(_, &val)| val != -1)
        .map(|(idx, &val)| val * idx as i64)
        .sum()
}

fn sol2(lines: &str) -> i64 {
    let mut pos_types: Vec<(i64, i64)> = Vec::new();
    for (idx, ch) in lines.chars().enumerate() {
        let length = ch.to_digit(10).unwrap() as i64;
        if idx % 2 == 0 {
            pos_types.push((length, (idx / 2) as i64));
        } else {
            pos_types.push((length, -1));
        }
    }

    let mut fid = pos_types.len() - 1;
    while fid > 0 {
        let f_element = pos_types.get(fid).unwrap().clone();
        if f_element.1 == -1 {
            fid -= 1;
            continue;
        }
        let mut sid = 0;
        while sid < fid {
            let s_element = pos_types.get_mut(sid).unwrap();
            if s_element.1 == -1 && s_element.0 >= f_element.0 {
                s_element.0 -= f_element.0;
                pos_types[fid].1 = -1;
                pos_types.insert(sid, f_element);
                break;
            }
            sid += 1;
        }
        fid -= 1;
    }

    let mut sol2 = 0;
    let mut pos = 0;
    for val in pos_types {
        if val.1 != -1 {
            sol2 += val.1 * (pos..pos + val.0).sum::<i64>();
        }
        pos += val.0;
    }
    sol2
}
