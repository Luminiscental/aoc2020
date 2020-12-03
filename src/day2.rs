use crate::day::Day;
use itertools::Itertools;

type Password<'a> = &'a [u8];

pub struct Policy {
    min: usize,
    max: usize,
    letter: u8,
}

impl Policy {
    fn validate_sled(&self, password: &[u8]) -> bool {
        let count = bytecount::count(password, self.letter);
        self.min <= count && count <= self.max
    }

    fn validate_toboggan(&self, password: &[u8]) -> bool {
        (password[self.min - 1] == self.letter) ^ (password[self.max - 1] == self.letter)
    }
}

pub struct Day2 {}

impl<'a> Day<'a> for Day2 {
    type Input = Vec<(Policy, Password<'a>)>;
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 2;

    fn parse(input: &'a str) -> Self::Input {
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let (min, max, letter, password) = line
                    .split(&['-', ' ', ':'][..])
                    .filter(|s| !s.is_empty())
                    .next_tuple()
                    .expect("failed to parse line");
                let min = min.parse::<usize>().expect("failed to parse min");
                let max = max.parse::<usize>().expect("failed to parse max");
                assert!(letter.len() == 1, "expected single letter");
                let letter = letter.as_bytes()[0];
                (Policy { min, max, letter }, password.as_bytes())
            })
            .collect()
    }

    fn solve_part1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .filter(|pair| pair.0.validate_sled(pair.1))
            .count()
    }

    fn solve_part2(input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .filter(|pair| pair.0.validate_toboggan(pair.1))
            .count()
    }
}
