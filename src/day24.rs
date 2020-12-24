use crate::{
    day::Day,
    util::{self, Coord},
};
use std::collections::HashSet;

// invariant: x + y + z == 0
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct HexCoord(i32, i32, i32);

impl Coord for HexCoord {
    fn for_neighbours<F: FnMut(Self)>(&self, mut f: F) {
        f(Self(self.0, self.1 + 1, self.2 - 1));
        f(Self(self.0, self.1 - 1, self.2 + 1));
        f(Self(self.0 - 1, self.1 + 1, self.2));
        f(Self(self.0 - 1, self.1, self.2 + 1));
        f(Self(self.0 + 1, self.1 - 1, self.2));
        f(Self(self.0 + 1, self.1, self.2 - 1));
    }
}

pub struct Day24 {}

impl<'a> Day<'a> for Day24 {
    type Input1 = Vec<HexCoord>;
    type Input2 = HashSet<HexCoord>;
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 24;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        raw_input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|mut line| {
                let mut cursor = HexCoord(0, 0, 0);
                while !line.is_empty() {
                    let (skip, dir) = match &line[0..1] {
                        "n" => match &line[1..2] {
                            "e" => (2, (0, 1, -1)),
                            "w" => (2, (1, 0, -1)),
                            _ => unreachable!(),
                        },
                        "s" => match &line[1..2] {
                            "e" => (2, (-1, 0, 1)),
                            "w" => (2, (0, -1, 1)),
                            _ => unreachable!(),
                        },
                        "e" => (1, (-1, 1, 0)),
                        "w" => (1, (1, -1, 0)),
                        _ => unreachable!(),
                    };
                    cursor.0 += dir.0;
                    cursor.1 += dir.1;
                    cursor.2 += dir.2;
                    line = &line[skip..];
                }
                cursor
            })
            .collect()
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let mut grid = HashSet::new();
        for &tile in input.iter() {
            if !grid.insert(tile) {
                grid.remove(&tile);
            }
        }
        let black_count = grid.len();
        (grid, black_count)
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        let mut grid = input;
        for _ in 1..=100 {
            util::step_cellular_automata(&mut grid, |n| n == 2, |n| (n != 1 && n != 2));
        }
        grid.len()
    }
}
