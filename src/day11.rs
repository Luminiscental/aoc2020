use crate::day::Day;

fn increment_seats1(seats: &mut Vec<Vec<char>>, scratchpad: &mut Vec<Vec<usize>>) -> bool {
    for j in 0..seats[0].len() {
        for i in 0..seats.len() {
            match seats[i][j] {
                '#' => {
                    if i < seats.len() - 1 && j < seats[0].len() - 1 {
                        scratchpad[i + 1][j + 1] += 1;
                    }
                    if j < seats[0].len() - 1 {
                        scratchpad[i][j + 1] += 1;
                    }
                    if i < seats.len() - 1 {
                        scratchpad[i + 1][j] += 1;
                    }
                    if i > 0 {
                        scratchpad[i - 1][j] += 1;
                    }
                    if i > 0 && j < seats[0].len() - 1 {
                        scratchpad[i - 1][j + 1] += 1;
                    }
                    if i > 0 && j > 0 {
                        scratchpad[i - 1][j - 1] += 1;
                    }
                    if j > 0 {
                        scratchpad[i][j - 1] += 1;
                    }
                    if i < seats.len() - 1 && j > 0 {
                        scratchpad[i + 1][j - 1] += 1;
                    }
                }
                'L' | '.' => (),
                _ => panic!("unrecognized tile"),
            }
        }
    }

    let mut changed = false;
    for j in 0..seats[0].len() {
        for i in 0..seats.len() {
            match seats[i][j] {
                '#' => {
                    if scratchpad[i][j] >= 4 {
                        seats[i][j] = 'L';
                        changed = true;
                    }
                }
                'L' => {
                    if scratchpad[i][j] == 0 {
                        seats[i][j] = '#';
                        changed = true;
                    }
                }
                '.' => (),
                _ => unreachable!(),
            }
            scratchpad[i][j] = 0;
        }
    }
    changed
}

fn increment_seats2(seats: &mut Vec<Vec<char>>, scratchpad: &mut Vec<Vec<usize>>) -> bool {
    // count left-right
    for i in 0..seats.len() {
        let mut last_j = None;
        let mut last_empty = false;
        for j in 0..seats[0].len() {
            match seats[i][j] {
                '#' => {
                    if let Some(last_j) = last_j {
                        scratchpad[i][last_j] += 1;
                        if !last_empty {
                            scratchpad[i][j] += 1;
                        }
                    }
                    last_j = Some(j);
                    last_empty = false;
                }
                'L' => {
                    if last_j.is_some() && !last_empty {
                        scratchpad[i][j] += 1;
                    }
                    last_j = Some(j);
                    last_empty = true;
                }
                '.' => (),
                _ => unreachable!(),
            }
        }
    }

    // count up-down
    for j in 0..seats[0].len() {
        let mut last_i: Option<usize> = None;
        let mut last_empty = false;
        for i in 0..seats.len() {
            match seats[i][j] {
                '#' => {
                    if let Some(last_i) = last_i {
                        scratchpad[last_i][j] += 1;
                        if !last_empty {
                            scratchpad[i][j] += 1;
                        }
                    }
                    last_i = Some(i);
                    last_empty = false;
                }
                'L' => {
                    if last_i.is_some() && !last_empty {
                        scratchpad[i][j] += 1;
                    }
                    last_i = Some(i);
                    last_empty = true;
                }
                '.' => (),
                _ => unreachable!(),
            }
        }
    }

    // count positive diagonals
    for n in 0..(seats[0].len() + seats.len() - 1) {
        let i0 = if n < seats[0].len() {
            0
        } else {
            n + 1 - seats[0].len()
        };
        let j0 = if n < seats[0].len() {
            seats[0].len() - 1 - n
        } else {
            0
        };
        let length = usize::min(seats.len() - i0, seats[0].len() - j0);
        let mut last_d: Option<usize> = None;
        let mut last_empty = false;
        for d in 0..length {
            match seats[i0 + d][j0 + d] {
                '#' => {
                    if let Some(last_d) = last_d {
                        scratchpad[i0 + last_d][j0 + last_d] += 1;
                        if !last_empty {
                            scratchpad[i0 + d][j0 + d] += 1;
                        }
                    }
                    last_d = Some(d);
                    last_empty = false;
                }
                'L' => {
                    if last_d.is_some() && !last_empty {
                        scratchpad[i0 + d][j0 + d] += 1;
                    }
                    last_d = Some(d);
                    last_empty = true;
                }
                '.' => (),
                _ => unreachable!(),
            }
        }
    }

    // count negative diagonals
    for n in 0..(seats[0].len() + seats.len() - 1) {
        let i0 = if n < seats[0].len() {
            0
        } else {
            n + 1 - seats[0].len()
        };
        let j0 = if n < seats[0].len() {
            n
        } else {
            seats[0].len() - 1
        };
        let length = usize::min(seats.len() - i0, j0 + 1);
        let mut last_d: Option<usize> = None;
        let mut last_empty = false;
        for d in 0..length {
            match seats[i0 + d][j0 - d] {
                '#' => {
                    if let Some(last_d) = last_d {
                        scratchpad[i0 + last_d][j0 - last_d] += 1;
                        if !last_empty {
                            scratchpad[i0 + d][j0 - d] += 1;
                        }
                    }
                    last_d = Some(d);
                    last_empty = false;
                }
                'L' => {
                    if last_d.is_some() && !last_empty {
                        scratchpad[i0 + d][j0 - d] += 1;
                    }
                    last_d = Some(d);
                    last_empty = true;
                }
                '.' => (),
                _ => unreachable!(),
            }
        }
    }

    let mut changed = false;
    for j in 0..seats[0].len() {
        for i in 0..seats.len() {
            match seats[i][j] {
                '#' => {
                    if scratchpad[i][j] >= 5 {
                        seats[i][j] = 'L';
                        changed = true;
                    }
                }
                'L' => {
                    if scratchpad[i][j] == 0 {
                        seats[i][j] = '#';
                        changed = true;
                    }
                }
                '.' => (),
                _ => unreachable!(),
            }
            scratchpad[i][j] = 0;
        }
    }
    changed
}

pub struct Day11 {}

impl<'a> Day<'a> for Day11 {
    type Input1 = Vec<Vec<char>>;
    type Input2 = Vec<Vec<char>>;
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 11;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        raw_input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect()
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let mut seats = input.clone();
        let mut scratchpad = seats
            .iter()
            .map(|row| row.iter().map(|_| 0).collect())
            .collect();
        while increment_seats1(&mut seats, &mut scratchpad) {}
        let vacant = seats
            .iter()
            .map(|row| row.iter().filter(|&&c| c == '#').count())
            .sum();
        (input, vacant)
    }

    fn solve_part2(mut input: Self::Input2) -> Self::Output2 {
        let mut scratchpad = input
            .iter()
            .map(|row| row.iter().map(|_| 0).collect())
            .collect();
        while increment_seats2(&mut input, &mut scratchpad) {}
        input
            .iter()
            .map(|row| row.iter().filter(|&&c| c == '#').count())
            .sum()
    }
}
