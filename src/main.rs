use std::{
    fs::File,
    io::{self, Read},
};

mod day;
mod day13;

use day::Day;
use day13::Day13;

fn main() -> io::Result<()> {
    type CurrentDay = Day13;

    let mut input_file = File::open(format!("res/day{}.in", CurrentDay::INDEX))?;
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string)?;
    CurrentDay::solve_and_print(&input_string);

    Ok(())
}
