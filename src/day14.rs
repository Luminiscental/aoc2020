use crate::day::Day;
use std::{collections::HashMap, str::FromStr};

#[derive(Clone)]
pub struct Bitmask {
    bits: [Option<usize>; 36],
}

impl Bitmask {
    fn apply(&self, mut value: usize) -> usize {
        for (idx, bit) in self.bits.iter().enumerate() {
            if let Some(bit) = bit {
                value &= !(1 << idx);
                value |= bit << idx;
            }
        }
        value
    }

    fn apply_v2(&self, value: usize) -> Bitmask {
        let mut result = Bitmask { bits: [None; 36] };
        for (idx, bit) in self.bits.iter().enumerate() {
            match bit {
                Some(0) => result.bits[idx] = Some((value & (1 << idx)) >> idx),
                &bit => result.bits[idx] = bit,
            }
        }
        result
    }

    fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(usize),
    {
        let mut values = vec![0];
        for (idx, bit) in self.bits.iter().enumerate() {
            if bit.is_none() {
                let val = 1 << idx;
                values.extend::<Vec<_>>(values.iter().map(|value| value + val).collect());
            } else {
                let val = bit.unwrap() << idx;
                for value in values.iter_mut() {
                    *value += val;
                }
            }
        }
        for value in values.into_iter() {
            f(value);
        }
    }
}

impl FromStr for Bitmask {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, ()> {
        let elems: Vec<_> = string
            .chars()
            .map(|c| match c {
                'X' => Ok(None),
                '0' => Ok(Some(0)),
                '1' => Ok(Some(1)),
                _ => Err(()),
            })
            .collect();
        let mut result = Bitmask { bits: [None; 36] };
        for (idx, elem) in elems.into_iter().enumerate() {
            result.bits[35 - idx] = elem?;
        }
        Ok(result)
    }
}

pub enum Operation {
    SetMemory(usize, usize),
    SetMask(Bitmask),
}

impl Operation {
    fn perform(&self, mask: &mut Bitmask, memory: &mut HashMap<usize, usize>) {
        match self {
            Self::SetMemory(idx, val) => {
                memory.insert(*idx, mask.apply(*val));
            }
            Self::SetMask(new_mask) => *mask = new_mask.clone(),
        }
    }

    fn perform_v2(&self, mask: &mut Bitmask, memory: &mut HashMap<usize, usize>) {
        match self {
            Self::SetMemory(idx, val) => {
                let val = *val;
                mask.apply_v2(*idx).for_each(|idx| {
                    memory.insert(idx, val);
                });
            }
            Self::SetMask(new_mask) => *mask = new_mask.clone(),
        }
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, ()> {
        let mut lhs_rhs = string.split(" = ");
        let lhs = lhs_rhs.next().unwrap();
        let rhs = lhs_rhs.next().unwrap();
        match lhs {
            "mask" => Ok(Operation::SetMask(rhs.parse::<Bitmask>()?)),
            _mem => {
                let lhs_len = lhs.len();
                let idx = lhs[4..lhs_len - 1].parse::<usize>().map_err(|_| ())?;
                let val = rhs.parse::<usize>().map_err(|_| ())?;
                Ok(Operation::SetMemory(idx, val))
            }
        }
    }
}

pub struct Day14 {}

impl<'a> Day<'a> for Day14 {
    type Input1 = Vec<Operation>;
    type Input2 = Vec<Operation>;
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 14;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        raw_input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<Operation>().unwrap())
            .collect()
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let mut mask = Bitmask { bits: [None; 36] };
        let mut memory = HashMap::new();
        for operation in input.iter() {
            operation.perform(&mut mask, &mut memory);
        }
        (input, memory.values().sum())
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        let mut mask = Bitmask { bits: [None; 36] };
        let mut memory = HashMap::new();
        for operation in input.iter() {
            operation.perform_v2(&mut mask, &mut memory);
        }
        memory.values().sum()
    }
}
