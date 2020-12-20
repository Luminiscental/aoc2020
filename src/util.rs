use std::cmp::Ord;

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
