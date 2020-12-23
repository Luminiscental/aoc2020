use crate::day::Day;
use std::iter;

fn dec_cup(cup: usize, cup_count: usize) -> usize {
    match cup {
        1 => cup_count,
        n => n - 1,
    }
}

pub struct Cups {
    current: usize,
    next: Vec<usize>,
}

impl Iterator for Cups {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let item = self.current;
        self.current = self.next_cup(self.current);
        Some(item)
    }
}

impl Cups {
    fn next_cup(&self, cup: usize) -> usize {
        self.next[cup - 1]
    }

    fn set_next_cup(&mut self, cup: usize, next_cup: usize) {
        self.next[cup - 1] = next_cup;
    }

    fn iter_from(self, cup: usize) -> Self {
        Self {
            current: cup,
            ..self
        }
    }

    fn play_moves(mut self, move_count: usize) -> Self {
        for _ in 1..=move_count {
            self.perform_move();
        }
        self
    }

    fn from_slice(slice: &[usize]) -> Self {
        let current = slice[0];
        let mut next: Vec<_> = slice.into();
        let cups = slice.iter().copied();
        let cups_shifted = slice.iter().copied().skip(1).chain(iter::once(slice[0]));
        for (cup, next_cup) in cups.zip(cups_shifted) {
            next[cup - 1] = next_cup;
        }
        Cups { current, next }
    }

    fn find_dest(&self, picked_up: &[usize]) -> usize {
        itertools::iterate(self.current, |&cup| dec_cup(cup, self.next.len()))
            .skip(1)
            .find(|cup| !picked_up.contains(cup))
            .unwrap()
    }

    fn perform_move(&mut self) {
        // pick up cups
        let pickup1 = self.next_cup(self.current);
        let pickup2 = self.next_cup(pickup1);
        let pickup3 = self.next_cup(pickup2);
        self.set_next_cup(self.current, self.next_cup(pickup3));
        // find destination
        let dest_cup = self.find_dest(&[pickup1, pickup2, pickup3]);
        // put down cups
        self.set_next_cup(pickup3, self.next_cup(dest_cup));
        self.set_next_cup(dest_cup, pickup1);
        // advance current
        self.current = self.next_cup(self.current);
    }
}

pub struct Day23 {}

impl<'a> Day<'a> for Day23 {
    type Input1 = Vec<usize>;
    type Input2 = Vec<usize>;
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 23;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        raw_input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let output = Cups::from_slice(&input)
            .play_moves(100)
            .iter_from(1)
            .skip(1)
            .take(8)
            .zip(itertools::iterate(10000000, |n| n / 10))
            .map(|pair| pair.0 * pair.1)
            .sum();
        (input, output)
    }

    fn solve_part2(mut input: Self::Input2) -> Self::Output2 {
        input.reserve(1000000 - 9);
        for i in 10..=1000000 {
            input.push(i);
        }
        Cups::from_slice(&input)
            .play_moves(10000000)
            .iter_from(1)
            .skip(1)
            .take(2)
            .product()
    }
}
