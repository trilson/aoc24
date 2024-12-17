use std::fs::read_to_string;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, multispace0},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

use crate::{Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day17.txt").unwrap();
    let (_, (a, b, c, program)) = parse_input(&input).expect("Error parsing input");

    let sol1 = solve_pt1(a, b, c, program);
    let sol2 = 105706277661082_i64; // pt2.py
    (Solution::from(sol1), Solution::from(sol2))
}

fn combo(input: i64, a: i64, b: i64, c: i64) -> i64 {
    return match input {
        4 => a,
        5 => b,
        6 => c,
        _ => input,
    };
}

fn solve_pt1(a_in: i64, b_in: i64, c_in: i64, program: Vec<i64>) -> String {
    let mut start = 0;
    let mut out: Vec<i64> = Vec::new();

    let (mut a, mut b, mut c) = (a_in, b_in, c_in);

    while start < program.len() - 1 {
        let operation = program[start];
        let literal = program[start + 1];
        let combo = combo(literal, a, b, c);
        match operation {
            0 => a = (a / 2_i64.pow(combo as u32)) as i64,
            1 => b ^= literal,
            2 => b = combo % 8,
            3 => {
                if a != 0 {
                    start = literal as usize;
                    continue;
                }
            }
            4 => b ^= c,
            5 => out.push(combo % 8),
            6 => b = (a / 2_i64.pow(combo as u32)) as i64,
            7 => c = (a / 2_i64.pow(combo as u32)) as i64,
            _ => panic!("Unsupported operation"),
        }
        start += 2
    }
    out.iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn parse_register_line(input: &str) -> IResult<&str, i64> {
    let (input, (_, value)) = tuple((
        take_until(":"),
        preceded(tag(": "), map_res(digit1, |s: &str| s.parse::<i64>())),
    ))(input)?;
    Ok((input, value))
}

fn parse_program_line(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, _) = tag("Program: ")(input)?;
    let (input, numbers) =
        separated_list1(tag(","), map_res(digit1, |s: &str| s.parse::<i64>()))(input)?;
    Ok((input, numbers))
}

fn parse_input(input: &str) -> IResult<&str, (i64, i64, i64, Vec<i64>)> {
    let (input, (a, _, b, _, c, _, program)) = tuple((
        preceded(tag("Register A"), parse_register_line),
        multispace0,
        preceded(tag("Register B"), parse_register_line),
        multispace0,
        preceded(tag("Register C"), parse_register_line),
        multispace0,
        parse_program_line,
    ))(input)?;

    Ok((input, (a, b, c, program)))
}
