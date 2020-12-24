use crate::{
    day::Day,
    util::{self, Coord3DWithDiagonals, Coord4DWithDiagonals},
};
use std::collections::HashSet;

pub struct Day17 {}

impl<'a> Day<'a> for Day17 {
    type Input1 = HashSet<Coord3DWithDiagonals>;
    type Input2 = HashSet<Coord4DWithDiagonals>;
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 17;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        let mut grid = HashSet::new();
        for (y, line) in raw_input
            .lines()
            .filter(|line| !line.is_empty())
            .enumerate()
        {
            for (x, c) in line.char_indices() {
                if c == '#' {
                    grid.insert(Coord3DWithDiagonals(x as i32, y as i32, 0));
                }
            }
        }
        grid
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let mut grid = input.clone();
        for _ in 1..=6 {
            util::step_cellular_automata(&mut grid, |n| n == 3, |n| (n != 2 && n != 3));
        }
        let grid4d = input
            .iter()
            .map(|pos| Coord4DWithDiagonals(pos.0, pos.1, pos.2, 0))
            .collect();
        (grid4d, grid.len())
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        let mut grid = input;
        for _ in 1..=6 {
            util::step_cellular_automata(&mut grid, |n| n == 3, |n| (n != 2 && n != 3));
        }
        grid.len()
    }
}
