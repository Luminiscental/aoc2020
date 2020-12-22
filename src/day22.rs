use crate::day::Day;
use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
};

enum Player {
    One,
    Two,
}

fn score(deck: VecDeque<usize>) -> usize {
    deck.iter().rev().zip(1..).map(|pair| pair.0 * pair.1).sum()
}

fn play_recursive(
    mut player1: VecDeque<usize>,
    mut player2: VecDeque<usize>,
    allow_recursion: bool,
) -> (Player, usize) {
    let mut states = HashSet::new();
    while !player1.is_empty() && !player2.is_empty() {
        if allow_recursion && !states.insert((player1.clone(), player2.clone())) {
            player2.clear();
            break;
        }
        let (play1, play2) = (player1.pop_front().unwrap(), player2.pop_front().unwrap());
        let winner = if !allow_recursion || play1 > player1.len() || play2 > player2.len() {
            match play1.cmp(&play2) {
                Ordering::Greater => Player::One,
                Ordering::Less => Player::Two,
                Ordering::Equal => panic!("tie"),
            }
        } else {
            play_recursive(
                player1.iter().take(play1).copied().collect(),
                player2.iter().take(play2).copied().collect(),
                allow_recursion,
            )
            .0
        };
        match winner {
            Player::One => {
                player1.push_back(play1);
                player1.push_back(play2);
            }
            Player::Two => {
                player2.push_back(play2);
                player2.push_back(play1);
            }
        }
    }
    if player2.is_empty() {
        (Player::One, score(player1))
    } else {
        (Player::Two, score(player2))
    }
}

pub struct Day22 {}

impl<'a> Day<'a> for Day22 {
    type Input1 = (VecDeque<usize>, VecDeque<usize>);
    type Input2 = (VecDeque<usize>, VecDeque<usize>);
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 22;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        let lines: Vec<_> = raw_input.lines().collect();
        let mut chunks = lines.split(|line| line.is_empty());
        (
            chunks.next().unwrap()[1..]
                .iter()
                .map(|line| line.parse().unwrap())
                .collect(),
            chunks.next().unwrap()[1..]
                .iter()
                .map(|line| line.parse().unwrap())
                .collect(),
        )
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let winning_score = play_recursive(input.0.clone(), input.1.clone(), false).1;
        (input, winning_score)
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        play_recursive(input.0, input.1, true).1
    }
}
