use std::{
    cmp::Ord,
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub trait Ignore {
    fn ignore(self);
}

impl<T> Ignore for T {
    fn ignore(self) {}
}

#[macro_export]
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

#[macro_export]
macro_rules! set(
    { $($value:expr),+ } => {
        {
            let mut m = ::std::collections::HashSet::new();
            $(
                m.insert($value);
            )+
            m
        }
     };
);

pub fn exp_modular(base: usize, exp: usize, modulo: usize) -> usize {
    match exp {
        0 => 1,
        1 => base % modulo,
        exp => {
            let root = exp_modular(base, exp / 2, modulo);
            if exp % 2 == 0 {
                (root * root) % modulo
            } else {
                ((base % modulo) * ((root * root) % modulo)) % modulo
            }
        }
    }
}

pub fn prime_divide_modular(num: usize, denom: usize, modulo: usize) -> usize {
    ((num % modulo) * exp_modular(denom, modulo - 2, modulo)) % modulo
}

pub fn range_2d<T: Ord>(range: impl Iterator<Item = (T, T)>) -> ((T, T), (T, T))
where
    T: Copy,
{
    let mut x_min = None;
    let mut x_max = None;
    let mut y_min = None;
    let mut y_max = None;
    for (tx, ty) in range {
        if x_min.is_none() || x_min.unwrap() > tx {
            x_min = Some(tx);
        }
        if x_max.is_none() || x_max.unwrap() < tx {
            x_max = Some(tx);
        }
        if y_min.is_none() || y_min.unwrap() > ty {
            y_min = Some(ty);
        }
        if y_max.is_none() || y_max.unwrap() < ty {
            y_max = Some(ty);
        }
    }
    (
        (x_min.unwrap(), x_max.unwrap()),
        (y_min.unwrap(), y_max.unwrap()),
    )
}

pub trait Coord: Sized {
    fn for_neighbours<F: FnMut(Self)>(&self, f: F);
}

pub fn step_cellular_automata<T, BirthFn, DeathFn>(
    alive: &mut HashSet<T>,
    birth: BirthFn,
    death: DeathFn,
) where
    T: Coord + Eq + Hash + Copy,
    BirthFn: Fn(usize) -> bool,
    DeathFn: Fn(usize) -> bool,
{
    let mut neighbour_counts: HashMap<T, usize> = HashMap::new();
    for cell in alive.iter() {
        cell.for_neighbours(|neighbour| {
            *neighbour_counts.entry(neighbour).or_insert(0) += 1;
        });
    }
    let to_add: Vec<T> = neighbour_counts
        .iter()
        .filter(|pair| !alive.contains(pair.0))
        .filter(|pair| birth(*pair.1))
        .map(|pair| pair.0)
        .copied()
        .collect();
    alive.retain(|cell| !death(neighbour_counts.get(cell).copied().unwrap_or(0)));
    alive.extend(to_add);
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Coord3DWithDiagonals(pub i32, pub i32, pub i32);

impl Coord for Coord3DWithDiagonals {
    fn for_neighbours<F: FnMut(Self)>(&self, mut f: F) {
        for &neighbour in [
            Self(self.0 - 1, self.1 - 1, self.2 - 1),
            Self(self.0 - 1, self.1 - 1, self.2),
            Self(self.0 - 1, self.1 - 1, self.2 + 1),
            Self(self.0 - 1, self.1, self.2 - 1),
            Self(self.0 - 1, self.1, self.2),
            Self(self.0 - 1, self.1, self.2 + 1),
            Self(self.0 - 1, self.1 + 1, self.2 - 1),
            Self(self.0 - 1, self.1 + 1, self.2),
            Self(self.0 - 1, self.1 + 1, self.2 + 1),
            Self(self.0, self.1 - 1, self.2 - 1),
            Self(self.0, self.1 - 1, self.2),
            Self(self.0, self.1 - 1, self.2 + 1),
            Self(self.0, self.1, self.2 - 1),
            Self(self.0, self.1, self.2 + 1),
            Self(self.0, self.1 + 1, self.2 - 1),
            Self(self.0, self.1 + 1, self.2),
            Self(self.0, self.1 + 1, self.2 + 1),
            Self(self.0 + 1, self.1 - 1, self.2 - 1),
            Self(self.0 + 1, self.1 - 1, self.2),
            Self(self.0 + 1, self.1 - 1, self.2 + 1),
            Self(self.0 + 1, self.1, self.2 - 1),
            Self(self.0 + 1, self.1, self.2),
            Self(self.0 + 1, self.1, self.2 + 1),
            Self(self.0 + 1, self.1 + 1, self.2 - 1),
            Self(self.0 + 1, self.1 + 1, self.2),
            Self(self.0 + 1, self.1 + 1, self.2 + 1),
        ]
        .iter()
        {
            f(neighbour);
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Coord4DWithDiagonals(pub i32, pub i32, pub i32, pub i32);

impl Coord for Coord4DWithDiagonals {
    fn for_neighbours<F: FnMut(Self)>(&self, mut f: F) {
        for &neighbour in [
            Self(self.0 - 1, self.1 - 1, self.2 - 1, self.3 - 1),
            Self(self.0 - 1, self.1 - 1, self.2 - 1, self.3),
            Self(self.0 - 1, self.1 - 1, self.2 - 1, self.3 + 1),
            Self(self.0 - 1, self.1 - 1, self.2, self.3 - 1),
            Self(self.0 - 1, self.1 - 1, self.2, self.3),
            Self(self.0 - 1, self.1 - 1, self.2, self.3 + 1),
            Self(self.0 - 1, self.1 - 1, self.2 + 1, self.3 - 1),
            Self(self.0 - 1, self.1 - 1, self.2 + 1, self.3),
            Self(self.0 - 1, self.1 - 1, self.2 + 1, self.3 + 1),
            Self(self.0 - 1, self.1, self.2 - 1, self.3 - 1),
            Self(self.0 - 1, self.1, self.2 - 1, self.3),
            Self(self.0 - 1, self.1, self.2 - 1, self.3 + 1),
            Self(self.0 - 1, self.1, self.2, self.3 - 1),
            Self(self.0 - 1, self.1, self.2, self.3),
            Self(self.0 - 1, self.1, self.2, self.3 + 1),
            Self(self.0 - 1, self.1, self.2 + 1, self.3 - 1),
            Self(self.0 - 1, self.1, self.2 + 1, self.3),
            Self(self.0 - 1, self.1, self.2 + 1, self.3 + 1),
            Self(self.0 - 1, self.1 + 1, self.2 - 1, self.3 - 1),
            Self(self.0 - 1, self.1 + 1, self.2 - 1, self.3),
            Self(self.0 - 1, self.1 + 1, self.2 - 1, self.3 + 1),
            Self(self.0 - 1, self.1 + 1, self.2, self.3 - 1),
            Self(self.0 - 1, self.1 + 1, self.2, self.3),
            Self(self.0 - 1, self.1 + 1, self.2, self.3 + 1),
            Self(self.0 - 1, self.1 + 1, self.2 + 1, self.3 - 1),
            Self(self.0 - 1, self.1 + 1, self.2 + 1, self.3),
            Self(self.0 - 1, self.1 + 1, self.2 + 1, self.3 + 1),
            Self(self.0, self.1 - 1, self.2 - 1, self.3 - 1),
            Self(self.0, self.1 - 1, self.2 - 1, self.3),
            Self(self.0, self.1 - 1, self.2 - 1, self.3 + 1),
            Self(self.0, self.1 - 1, self.2, self.3 - 1),
            Self(self.0, self.1 - 1, self.2, self.3),
            Self(self.0, self.1 - 1, self.2, self.3 + 1),
            Self(self.0, self.1 - 1, self.2 + 1, self.3 - 1),
            Self(self.0, self.1 - 1, self.2 + 1, self.3),
            Self(self.0, self.1 - 1, self.2 + 1, self.3 + 1),
            Self(self.0, self.1, self.2 - 1, self.3 - 1),
            Self(self.0, self.1, self.2 - 1, self.3),
            Self(self.0, self.1, self.2 - 1, self.3 + 1),
            Self(self.0, self.1, self.2, self.3 - 1),
            Self(self.0, self.1, self.2, self.3 + 1),
            Self(self.0, self.1, self.2 + 1, self.3 - 1),
            Self(self.0, self.1, self.2 + 1, self.3),
            Self(self.0, self.1, self.2 + 1, self.3 + 1),
            Self(self.0, self.1 + 1, self.2 - 1, self.3 - 1),
            Self(self.0, self.1 + 1, self.2 - 1, self.3),
            Self(self.0, self.1 + 1, self.2 - 1, self.3 + 1),
            Self(self.0, self.1 + 1, self.2, self.3 - 1),
            Self(self.0, self.1 + 1, self.2, self.3),
            Self(self.0, self.1 + 1, self.2, self.3 + 1),
            Self(self.0, self.1 + 1, self.2 + 1, self.3 - 1),
            Self(self.0, self.1 + 1, self.2 + 1, self.3),
            Self(self.0, self.1 + 1, self.2 + 1, self.3 + 1),
            Self(self.0 + 1, self.1 - 1, self.2 - 1, self.3 - 1),
            Self(self.0 + 1, self.1 - 1, self.2 - 1, self.3),
            Self(self.0 + 1, self.1 - 1, self.2 - 1, self.3 + 1),
            Self(self.0 + 1, self.1 - 1, self.2, self.3 - 1),
            Self(self.0 + 1, self.1 - 1, self.2, self.3),
            Self(self.0 + 1, self.1 - 1, self.2, self.3 + 1),
            Self(self.0 + 1, self.1 - 1, self.2 + 1, self.3 - 1),
            Self(self.0 + 1, self.1 - 1, self.2 + 1, self.3),
            Self(self.0 + 1, self.1 - 1, self.2 + 1, self.3 + 1),
            Self(self.0 + 1, self.1, self.2 - 1, self.3 - 1),
            Self(self.0 + 1, self.1, self.2 - 1, self.3),
            Self(self.0 + 1, self.1, self.2 - 1, self.3 + 1),
            Self(self.0 + 1, self.1, self.2, self.3 - 1),
            Self(self.0 + 1, self.1, self.2, self.3),
            Self(self.0 + 1, self.1, self.2, self.3 + 1),
            Self(self.0 + 1, self.1, self.2 + 1, self.3 - 1),
            Self(self.0 + 1, self.1, self.2 + 1, self.3),
            Self(self.0 + 1, self.1, self.2 + 1, self.3 + 1),
            Self(self.0 + 1, self.1 + 1, self.2 - 1, self.3 - 1),
            Self(self.0 + 1, self.1 + 1, self.2 - 1, self.3),
            Self(self.0 + 1, self.1 + 1, self.2 - 1, self.3 + 1),
            Self(self.0 + 1, self.1 + 1, self.2, self.3 - 1),
            Self(self.0 + 1, self.1 + 1, self.2, self.3),
            Self(self.0 + 1, self.1 + 1, self.2, self.3 + 1),
            Self(self.0 + 1, self.1 + 1, self.2 + 1, self.3 - 1),
            Self(self.0 + 1, self.1 + 1, self.2 + 1, self.3),
            Self(self.0 + 1, self.1 + 1, self.2 + 1, self.3 + 1),
        ]
        .iter()
        {
            f(neighbour);
        }
    }
}
