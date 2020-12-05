use crate::day::Day;

type Seat = usize;

fn parse_seat(desc: &str) -> Seat {
    let row: usize = desc[0..7]
        .as_bytes()
        .iter()
        .rev()
        .zip(0..)
        .filter(|pair| *pair.0 == b'B')
        .map(|pair| 1 << pair.1)
        .sum();
    let column: usize = desc[7..10]
        .as_bytes()
        .iter()
        .rev()
        .zip(0..)
        .filter(|pair| *pair.0 == b'R')
        .map(|pair| 1 << pair.1)
        .sum();
    8 * row + column
}

pub struct Day5 {}

impl<'a> Day<'a> for Day5 {
    type Input1 = Vec<Seat>;
    type Input2 = Vec<Seat>;
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 5;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        raw_input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| parse_seat(&line[0..10]))
            .collect()
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let max_id = *input.iter().max().unwrap();
        (input, max_id)
    }

    fn solve_part2(mut input: Self::Input2) -> Self::Output2 {
        input.sort();
        let (left, _right) = input
            .iter()
            .zip(input.iter().skip(1))
            .find(|pair| pair.1 - pair.0 == 2)
            .unwrap();
        left + 1
    }
}
