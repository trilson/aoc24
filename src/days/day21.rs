use bimap::BiMap;
use cached::proc_macro::cached;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::zip,
};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let codes = lines_from_file("input/day21.txt");
    let mut sol1 = 0;
    let mut sol2 = 0;

    for code in codes {
        let num: i64 = code[0..3].parse().expect("Integer value not found");
        sol1 += num * solve_pt1(code.clone(), "AAA".to_string()).expect("Must have a p1 solution");
        sol2 += num * solve_pt2(code.clone(), 25).expect("Must have a p2 solution");
    }

    (Solution::from(sol1), Solution::from(sol2))
}

static DIR_PAD: Lazy<BiMap<char, (i64, i64)>> = Lazy::new(|| {
    let mut map = BiMap::new();
    map.insert('^', (0, 1));
    map.insert('A', (0, 2));
    map.insert('<', (1, 0));
    map.insert('v', (1, 1));
    map.insert('>', (1, 2));
    map
});

static NUM_PAD: Lazy<BiMap<char, (i64, i64)>> = Lazy::new(|| {
    let mut map = BiMap::new();
    map.insert('7', (0, 0));
    map.insert('8', (0, 1));
    map.insert('9', (0, 2));
    map.insert('4', (1, 0));
    map.insert('5', (1, 1));
    map.insert('6', (1, 2));
    map.insert('1', (2, 0));
    map.insert('2', (2, 1));
    map.insert('3', (2, 2));
    map.insert('0', (3, 1));
    map.insert('A', (3, 2));
    map
});

fn solve_pt1(code: String, initial_state: String) -> Option<i64> {
    let mut queue: VecDeque<(i64, Vec<char>, Vec<char>)> =
        VecDeque::from([(0, code.chars().collect(), initial_state.chars().collect())]);

    let mut visited = HashSet::new();

    while let Some((presses, code_left, state)) = queue.pop_front() {
        if code_left.is_empty() {
            return Some(presses);
        }
        let visiting = (code_left.clone(), state.clone());
        if visited.contains(&visiting) {
            continue;
        }
        visited.insert(visiting);

        for dir in ['^', '<', '>', 'v'] {
            if let Some(moved) = find_next(dir, state[0], false) {
                let mut moved_state = state.clone();
                moved_state[0] = moved;
                queue.push_back((1 + presses, code_left.clone(), moved_state));
            }
        }

        let (a_state, a_code) = process_a_press(state, code_left);
        queue.push_back((1 + presses, a_code, a_state));
    }
    None
}

fn process_a_press(mut state: Vec<char>, mut code: Vec<char>) -> (Vec<char>, Vec<char>) {
    let (index, _ch) = state
        .iter()
        .enumerate()
        .find(|(_, &ch)| ch != 'A')
        .or_else(|| Some((state.len() - 1, &'A')))
        .expect("Must have a target");

    if index == state.len() - 1 {
        let num_to_pop = state[index];
        if num_to_pop == code[0] {
            code.remove(0);
        }
    } else if index == state.len() - 2 {
        if let Some(next_number) = find_next(state[index], state[index + 1], true) {
            state.pop();
            state.push(next_number);
        }
    } else {
        if let Some(next_dir) = find_next(state[index], state[index + 1], false) {
            state[index + 1] = next_dir;
        }
    }
    (state, code)
}

fn find_next(direction: char, current: char, num_pad: bool) -> Option<char> {
    let pad = if num_pad { &NUM_PAD } else { &DIR_PAD };
    let current_pos: (i64, i64) = *pad
        .get_by_left(&current)
        .expect("Current location must exist");
    let new_location = match direction {
        '^' => (current_pos.0 - 1, current_pos.1),
        '>' => (current_pos.0, current_pos.1 + 1),
        'v' => (current_pos.0 + 1, current_pos.1),
        '<' => (current_pos.0, current_pos.1 - 1),
        _ => panic!("Shouldn't get here"),
    };
    pad.get_by_right(&new_location).copied()
}

fn solve_pt2(code: String, depth: i32) -> Option<i64> {
    let num_pad_map = build_dir_map(true);
    let dir_pad_map = build_dir_map(false);

    let potential_paths = layer_one_paths(code.to_owned(), &num_pad_map);
    potential_paths
        .iter()
        .map(|c| calculate_length(c, depth, &dir_pad_map))
        .min()
}

#[cached(key = "(String, i32)", convert = "{ (sequence.to_string(), depth) }")]
fn calculate_length(sequence: &str, depth: i32, dir_pad_map: &HashMap<String, Vec<String>>) -> i64 {
    if depth == 1 {
        return sequence
            .chars()
            .zip("A".chars().chain(sequence.chars()))
            .map(|(x, y)| dir_pad_map.get(&format!("{x}{y}")).unwrap()[0].len() as i64)
            .sum();
    }

    sequence
        .chars()
        .zip("A".chars().chain(sequence.chars()))
        .map(|(x, y)| {
            dir_pad_map
                .get(&format!("{y}{x}"))
                .unwrap()
                .iter()
                .map(|subseq| calculate_length(subseq, depth - 1, &dir_pad_map))
                .min()
                .unwrap_or(0)
        })
        .sum()
}

fn layer_one_paths(code: String, num_pad: &HashMap<String, Vec<String>>) -> Vec<String> {
    zip(("A".to_string() + &code).chars(), code.chars())
        .map(|(f, t)| format!("{f}{t}"))
        .filter_map(|m| num_pad.get(&m))
        .multi_cartesian_product()
        .map(|combination| combination.iter().map(|s| s.as_str()).collect::<String>())
        .collect()
}

fn build_dir_map(num_pad: bool) -> HashMap<String, Vec<String>> {
    let mut dir_map = HashMap::new();

    let pad = if num_pad { &NUM_PAD } else { &DIR_PAD };
    for from in pad.left_values() {
        for to in pad.left_values() {
            let mut q: VecDeque<(char, Vec<char>)> = VecDeque::from([(*from, Vec::new())]);
            let mut visited = HashSet::new();
            let mut shortest_paths: Vec<String> = Vec::new();

            while let Some((current, journey)) = q.pop_front() {
                if current == *to {
                    if shortest_paths.is_empty() || journey.len() + 1 == shortest_paths[0].len() {
                        shortest_paths.push(String::from_iter(journey) + "A");
                    }
                    continue;
                }

                for dir in ['^', '>', 'v', '<'] {
                    if let Some(next) = find_next(dir, current, num_pad) {
                        if visited.contains(&next) {
                            continue;
                        }
                        let mut new_journey = journey.clone();
                        new_journey.push(dir);
                        q.push_back((next, new_journey));
                    }
                }

                visited.insert(current);
            }
            dir_map.insert(format!("{}{}", from, to), shortest_paths);
        }
    }
    dir_map
}
