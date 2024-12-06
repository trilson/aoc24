use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let lines: Vec<String> = lines_from_file("input/day05.txt");

    let mut sol1: i32 = 0;
    let mut sol2: i32 = 0;

    let mut dep_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    for l in lines.iter() {
        if l.contains('|') {
            let mut l_split = l.split('|').rev();
            dep_map
                .entry(l_split.next().unwrap())
                .or_insert_with(|| HashSet::new())
                .insert(l_split.next().unwrap());
        } else if l != "" {
            let mut pages: Vec<&str> = l.split(',').collect();
            if is_valid(&pages, &dep_map) {
                sol1 += mid_point(&pages);
            } else {
                pages.sort_by(|a, b| compare(a, b, &dep_map));
                sol2 += mid_point(&pages);
            }
        }
    }
    (Solution::from(sol1), Solution::from(sol2))
}

fn mid_point(pages: &Vec<&str>) -> i32 {
    pages[(pages.len() + 1) / 2 - 1]
        .parse::<i32>()
        .expect("should be a number")
}

fn compare(a: &str, b: &str, dep_map: &HashMap<&str, HashSet<&str>>) -> Ordering {
    if dep_map.get(b).unwrap_or(&HashSet::new()).contains(a) {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}
fn is_valid(pages: &Vec<&str>, dep_map: &HashMap<&str, HashSet<&str>>) -> bool {
    let mut forbidden: HashSet<&str> = HashSet::new();
    let mut p_iter = pages.iter();
    while let Some(val) = p_iter.next() {
        if forbidden.contains(val) {
            return false;
        }
        dep_map
            .get(val)
            .map(|be: &HashSet<&str>| forbidden.extend(be));
    }
    true
}
