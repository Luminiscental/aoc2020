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
