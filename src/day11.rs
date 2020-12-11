use crate::day::Day;

const DIRECTIONS: &[(i32, i32)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn in_range(i: i32, j: i32, ilen: usize, jlen: usize) -> bool {
    i >= 0 && i < ilen as i32 && j >= 0 && j < jlen as i32
}

fn increment_seats(
    seats: &mut Vec<Vec<char>>,
    neighbour_counts: &mut Vec<Vec<usize>>,
    crowded_threshold: usize,
    extended_sight: bool,
) -> bool {
    for (i, neighbour_row) in neighbour_counts.iter_mut().enumerate() {
        for (j, neighbour_count) in neighbour_row.iter_mut().enumerate() {
            for (delta_i, delta_j) in DIRECTIONS.iter() {
                let mut check_i = i as i32 + delta_i;
                let mut check_j = j as i32 + delta_j;
                while in_range(check_i, check_j, seats.len(), seats[0].len()) {
                    match seats[check_i as usize][check_j as usize] {
                        '#' => {
                            *neighbour_count += 1;
                            break;
                        }
                        '.' if extended_sight => {
                            check_i += delta_i;
                            check_j += delta_j;
                            continue;
                        }
                        '.' if !extended_sight => break,
                        'L' => break,
                        c => panic!("unrecognized character \"{}\"", c),
                    }
                }
            }
        }
    }

    let mut changed = false;
    for (seat_row, neighbour_row) in seats.iter_mut().zip(neighbour_counts.iter_mut()) {
        for (seat, neighbour_count) in seat_row.iter_mut().zip(neighbour_row.iter_mut()) {
            match seat {
                '#' if *neighbour_count >= crowded_threshold => {
                    *seat = 'L';
                    changed = true;
                }
                'L' if *neighbour_count == 0 => {
                    *seat = '#';
                    changed = true;
                }
                _ => (),
            }
            *neighbour_count = 0;
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
        let mut neighbour_counts = seats
            .iter()
            .map(|row| row.iter().map(|_| 0).collect())
            .collect();
        while increment_seats(&mut seats, &mut neighbour_counts, 4, false) {}
        let vacant = seats
            .iter()
            .map(|row| row.iter().filter(|&&c| c == '#').count())
            .sum();
        (input, vacant)
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        let mut seats = input;
        let mut neighbour_counts = seats
            .iter()
            .map(|row| row.iter().map(|_| 0).collect())
            .collect();
        while increment_seats(&mut seats, &mut neighbour_counts, 5, true) {}
        seats
            .iter()
            .map(|row| row.iter().filter(|&&c| c == '#').count())
            .sum()
    }
}
