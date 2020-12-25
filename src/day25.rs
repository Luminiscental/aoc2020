use crate::{day::Day, util};

pub struct Day25 {}

impl<'a> Day<'a> for Day25 {
    type Input1 = (usize, usize);
    type Input2 = ();
    type Output1 = usize;
    type Output2 = &'static str;

    const INDEX: usize = 25;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        let mut lines = raw_input.lines().filter(|line| !line.is_empty());
        let door_public = lines.next().and_then(|line| line.parse().ok()).unwrap();
        let card_public = lines.next().and_then(|line| line.parse().ok()).unwrap();
        (door_public, card_public)
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        const PRIME: usize = 20201227;
        let (door_public, card_public) = input;
        let mut public_key = 1;
        for loopsize in 1.. {
            public_key *= 7;
            public_key %= PRIME;
            if public_key == door_public {
                return ((), util::exp_modular(card_public, loopsize, PRIME));
            } else if public_key == card_public {
                return ((), util::exp_modular(door_public, loopsize, PRIME));
            }
        }
        unreachable!()
    }

    fn solve_part2(_input: Self::Input2) -> Self::Output2 {
        "Go click that button ;)"
    }
}
