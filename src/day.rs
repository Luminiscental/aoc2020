pub trait Day<'a> {
    type Input;
    type Output1;
    type Output2;

    fn parse(raw_input: &'a str) -> Self::Input;
    fn solve_part1(input: &Self::Input) -> Self::Output1;
    fn solve_part2(input: &Self::Input) -> Self::Output2;
}
