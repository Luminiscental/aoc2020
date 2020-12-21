use std::{
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

macro_rules! solve {
    ($day:ident) => {
        let mut input_file = File::open(format!("res/day{}.in", $day::INDEX))?;
        let mut input_string = String::new();
        input_file.read_to_string(&mut input_string)?;
        $day::solve_and_print(&input_string);
    };
}

fn main() -> io::Result<()> {
    solve!(Day01);
    solve!(Day02);
    solve!(Day03);
    solve!(Day04);
    solve!(Day05);
    solve!(Day06);
    solve!(Day07);
    solve!(Day08);
    solve!(Day09);
    solve!(Day10);
    solve!(Day11);
    solve!(Day12);
    solve!(Day13);
    solve!(Day14);
    solve!(Day15);
    solve!(Day16);
    solve!(Day17);
    solve!(Day18);
    solve!(Day19);
    solve!(Day20);
    solve!(Day21);
    Ok(())
}
