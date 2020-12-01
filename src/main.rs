use std::{
    fs::File,
    io::{self, Read},
    time::Instant,
};
use util::{iter_unordered_pairs, iter_unordered_triples};

mod util;

fn parse(input: String) -> Vec<u32> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.parse::<u32>()
                .expect("expected unsigned int as input line")
        })
        .collect()
}

fn part1(input: &[u32]) -> u32 {
    for (number1, number2) in iter_unordered_pairs(input) {
        if number1 + number2 == 2020 {
            return number1 * number2;
        }
    }
    panic!("no solution found");
}

fn part2(input: &[u32]) -> u32 {
    for [number1, number2, number3] in iter_unordered_triples(input) {
        if number1 + number2 + number3 == 2020 {
            return number1 * number2 * number3;
        }
    }
    panic!("no solution found");
}

fn main() -> io::Result<()> {
    let mut input_file = File::open("res/day1.in")?;
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string)?;

    let parsed_input = parse(input_string);
    let time1 = Instant::now();
    let part1_answer = part1(&parsed_input);
    let time2 = Instant::now();
    let part2_answer = part2(&parsed_input);
    let time3 = Instant::now();

    println!(
        "part1: {} (elapsed {}ms)",
        part1_answer,
        (time2 - time1).as_millis()
    );
    println!(
        "part2: {} (elapsed {}ms)",
        part2_answer,
        (time3 - time2).as_millis()
    );

    Ok(())
}
