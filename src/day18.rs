use crate::{day::Day, map};
use std::collections::HashMap;

pub enum Expression {
    Binary(u8, Box<Expression>, Box<Expression>),
    Term(usize),
}

impl Expression {
    fn parse_whole(string: &str, precedences: &HashMap<u8, usize>) -> Expression {
        let (expr, rest) = Expression::parse_precedence(string.as_bytes(), 0, precedences);
        assert!(rest.is_empty());
        expr
    }

    fn parse_prefix<'a>(
        string: &'a [u8],
        precedences: &HashMap<u8, usize>,
    ) -> (Expression, &'a [u8]) {
        match string[0] {
            b' ' => Expression::parse_prefix(&string[1..], precedences),
            b'(' => {
                let (group, rest) = Expression::parse_precedence(&string[1..], 0, precedences);
                assert!(rest[0] == b')');
                (group, &rest[1..])
            }
            digit if digit.is_ascii_digit() => {
                (Expression::Term((digit - b'0') as usize), &string[1..])
            }
            c => panic!("unexpected character \"{}\"", c),
        }
    }

    fn parse_precedence<'a>(
        string: &'a [u8],
        precedence: usize,
        precedences: &HashMap<u8, usize>,
    ) -> (Expression, &'a [u8]) {
        let (mut expr, mut string) = Expression::parse_prefix(string, precedences);
        while !string.is_empty() {
            match string[0] {
                b' ' => string = &string[1..],
                op if precedences.contains_key(&op) && precedence <= precedences[&op] => {
                    let (rhs, rest) = Expression::parse_precedence(
                        &string[1..],
                        precedences[&op] + 1,
                        precedences,
                    );
                    expr = Expression::Binary(op, Box::new(expr), Box::new(rhs));
                    string = rest;
                }
                _ => break,
            }
        }
        (expr, string)
    }

    fn evaluate(&self) -> usize {
        match self {
            Expression::Binary(b'*', lhs, rhs) => lhs.evaluate() * rhs.evaluate(),
            Expression::Binary(b'+', lhs, rhs) => lhs.evaluate() + rhs.evaluate(),
            Expression::Binary(op, _, _) => panic!("unexpected operation: \"{}\"", op),
            Expression::Term(n) => *n,
        }
    }
}

pub struct Day18 {}

impl<'a> Day<'a> for Day18 {
    type Input1 = Vec<&'a str>;
    type Input2 = Vec<&'a str>;
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 18;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        raw_input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.trim())
            .collect()
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let precs = map! {b'*' => 1, b'+' => 1};
        let sum = input
            .iter()
            .map(|line| Expression::parse_whole(line, &precs).evaluate())
            .sum();
        (input, sum)
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        let precs = map! {b'*' => 1, b'+' => 2};
        input
            .iter()
            .map(|line| Expression::parse_whole(line, &precs).evaluate())
            .sum()
    }
}
