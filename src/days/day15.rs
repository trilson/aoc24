use crate::{Solution, SolutionPair};
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day15.txt").unwrap();
    let mut split = input.split("\r\n\r\n");
    let grid: Vec<Vec<char>> = split
        .next()
        .expect("Must have grid")
        .split("\r\n")
        .map(|c| c.chars().collect())
        .collect();

    let instructions = split
        .next()
        .expect("Must have instructions")
        .replace("\r\n", "");

    let mut dir_map: HashMap<char, (i32, i32)> = HashMap::new();
    dir_map.insert('^', (-1, 0));
    dir_map.insert('>', (0, 1));
    dir_map.insert('v', (1, 0));
    dir_map.insert('<', (0, -1));

    let sol1 = solve_pt1(&grid, &instructions, &dir_map);
    let sol2 = solve_pt2(&grid, &instructions, &dir_map);
    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_pt1(
    input_grid: &Vec<Vec<char>>,
    instructions: &String,
    dir_map: &HashMap<char, (i32, i32)>,
) -> i32 {
    let mut grid = input_grid.clone();
    let (mut start_row, mut start_col) = (0, 0);
    for (row, l) in grid.iter().enumerate() {
        for (col, ch) in l.iter().enumerate() {
            if *ch == '@' {
                start_row = row as i32;
                start_col = col as i32;
                break;
            }
        }
    }
    for instruction in instructions.chars() {
        let dir = dir_map.get(&instruction).expect("Invalid instruction");
        let (row, col) = (start_row + dir.0, start_col + dir.1);

        if let Some(ch) = grid.get(row as usize).and_then(|r| r.get(col as usize)) {
            if *ch == 'O' {
                let (mut new_row, mut new_col) = (row, col);
                while let Some(&ch) = grid
                    .get(new_row as usize)
                    .and_then(|r| r.get(new_col as usize))
                {
                    match ch {
                        '#' => break,
                        '.' => {
                            grid.get_mut(new_row as usize).unwrap()[new_col as usize] = 'O';
                            grid.get_mut(row as usize).unwrap()[col as usize] = '.';
                            break;
                        }
                        _ => {
                            new_row += dir.0;
                            new_col += dir.1;
                        }
                    }
                }
            }
            let updated_char = grid.get(row as usize).unwrap()[col as usize];
            if updated_char == '.' || updated_char == '@' {
                start_row += dir.0;
                start_col += dir.1;
            }
        }
    }
    score(&grid, 'O')
}

fn solve_pt2(
    input_grid: &Vec<Vec<char>>,
    instructions: &String,
    dir_map: &HashMap<char, (i32, i32)>,
) -> i32 {
    let (mut sr, mut scol) = (0i32, 0i32);

    let mut grid: Vec<Vec<char>> = Vec::new();
    for (idx, line) in input_grid.iter().enumerate() {
        let mut row: Vec<char> = Vec::new();
        for ch in line.iter() {
            match ch {
                '@' => {
                    row.extend(vec!['.', '.']);
                    sr = idx as i32;
                    scol = (row.len() - 2) as i32;
                }
                'O' => row.extend(vec!['[', ']']),
                '#' => row.extend(vec!['#', '#']),
                _ => row.extend(vec!['.', '.']),
            }
        }
        grid.push(row);
    }

    for instruction in instructions.chars() {
        let (dr, dc) = *dir_map.get(&instruction).expect("Invalid instruction");
        let row = sr + dr;
        let col = scol + dc;
        let row_u = row as usize;
        let col_u = col as usize;
        let ch = grid[row_u][col_u];

        if dr == 0 {
            if ch == '[' || ch == ']' {
                let mut nc = col;
                loop {
                    let sr_u = sr as usize;
                    let nc_u = nc as usize;
                    let ch = grid[sr_u][nc_u];

                    if ch == '#' {
                        break;
                    }
                    if ch == '.' {
                        let col_u = col as usize;
                        let sr_u = sr as usize;
                        if dc < 0 {
                            grid[sr_u].copy_within(nc_u + 1..(col_u + 1), nc_u);
                        } else {
                            grid[sr_u].copy_within(col_u..nc_u, col_u + 1);
                        }
                        grid[sr_u][col_u] = '.';
                        break;
                    }
                    nc += dc;
                }
            }
        } else {
            if ch == '[' || ch == ']' {
                let st = if ch == '[' { scol } else { scol - 1 };
                let mut box_layers: Vec<HashSet<usize>> = Vec::new();
                let mut first_layer = HashSet::new();
                first_layer.insert(st as usize);
                box_layers.push(first_layer);

                let mut layer = 1;
                let mut can_progress = true;
                let mut success = false;

                while can_progress {
                    let mut next_layer: HashSet<usize> = HashSet::new();
                    let prev_layer = &box_layers[layer - 1];

                    for &bx in prev_layer {
                        let above = (sr + ((1 + layer as i32) * dr)) as usize;

                        let left = grid[above][bx];
                        if left == '#' {
                            can_progress = false;
                            break;
                        }
                        if left == ']' {
                            next_layer.insert(bx - 1);
                        }
                        if left == '[' {
                            next_layer.insert(bx);
                        }

                        let right = grid[above][bx + 1];
                        if right == '#' {
                            can_progress = false;
                            break;
                        }
                        if right == '[' {
                            next_layer.insert(bx + 1);
                        }
                        if right == ']' {
                            next_layer.insert(bx);
                        }
                    }
                    if next_layer.is_empty() && can_progress {
                        success = true;
                        break;
                    }
                    box_layers.push(next_layer);
                    layer += 1;
                }

                if success {
                    for i in (0..box_layers.len()).rev() {
                        for &bx in &box_layers[i] {
                            let before = (sr + (2 * dr) + (i as i32 * dr)) as usize;
                            let after = (sr + dr + (i as i32 * dr)) as usize;

                            grid[before][bx..bx + 2].copy_from_slice(&['[', ']']);
                            grid[after][bx..bx + 2].copy_from_slice(&['.', '.']);
                        }
                    }
                }
            }
        }
        if grid[row_u][col_u] == '.' {
            sr = row;
            scol = col;
        }
    }

    score(&grid, '[')
}

fn score(grid: &Vec<Vec<char>>, target: char) -> i32 {
    let mut sol1 = 0;
    for (idx, line) in grid.iter().enumerate() {
        for (idy, ch) in line.iter().enumerate() {
            if *ch == target {
                sol1 += 100 * idx;
                sol1 += idy;
            }
        }
    }
    sol1 as i32
}
