use std::{
    fs::File,
    io::{self, Read},
};

mod day;
mod day7;

use day::Day;
use day7::Day7;

fn main() -> io::Result<()> {
    type CurrentDay = Day7;

    let mut input_file = File::open(format!("res/day{}.in", CurrentDay::INDEX))?;
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string)?;
    CurrentDay::solve_and_print(&input_string);

    Ok(())
}
