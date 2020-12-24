use crate::{day::Day, map};
use std::collections::{HashMap, HashSet};

fn directions() -> HashMap<&'static str, (i32, i32, i32)> {
    map! {
        "ne" => (0, 1, -1),
        "nw" => (1, 0, -1),
        "se" => (-1, 0, 1),
        "sw" => (0, -1, 1),
        "e" => (-1, 1, 0),
        "w" => (1, -1, 0)
    }
}

pub struct Day24 {}

impl<'a> Day<'a> for Day24 {
    type Input1 = Vec<(i32, i32, i32)>;
    type Input2 = HashSet<(i32, i32, i32)>;
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 24;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        let directions = directions();
        raw_input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|mut line| {
                let mut cursor = (0, 0, 0);
                while !line.is_empty() {
                    for (name, direction) in directions.iter() {
                        if line.starts_with(name) {
                            cursor.0 += direction.0;
                            cursor.1 += direction.1;
                            cursor.2 += direction.2;
                            line = &line[name.len()..];
                            break;
                        }
                    }
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
        let mut neighbour_counts = HashMap::new();
        let mut to_add = Vec::new();
        let directions: Vec<_> = directions().values().copied().collect();
        for _ in 1..=100 {
            for tile in grid.iter() {
                for neighbour in directions
                    .iter()
                    .map(|dir| (tile.0 + dir.0, tile.1 + dir.1, tile.2 + dir.2))
                {
                    *neighbour_counts.entry(neighbour).or_insert(0) += 1;
                }
            }
            to_add.extend(
                neighbour_counts
                    .iter()
                    .filter(|pair| !grid.contains(pair.0) && *pair.1 == 2)
                    .map(|pair| pair.0),
            );
            grid.retain(|tile| [1, 2].contains(neighbour_counts.get(tile).unwrap_or(&0)));
            grid.extend(&to_add);
            neighbour_counts.clear();
            to_add.clear();
        }
        grid.len()
    }
}
