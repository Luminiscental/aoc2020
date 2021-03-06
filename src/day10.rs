use crate::day::Day;
use std::iter;

pub struct Day10 {}

impl<'a> Day<'a> for Day10 {
    type Input1 = Vec<usize>;
    type Input2 = Vec<usize>;
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 10;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        let mut jolt_adaptors: Vec<_> = raw_input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<usize>().unwrap())
            .collect();
        jolt_adaptors.sort();
        iter::once(jolt_adaptors[0])
            .chain(
                jolt_adaptors
                    .iter()
                    .skip(1)
                    .zip(jolt_adaptors.iter())
                    .map(|pair| {
                        let (next, prev) = pair;
                        next - prev
                    }),
            )
            .chain(iter::once(3))
            .collect()
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let mut one_count = 0;
        let mut three_count = 0;
        for jolt_difference in input.iter() {
            match jolt_difference {
                1 => one_count += 1,
                3 => three_count += 1,
                _ => panic!("expected only gaps of 1 or 3 jolts"),
            }
        }
        (input, one_count * three_count)
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        // I only get up to 7 in this sequence with my input
        let tribonacci = &[1, 1, 2, 4, 7, 11, 24];
        input
            .split(|&n| n == 3)
            .map(|xs| tribonacci[xs.len()])
            .product()
    }
}
