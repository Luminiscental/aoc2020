use crate::day::Day;
use crate::util;

pub struct Day13 {}

impl<'a> Day<'a> for Day13 {
    type Input1 = (usize, Vec<Option<usize>>);
    type Input2 = Vec<Option<usize>>;
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 13;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        let lines: Vec<_> = raw_input.lines().filter(|line| !line.is_empty()).collect();
        (
            lines[0].parse::<usize>().unwrap(),
            lines[1]
                .split(',')
                .map(|s| match s {
                    "x" => None,
                    n => Some(n.parse::<usize>().unwrap()),
                })
                .collect(),
        )
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let (min, buses) = input;
        let (earliest_bus, wait_time) = buses
            .iter()
            .filter_map(|&bus| bus)
            .map(|bus| match min % bus {
                0 => (bus, 0),
                r => (bus, bus - r),
            })
            .min_by_key(|pair| pair.1)
            .unwrap();
        (buses, earliest_bus * wait_time)
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        let congruences: Vec<_> = input
            .iter()
            .enumerate()
            .filter_map(|pair| match pair {
                (_, None) => None,
                (idx, Some(n)) => Some(((-(idx as i32)).rem_euclid(*n as i32) as usize, *n)),
            })
            .collect();
        let mut solution = 0;
        let big_modulo: usize = congruences.iter().map(|pair| pair.1).product();
        for (remainder, small_modulo) in congruences {
            let zero_for_others = big_modulo / small_modulo;
            solution += zero_for_others * util::prime_divide_modular(remainder, zero_for_others, small_modulo);
        }
        solution % big_modulo
    }
}
