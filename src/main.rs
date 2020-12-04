use std::{
    fs::File,
    io::{self, Read},
};

mod day;
mod day4;

use day::Day;
use day4::Day4;

fn main() -> io::Result<()> {
    type CurrentDay = Day4;

    let mut input_file = File::open(format!("res/day{}.in", CurrentDay::INDEX))?;
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string)?;
    CurrentDay::solve_and_print(&input_string);

    Ok(())
}
