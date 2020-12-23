use crate::day::Day;

fn dec_cup(cup: usize, cup_count: usize) -> usize {
    match cup {
        1 => cup_count,
        n => n - 1,
    }
}

struct IterCups<'a> {
    current: usize,
    next: &'a Vec<usize>,
}

impl<'a> Iterator for IterCups<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let item = Some(self.current);
        self.current = self.next[self.current - 1];
        item
    }
}

#[derive(Debug, Clone)]
pub struct Cups {
    current: usize,
    next: Vec<usize>,
}

impl Cups {
    fn from_slice(slice: &[usize]) -> Self {
        let current = slice[0];
        let mut next: Vec<_> = slice.into();
        for (value, next_value) in slice.iter().copied().zip(slice.iter().copied().skip(1)) {
            next[value - 1] = next_value;
        }
        next[slice[slice.len() - 1] - 1] = slice[0];
        Cups { current, next }
    }

    fn next(&self, cup: usize) -> usize {
        self.next[cup - 1]
    }

    fn set_next(&mut self, cup: usize, next_cup: usize) {
        self.next[cup - 1] = next_cup;
    }

    fn iter_from(&self, cup: usize) -> IterCups<'_> {
        IterCups {
            current: cup,
            next: &self.next,
        }
    }

    fn perform_move(&mut self) {
        // pick up the cups
        let pickup1 = self.next(self.current);
        let pickup2 = self.next(pickup1);
        let pickup3 = self.next(pickup2);
        self.set_next(self.current, self.next(pickup3));
        // find the destination
        let dest_cup = itertools::iterate(self.current, |&cup| dec_cup(cup, self.next.len()))
            .skip(1)
            .find(|cup| ![pickup1, pickup2, pickup3].contains(cup))
            .unwrap();
        // put down the cups
        self.set_next(pickup3, self.next(dest_cup));
        self.set_next(dest_cup, pickup1);
        // advance current
        self.current = self.next(self.current);
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
        let mut cups = Cups::from_slice(&input);
        for _ in 1..=100 {
            cups.perform_move();
        }
        (
            input,
            cups.iter_from(1)
                .skip(1)
                .take(8)
                .zip(itertools::iterate(10000000, |n| n / 10))
                .map(|pair| pair.0 * pair.1)
                .sum(),
        )
    }

    fn solve_part2(mut input: Self::Input2) -> Self::Output2 {
        input.reserve(1000000 - 9);
        for i in 10..=1000000 {
            input.push(i);
        }
        let mut cups = Cups::from_slice(&input);
        for _ in 1..=10000000 {
            cups.perform_move();
        }
        cups.iter_from(1).skip(1).take(2).product()
    }
}
