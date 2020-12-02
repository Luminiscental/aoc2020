pub fn parse(input: &str) -> Vec<u32> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.parse::<u32>()
                .expect("expected unsigned int as input line")
        })
        .collect()
}

pub fn part1(input: &[u32]) -> u32 {
    for (idx1, num1) in input.iter().enumerate() {
        for num2 in input[0..=idx1].iter() {
            if num1 + num2 == 2020 {
                return num1 * num2;
            }
        }
    }
    panic!("no solution found");
}

pub fn part2(input: &[u32]) -> u32 {
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
