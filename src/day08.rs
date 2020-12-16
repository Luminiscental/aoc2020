use crate::day::Day;
use std::collections::HashSet;

struct Failure {
    acc: i32,
    visited: HashSet<i32>,
}

fn run_code(code: &[(&str, i32)]) -> Result<i32, Failure> {
    let mut ip = 0;
    let mut acc = 0;
    let mut visited = HashSet::new();
    while (ip as usize) != code.len() {
        if !visited.insert(ip) {
            return Err(Failure { acc, visited });
        }
        match code[ip as usize] {
            ("nop", _) => {
                ip += 1;
            }
            ("acc", d) => {
                acc += d;
                ip += 1;
            }
            ("jmp", d) => {
                ip += d;
            }
            _ => unreachable!(),
        }
    }
    Ok(acc)
}

pub struct Day08 {}

impl<'a> Day<'a> for Day08 {
    type Input1 = Vec<(&'a str, i32)>;
    type Input2 = (Vec<(&'a str, i32)>, HashSet<i32>);
    type Output1 = i32;
    type Output2 = i32;

    const INDEX: usize = 8;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        raw_input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| (&line[0..3], line[4..].parse::<i32>().unwrap()))
            .collect()
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        match run_code(&input) {
            Ok(_) => panic!("no loop found"),
            Err(failure) => ((input, failure.visited), failure.acc),
        }
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        let (mut input, visited) = input;
        let candidates: Vec<_> = input
            .iter()
            .enumerate()
            .filter(|idx_val| visited.contains(&(idx_val.0 as i32)))
            .filter_map(|idx_val| match idx_val.1 {
                ("jmp", _) => Some((idx_val.0, "nop")),
                ("nop", d) if *d != 0 => Some((idx_val.0, "jmp")),
                _ => None,
            })
            .collect();
        for (idx, replacement) in candidates.into_iter() {
            let prev = input[idx].0;
            input[idx].0 = replacement;
            if let Ok(acc) = run_code(&input) {
                return acc;
            }
            input[idx].0 = prev;
        }
        panic!("no fix found");
    }
}
