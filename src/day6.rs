use crate::day::Day;

#[derive(Clone, Copy)]
pub struct AlphabetSet {
    flags: u32,
}

impl AlphabetSet {
    fn from(string: &str) -> Self {
        Self {
            flags: string.bytes().fold(0, |flags, c| flags | (1 << (c - b'a'))),
        }
    }

    fn union(self, other: &Self) -> Self {
        Self {
            flags: self.flags | other.flags,
        }
    }

    fn intersection(self, other: &Self) -> Self {
        Self {
            flags: self.flags & other.flags,
        }
    }

    fn count(&self) -> u32 {
        self.flags.count_ones()
    }
}

pub struct Day6 {}

impl<'a> Day<'a> for Day6 {
    type Input1 = Vec<Vec<AlphabetSet>>;
    type Input2 = Vec<Vec<AlphabetSet>>;
    type Output1 = u32;
    type Output2 = u32;

    const INDEX: usize = 6;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        let lines: Vec<_> = raw_input.lines().collect();
        lines
            .split(|line| line.is_empty())
            .filter(|group| !group.is_empty())
            .map(|group| group.iter().map(|s| AlphabetSet::from(s)).collect())
            .collect()
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let sum = input
            .iter()
            .map(|group| {
                group[1..]
                    .iter()
                    .fold(group[0], |counter, person| counter.union(person))
                    .count()
            })
            .sum();
        (input, sum)
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        input
            .iter()
            .map(|group| {
                group[1..]
                    .iter()
                    .fold(group[0], |counter, person| counter.intersection(person))
                    .count()
            })
            .sum()
    }
}
