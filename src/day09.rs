use crate::day::Day;
use std::{cmp::Ordering, collections::HashMap};

pub struct Day09 {}

impl<'a> Day<'a> for Day09 {
    type Input1 = Vec<usize>;
    type Input2 = (usize, Vec<usize>);
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 9;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        raw_input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<usize>().unwrap())
            .collect()
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let mut available_sums = HashMap::new();
        for (i, num1) in input[0..25].iter().enumerate() {
            for num2 in input[0..i].iter() {
                *available_sums.entry(num1 + num2).or_insert(0) += 1;
            }
        }
        let mut current_idx = 25;
        while current_idx < input.len() {
            let value = input[current_idx];
            if available_sums.get(&value).filter(|&&n| n > 0).is_some() {
                let lost_idx = current_idx - 25;
                let lost_value = input[lost_idx];
                let new_value = input[current_idx];
                for num in input[lost_idx + 1..current_idx].iter() {
                    *available_sums.entry(lost_value + num).or_insert(0) -= 1;
                    *available_sums.entry(num + new_value).or_insert(0) += 1;
                }
                current_idx += 1;
            } else {
                return ((value, input), value);
            }
        }
        panic!("no solution found");
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        let (value, input) = input;
        let mut low = 0;
        let mut high = 1;
        while high < input.len() {
            let sum: usize = input[low..=high].iter().sum();
            match sum.cmp(&value) {
                Ordering::Less => high += 1,
                Ordering::Greater if low < high - 1 => low += 1,
                Ordering::Greater => {
                    // I don't end up in this path but theoretically it should be necessary
                    low += 1;
                    high += 1;
                }
                Ordering::Equal => {
                    let small = input[low..=high].iter().min().unwrap();
                    let big = input[low..=high].iter().max().unwrap();
                    return small + big;
                }
            }
        }
        panic!("no solution found");
    }
}
