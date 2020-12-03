use std::{
    fs::File,
    io::{self, Read},
    time::Instant,
};

mod day;
mod day3;

use day::Day;
use day3::Day3;

fn main() -> io::Result<()> {
    let mut input_file = File::open("res/day3.in")?;
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string)?;

    let parsed_input = Day3::parse(&input_string);
    let time1 = Instant::now();
    let part1_answer = Day3::solve_part1(&parsed_input);
    let time2 = Instant::now();
    let part2_answer = Day3::solve_part2(&parsed_input);
    let time3 = Instant::now();

    println!();
    println!("day3:");
    println!(
        "  part1: {} (elapsed {}ms)",
        part1_answer,
        1000.0 * (time2 - time1).as_secs_f32()
    );
    println!(
        "  part2: {} (elapsed {}ms)",
        part2_answer,
        1000.0 * (time3 - time2).as_secs_f32()
    );

    Ok(())
}
