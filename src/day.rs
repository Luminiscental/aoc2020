use std::{fmt, time::Instant};

pub trait Day<'a> {
    type Input;
    type Output1;
    type Output2;

    const INDEX: usize;

    fn parse(raw_input: &'a str) -> Self::Input;
    fn solve_part1(input: &Self::Input) -> Self::Output1;
    fn solve_part2(input: &Self::Input) -> Self::Output2;

    fn solve_and_print(raw_input: &'a str)
    where
        Self::Output1: fmt::Display,
        Self::Output2: fmt::Display,
    {
        let parsed_input = Self::parse(&raw_input);
        let time1 = Instant::now();
        let part1_answer = Self::solve_part1(&parsed_input);
        let time2 = Instant::now();
        let part2_answer = Self::solve_part2(&parsed_input);
        let time3 = Instant::now();

        println!();
        println!("day{}:", Self::INDEX);
        println!(
            "  part1: {} (elapsed {}ms)",
            part1_answer,
            1000.0 * (time2 - time1).as_secs_f32()
        );
        println!(
            "  part2: {} (elapsed {}ms)",
            part2_answer,
            1000.0 * (time3 - time2).as_secs_f32()
        );
    }
}
