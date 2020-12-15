use crate::day::Day;

fn build_sequence(start_numbers: &[usize], last_turn: usize) -> usize {
    let mut seen = vec![0; last_turn];
    let mut last_number = 0;
    for (idx, number) in start_numbers.iter().enumerate() {
        seen[*number] = idx + 1;
        last_number = *number;
    }
    let mut turn = start_numbers.len() + 1;
    while turn <= last_turn {
        let next_number = match seen[last_number] {
            0 => 0,
            early_turn => turn - 1 - early_turn,
        };
        seen[last_number] = turn - 1;
        last_number = next_number;
        turn += 1;
    }
    last_number
}

pub struct Day15 {}

impl<'a> Day<'a> for Day15 {
    type Input1 = Vec<usize>;
    type Input2 = Vec<usize>;
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 15;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        raw_input
            .split(',')
            .map(|n| n.trim().parse::<usize>().unwrap())
            .collect()
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let answer = build_sequence(&input, 2020);
        (input, answer)
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        build_sequence(&input, 30000000)
    }
}
