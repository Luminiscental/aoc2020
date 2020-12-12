use crate::day::Day;
use std::str::FromStr;

pub enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "N" => Ok(Self::North),
            "E" => Ok(Self::East),
            "S" => Ok(Self::South),
            "W" => Ok(Self::West),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            "F" => Ok(Self::Forward),
            _ => Err(()),
        }
    }
}

fn rotate(point_x: &mut i32, point_y: &mut i32, angle: i32) {
    let (cos, sin) = match angle {
        90 => (0, 1),
        180 => (-1, 0),
        270 => (0, -1),
        _ => unreachable!(),
    };
    let new_facing_x = cos * *point_x - sin * *point_y;
    let new_facing_y = sin * *point_x + cos * *point_y;
    *point_x = new_facing_x;
    *point_y = new_facing_y;
}

pub struct Day12 {}

impl<'a> Day<'a> for Day12 {
    type Input1 = Vec<(Direction, i32)>;
    type Input2 = Vec<(Direction, i32)>;
    type Output1 = i32;
    type Output2 = i32;

    const INDEX: usize = 12;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        raw_input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                (
                    line[0..1].parse::<Direction>().unwrap(),
                    line[1..].parse::<i32>().unwrap(),
                )
            })
            .collect()
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let (mut facing_x, mut facing_y) = (1, 0);
        let (mut offset_x, mut offset_y) = (0, 0);
        for (dir, dist) in input.iter() {
            match dir {
                Direction::North => offset_y += dist,
                Direction::East => offset_x += dist,
                Direction::South => offset_y -= dist,
                Direction::West => offset_x -= dist,
                Direction::Forward => {
                    offset_x += facing_x * dist;
                    offset_y += facing_y * dist;
                }
                Direction::Left => {
                    rotate(&mut facing_x, &mut facing_y, *dist);
                }
                Direction::Right => {
                    rotate(&mut facing_x, &mut facing_y, 360 - dist);
                }
            }
        }
        (input, offset_x.abs() + offset_y.abs())
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        let (mut facing_x, mut facing_y) = (1, 0);
        let (mut waypoint_x, mut waypoint_y) = (10, 1);
        let (mut offset_x, mut offset_y) = (0, 0);
        for (dir, dist) in input.iter() {
            match dir {
                Direction::North => waypoint_y += dist,
                Direction::East => waypoint_x += dist,
                Direction::South => waypoint_y -= dist,
                Direction::West => waypoint_x -= dist,
                Direction::Forward => {
                    offset_x += waypoint_x * dist;
                    offset_y += waypoint_y * dist;
                }
                Direction::Left => {
                    rotate(&mut facing_x, &mut facing_y, *dist);
                    rotate(&mut waypoint_x, &mut waypoint_y, *dist);
                }
                Direction::Right => {
                    rotate(&mut facing_x, &mut facing_y, 360 - dist);
                    rotate(&mut waypoint_x, &mut waypoint_y, 360 - dist);
                }
            }
        }
        offset_x.abs() + offset_y.abs()
    }
}
