use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let input = lines_from_file("input/day23.txt");
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    for mut con in input.iter().map(|c| c.split('-')) {
        let (from, to) = (
            con.next().expect("Must have left"),
            con.next().expect("Must have right"),
        );
        graph.entry(from).or_insert(HashSet::new()).insert(to);
        graph.entry(to).or_insert(HashSet::new()).insert(from);
    }

    let sol1: i32 = solve_pt1(&graph);
    let sol2: String = solve_pt2(&graph);

    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_pt1(graph: &HashMap<&str, HashSet<&str>>) -> i32 {
    let mut triplets = HashSet::new();
    for (&root, links) in graph.iter() {
        for pair in links.iter().combinations(2) {
            let (l, r) = (pair[0], pair[1]);
            let is_triangle = graph.get(l).map_or(false, |adj| adj.contains(r));
            let has_t = root.starts_with('t') || l.starts_with('t') || r.starts_with('t');
            if is_triangle && has_t {
                let mut triple = [root, l, r];
                triple.sort_unstable();
                triplets.insert(triple);
            }
        }
    }
    triplets.len() as i32
}

fn solve_pt2(graph: &HashMap<&str, HashSet<&str>>) -> String {
    let r = HashSet::new();
    let p: HashSet<_> = graph.keys().copied().collect();
    let x = HashSet::new();

    let mut all_cliques = Vec::new();
    bron_kerbosch(&r, &p, &x, &graph, &mut all_cliques);

    all_cliques
        .iter()
        .max_by_key(|clique| clique.len())
        .map(|clique| clique.iter().sorted().join(","))
        .expect("Must have a max clique")
}

fn bron_kerbosch<'a>(
    current_clique: &HashSet<&'a str>,
    p: &HashSet<&'a str>,
    x: &HashSet<&'a str>,
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    all_cliques: &mut Vec<HashSet<&'a str>>,
) {
    if p.is_empty() && x.is_empty() {
        all_cliques.push(current_clique.clone());
        return;
    }

    let pivot = p.union(x).next().unwrap();
    let pivot_neighbors: HashSet<&str> = graph.get(pivot).cloned().unwrap_or_else(HashSet::new);

    let p_without_pivot_neighbors: Vec<_> = p.difference(&pivot_neighbors).copied().collect();

    for &v in &p_without_pivot_neighbors {
        let mut new_clique = current_clique.clone();
        new_clique.insert(v);

        let neighbors_v: HashSet<&str> = graph.get(v).cloned().unwrap_or_else(HashSet::new);
        let p_new: HashSet<_> = p.intersection(&neighbors_v).copied().collect();
        let x_new: HashSet<_> = x.intersection(&neighbors_v).copied().collect();

        bron_kerbosch(&new_clique, &p_new, &x_new, graph, all_cliques);
    }
}
