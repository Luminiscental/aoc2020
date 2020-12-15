use std::{
    fs::File,
    io::{self, Read},
};

mod util;
mod day;
mod day15;

use day::Day;
use day15::Day15;

fn main() -> io::Result<()> {
    type CurrentDay = Day15;

    let mut input_file = File::open(format!("res/day{}.in", CurrentDay::INDEX))?;
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string)?;
    CurrentDay::solve_and_print(&input_string);

    Ok(())
}
