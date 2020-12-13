use crate::day::Day;

fn mulmod(mut rem1: usize, mut rem2: usize, base: usize) -> usize {
    if rem1 > rem2 {
        let old = (rem1, rem2);
        rem1 = old.1;
        rem2 = old.0;
    }
    rem1 %= base;
    let mut result = 0;
    while rem2 != 0 {
        if rem2 % 2 == 1 {
            result = (result + rem1) % base;
        }
        rem1 = (2 * rem1) % base;
        rem2 /= 2;
    }
    result
}

fn negmod(rem: usize, base: usize) -> usize {
    let mut zero = base;
    while zero < rem {
        zero += base;
    }
    zero - rem
}

fn extended_euclid(n1: usize, n2: usize) -> (i64, i64, i64) {
    let mut r_pair = (n1 as i64, n2 as i64);
    let mut s1_pair = (1, 0);
    let mut s2_pair = (0, 1);

    while r_pair.1 != 0 {
        let quot = r_pair.0 / r_pair.1;
        r_pair = (r_pair.1, r_pair.0 - quot * r_pair.1);
        s1_pair = (s1_pair.1, s1_pair.0 - quot * s1_pair.1);
        s2_pair = (s2_pair.1, s2_pair.0 - quot * s2_pair.1);
    }

    (s1_pair.0, s2_pair.0, r_pair.0)
}

fn solve_2_congruences(congruence1: (usize, usize), congruence2: (usize, usize)) -> (usize, usize) {
    let (r1, n1) = congruence1;
    let (r2, n2) = congruence2;
    let n12 = n1 * n2;
    let (mut s1, mut s2, gcd) = extended_euclid(n1, n2);
    assert!(gcd == 1, "not coprime");
    while s1 < 0 {
        s1 += n12 as i64;
    }
    while s2 < 0 {
        s2 += n12 as i64;
    }
    let s1 = s1 as usize;
    let s2 = s2 as usize;
    let r12 = mulmod(mulmod(s2, n2, n12), r1, n12) + mulmod(mulmod(s1, n1, n12), r2, n12);
    (r12, n12)
}

pub struct Day13 {}

impl<'a> Day<'a> for Day13 {
    type Input1 = (usize, Vec<Option<usize>>);
    type Input2 = Vec<Option<usize>>;
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 13;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        let lines: Vec<_> = raw_input.lines().filter(|line| !line.is_empty()).collect();
        (
            lines[0].parse::<usize>().unwrap(),
            lines[1]
                .split(',')
                .map(|s| match s {
                    "x" => None,
                    n => Some(n.parse::<usize>().unwrap()),
                })
                .collect(),
        )
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let (min, buses) = input;
        let (earliest_bus, wait_time) = buses
            .iter()
            .filter_map(|&bus| bus)
            .map(|bus| match min % bus {
                0 => (bus, 0),
                r => (bus, bus - r),
            })
            .min_by_key(|pair| pair.1)
            .unwrap();
        (buses, earliest_bus * wait_time)
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        let mut congruences = input.iter().enumerate().filter_map(|pair| match pair {
            (_, None) => None,
            (idx, Some(n)) => Some((negmod(idx, *n), *n)),
        });
        let first = congruences.next().unwrap();
        let (r, _) = congruences.fold(first, solve_2_congruences);
        r
    }
}
