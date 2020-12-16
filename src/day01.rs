use crate::day::Day;

pub struct Day01 {}

impl<'a> Day<'a> for Day01 {
    type Input1 = Vec<u32>;
    type Input2 = Vec<u32>;
    type Output1 = u32;
    type Output2 = u32;

    const INDEX: usize = 1;

    fn parse(input: &'a str) -> Self::Input1 {
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.parse::<u32>()
                    .expect("expected unsigned int as input line")
            })
            .collect()
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let mut answer = None;
        for (idx1, num1) in input.iter().enumerate() {
            for num2 in input[0..=idx1].iter() {
                if num1 + num2 == 2020 {
                    answer = Some(num1 * num2);
                    break;
                }
            }
        }
        (input, answer.unwrap())
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        for (idx1, num1) in input.iter().enumerate() {
            for (idx2, num2) in input[0..=idx1].iter().enumerate() {
                if num1 + num2 > 2020 {
                    continue;
                }
                for num3 in input[0..=idx2].iter() {
                    if num1 + num2 + num3 == 2020 {
                        return num1 * num2 * num3;
                    }
                }
            }
        }
        panic!("no solution found");
    }
}
