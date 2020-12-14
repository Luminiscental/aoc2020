use crate::day::Day;
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub struct Bitmask<'a> {
    bits: &'a str,
}

impl<'a> Bitmask<'a> {
    fn apply(&self, mut value: usize) -> usize {
        for (idx, bit) in self.bits.chars().rev().enumerate() {
            match bit {
                '0' => value &= !(1 << idx),
                '1' => value |= 1 << idx,
                _ => continue,
            }
        }
        value
    }

    fn apply_v2(&self, value: usize) -> Vec<usize> {
        let mut values = vec![0];
        for (idx, bit) in self.bits.chars().rev().enumerate() {
            if bit == '1' || (bit == '0' && (value & (1 << idx) != 0)) {
                values.iter_mut().for_each(|v| *v += 1 << idx);
            } else if bit == 'X' {
                for i in 0..values.len() {
                    values.push(values[i] + (1 << idx));
                }
            }
        }
        values
    }
}

pub enum Operation<'a> {
    SetMemory(usize, usize),
    SetMask(Bitmask<'a>),
}

impl<'a> Operation<'a> {
    fn perform(&self, mask: &mut Bitmask<'a>, memory: &mut HashMap<usize, usize>) {
        match self {
            Self::SetMemory(idx, val) => {
                memory.insert(*idx, mask.apply(*val));
            }
            Self::SetMask(new_mask) => *mask = *new_mask,
        }
    }

    fn perform_v2(&self, mask: &mut Bitmask<'a>, memory: &mut HashMap<usize, usize>) {
        match self {
            Self::SetMemory(idx, val) => {
                let val = *val;
                mask.apply_v2(*idx).into_iter().for_each(|idx| {
                    memory.insert(idx, val);
                });
            }
            Self::SetMask(new_mask) => *mask = *new_mask,
        }
    }
}

pub struct Day14 {}

impl<'a> Day<'a> for Day14 {
    type Input1 = Vec<Operation<'a>>;
    type Input2 = Vec<Operation<'a>>;
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 14;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        raw_input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut parts = line.split(" = ");
                let lhs = parts.next().unwrap();
                let rhs = parts.next().unwrap();
                match lhs {
                    "mask" => Operation::SetMask(Bitmask { bits: rhs }),
                    _mem => Operation::SetMemory(
                        lhs[4..lhs.len() - 1].parse::<usize>().unwrap(),
                        rhs.parse::<usize>().unwrap(),
                    ),
                }
            })
            .collect()
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let mut mask = Bitmask { bits: "" };
        let mut memory = HashMap::new();
        for operation in input.iter() {
            operation.perform(&mut mask, &mut memory);
        }
        (input, memory.values().sum())
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        let mut mask = Bitmask { bits: "" };
        let mut memory = HashMap::new();
        for operation in input.iter() {
            operation.perform_v2(&mut mask, &mut memory);
        }
        memory.values().sum()
    }
}
