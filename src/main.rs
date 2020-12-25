use std::{
    env,
    fs::File,
    io::{self, Read},
};

mod day;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod util;

use day::Day;
use day01::Day01;
use day02::Day02;
use day03::Day03;
use day04::Day04;
use day05::Day05;
use day06::Day06;
use day07::Day07;
use day08::Day08;
use day09::Day09;
use day10::Day10;
use day11::Day11;
use day12::Day12;
use day13::Day13;
use day14::Day14;
use day15::Day15;
use day16::Day16;
use day17::Day17;
use day18::Day18;
use day19::Day19;
use day20::Day20;
use day21::Day21;
use day22::Day22;
use day23::Day23;
use day24::Day24;
use day25::Day25;

macro_rules! solve {
    ($day:ident) => {{
        let mut input_file = File::open(format!("res/day{}.in", $day::INDEX))?;
        let mut input_string = String::new();
        input_file.read_to_string(&mut input_string)?;
        $day::solve_and_print(&input_string);
    }};
}

fn main() -> io::Result<()> {
    match env::args().skip(1).next() {
        None => eprintln!("expected an argument to choose which day to solve"),
        Some(day) => match day.parse::<usize>() {
            Err(err) => eprintln!("unrecognized day \"{}\": {}", day, err),
            Ok(1) => solve!(Day01),
            Ok(2) => solve!(Day02),
            Ok(3) => solve!(Day03),
            Ok(4) => solve!(Day04),
            Ok(5) => solve!(Day05),
            Ok(6) => solve!(Day06),
            Ok(7) => solve!(Day07),
            Ok(8) => solve!(Day08),
            Ok(9) => solve!(Day09),
            Ok(10) => solve!(Day10),
            Ok(11) => solve!(Day11),
            Ok(12) => solve!(Day12),
            Ok(13) => solve!(Day13),
            Ok(14) => solve!(Day14),
            Ok(15) => solve!(Day15),
            Ok(16) => solve!(Day16),
            Ok(17) => solve!(Day17),
            Ok(18) => solve!(Day18),
            Ok(19) => solve!(Day19),
            Ok(20) => solve!(Day20),
            Ok(21) => solve!(Day21),
            Ok(22) => solve!(Day22),
            Ok(23) => solve!(Day23),
            Ok(24) => solve!(Day24),
            Ok(25) => solve!(Day25),
            _ => todo!(),
        },
    }
    Ok(())
}
