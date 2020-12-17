use crate::day::Day;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    iter,
};

trait Ignorable {
    fn ignore(self);
}

impl<T> Ignorable for T {
    fn ignore(self) {}
}

fn step_grid_3d(
    grid: &mut HashSet<(i32, i32, i32)>,
    initial_width: usize,
    initial_height: usize,
    step: usize,
) {
    lazy_static! {
        static ref DIRECTIONS_3D: Vec<(i32, i32, i32)> = iter::repeat(-1..=1)
            .take(3)
            .multi_cartesian_product()
            .map(|v| (v[0], v[1], v[2]))
            .filter(|&pos| pos != (0, 0, 0))
            .collect();
    }
    let x_range = || -(step as i32)..(initial_width + step) as i32;
    let y_range = || -(step as i32)..(initial_height + step) as i32;
    let z_range = || -(step as i32)..(step + 1) as i32;
    let mut neighbours = HashMap::new();
    for z in z_range() {
        for y in y_range() {
            for x in x_range() {
                for (dx, dy, dz) in DIRECTIONS_3D.iter() {
                    let count = neighbours.entry((x, y, z)).or_insert(0);
                    if grid.contains(&(x + dx, y + dy, z + dz)) {
                        *count += 1;
                    }
                }
            }
        }
    }
    for z in z_range() {
        for y in y_range() {
            for x in x_range() {
                match (grid.contains(&(x, y, z)), neighbours[&(x, y, z)]) {
                    (true, 2) => (),
                    (true, 3) => (),
                    (true, _) => grid.remove(&(x, y, z)).ignore(),
                    (false, 3) => grid.insert((x, y, z)).ignore(),
                    (false, _) => (),
                }
            }
        }
    }
}

fn step_grid_4d(
    grid: &mut HashSet<(i32, i32, i32, i32)>,
    initial_width: usize,
    initial_height: usize,
    step: usize,
) {
    lazy_static! {
        static ref DIRECTIONS_4D: Vec<(i32, i32, i32, i32)> = iter::repeat(-1..=1)
            .take(4)
            .multi_cartesian_product()
            .map(|v| (v[0], v[1], v[2], v[3]))
            .filter(|&pos| pos != (0, 0, 0, 0))
            .collect();
    }
    let x_range = || -(step as i32)..(initial_width + step) as i32;
    let y_range = || -(step as i32)..(initial_height + step) as i32;
    let z_range = || -(step as i32)..(step + 1) as i32;
    let w_range = z_range;
    let mut neighbours = HashMap::new();
    for w in w_range() {
        for z in z_range() {
            for y in y_range() {
                for x in x_range() {
                    for (dx, dy, dz, dw) in DIRECTIONS_4D.iter() {
                        let count = neighbours.entry((x, y, z, w)).or_insert(0);
                        if grid.contains(&(x + dx, y + dy, z + dz, w + dw)) {
                            *count += 1;
                        }
                    }
                }
            }
        }
    }
    for w in w_range() {
        for z in z_range() {
            for y in y_range() {
                for x in x_range() {
                    match (grid.contains(&(x, y, z, w)), neighbours[&(x, y, z, w)]) {
                        (true, 2) => (),
                        (true, 3) => (),
                        (true, _) => grid.remove(&(x, y, z, w)).ignore(),
                        (false, 3) => grid.insert((x, y, z, w)).ignore(),
                        (false, _) => (),
                    }
                }
            }
        }
    }
}

pub struct Day17 {}

impl<'a> Day<'a> for Day17 {
    type Input1 = (usize, usize, HashSet<(i32, i32, i32)>);
    type Input2 = (usize, usize, HashSet<(i32, i32, i32, i32)>);
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 17;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        let mut width = 0;
        let mut height = 0;
        let mut grid = HashSet::new();
        for (y, line) in raw_input
            .lines()
            .filter(|line| !line.is_empty())
            .enumerate()
        {
            width = line.len();
            height = y + 1;
            for (x, c) in line.char_indices() {
                if c == '#' {
                    grid.insert((x as i32, y as i32, 0));
                }
            }
        }
        (width, height, grid)
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let (width, height, input) = input;
        let mut grid = input.clone();
        for i in 1..=6 {
            step_grid_3d(&mut grid, width, height, i);
        }
        (
            (
                width,
                height,
                input.iter().map(|pos| (pos.0, pos.1, pos.2, 0)).collect(),
            ),
            grid.len(),
        )
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        let (width, height, mut grid) = input;
        for i in 1..=6 {
            step_grid_4d(&mut grid, width, height, i);
        }
        grid.len()
    }
}
