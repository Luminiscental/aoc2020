use std::{
    fs::File,
    io::{self, Read},
};

mod day;
mod day5;

use day::Day;
use day5::Day5;

fn main() -> io::Result<()> {
    type CurrentDay = Day5;

    let mut input_file = File::open(format!("res/day{}.in", CurrentDay::INDEX))?;
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string)?;
    CurrentDay::solve_and_print(&input_string);

    Ok(())
}
