use std::collections::{HashMap, HashSet};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

const X: i32 = 101;
const X_MID: f32 = X as f32 / 2_f32;
const Y: i32 = 103;
const Y_MID: f32 = Y as f32 / 2_f32;

const T_1: i32 = 100;
const T_2: i32 = X * Y;

struct Robot {
    start_x: i32,
    start_y: i32,
    velocity_x: i32,
    velocity_y: i32,
}

impl Robot {
    pub fn transport(&self, time: i32) -> (i32, i32) {
        (
            (self.start_x + (self.velocity_x * time)).rem_euclid(X),
            (self.start_y + (self.velocity_y * time)).rem_euclid(Y),
        )
    }

    pub fn from(input: &String) -> Self {
        let mut pos_vel = input.split_whitespace().flat_map(|seq| seq[2..].split(','));
        Robot {
            start_x: pos_vel.next().expect("P_X").parse().unwrap(),
            start_y: pos_vel.next().expect("P_Y").parse().unwrap(),
            velocity_x: pos_vel.next().expect("V_X").parse().unwrap(),
            velocity_y: pos_vel.next().expect("V_Y").parse().unwrap(),
        }
    }
}

pub fn solve() -> SolutionPair {
    let robots: Vec<Robot> = lines_from_file("input/day14.txt")
        .iter()
        .map(|line| Robot::from(line))
        .collect();

    (
        Solution::from(solve_pt1(&robots).expect("Pt 1")),
        Solution::from(solve_pt2(&robots).expect("Pt 2")),
    )
}

fn solve_pt1(robots: &Vec<Robot>) -> Option<i32> {
    let mut quadrants: HashMap<(bool, bool), i32> = HashMap::new();
    for robot in robots {
        let (x, y) = robot.transport(T_1);
        if x != X_MID as i32 && y != Y_MID as i32 {
            *quadrants
                .entry((x as f32 > X_MID, y as f32 > Y_MID))
                .or_insert(0) += 1;
        }
    }
    quadrants.values().copied().reduce(|acc, e| acc * e)
}

fn solve_pt2(robots: &Vec<Robot>) -> Option<i32> {
    for time in 1..T_2 {
        let mut distribution: HashSet<(i32, i32)> = HashSet::new();
        for robot in robots {
            distribution.insert(robot.transport(time));
        }
        if distribution.len() == robots.len() {
            return Some(time);
        }
    }
    None
}
