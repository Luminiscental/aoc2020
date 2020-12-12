use crate::day::Day;

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

fn final_ship_distance<'a>(
    actions: &[(&'a str, i32)],
    mut waypoint_x: i32,
    mut waypoint_y: i32,
    move_waypoint: bool,
) -> i32 {
    let (mut offset_x, mut offset_y) = (0, 0);
    for (dir, dist) in actions.iter() {
        match *dir {
            "N" if move_waypoint => waypoint_y += dist,
            "E" if move_waypoint => waypoint_x += dist,
            "S" if move_waypoint => waypoint_y -= dist,
            "W" if move_waypoint => waypoint_x -= dist,

            "N" => offset_y += dist,
            "E" => offset_x += dist,
            "S" => offset_y -= dist,
            "W" => offset_x -= dist,

            "F" => {
                offset_x += waypoint_x * dist;
                offset_y += waypoint_y * dist;
            }
            "L" => rotate(&mut waypoint_x, &mut waypoint_y, *dist),
            "R" => rotate(&mut waypoint_x, &mut waypoint_y, 360 - dist),
            _ => unreachable!(),
        }
    }
    offset_x.abs() + offset_y.abs()
}

pub struct Day12 {}

impl<'a> Day<'a> for Day12 {
    type Input1 = Vec<(&'a str, i32)>;
    type Input2 = Vec<(&'a str, i32)>;
    type Output1 = i32;
    type Output2 = i32;

    const INDEX: usize = 12;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        raw_input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| (&line[0..1], line[1..].parse::<i32>().unwrap()))
            .collect()
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let dist = final_ship_distance(&input, 1, 0, false);
        (input, dist)
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        final_ship_distance(&input, 10, 1, true)
    }
}
