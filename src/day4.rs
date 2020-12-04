use crate::day::Day;
use std::collections::HashMap;

pub struct Day4 {}

fn validate_number(text: &str, min: usize, max: usize) -> bool {
    text.parse::<usize>()
        .map(|n| min <= n && n <= max)
        .unwrap_or(false)
}

fn split_last2(text: &str) -> (&str, &str) {
    text.split_at((text.len() - 2).max(0))
}

pub struct KeyedPassport<'a> {
    birth_year: &'a str,
    issue_year: &'a str,
    expiration_year: &'a str,
    height: &'a str,
    hair_color: &'a str,
    eye_color: &'a str,
    passport_id: &'a str,
}

impl<'a> KeyedPassport<'a> {
    fn valid(&self) -> bool {
        let birth_year_valid = validate_number(self.birth_year, 1920, 2002);
        let issue_year_valid = validate_number(self.issue_year, 2010, 2020);
        let expiration_year_valid = validate_number(self.expiration_year, 2020, 2030);
        let height_valid = match split_last2(self.height) {
            (number, "cm") => validate_number(number, 150, 193),
            (number, "in") => validate_number(number, 59, 76),
            _ => false,
        };
        let hair_color_valid = self.hair_color.len() == 7
            && self.hair_color.starts_with('#')
            && self
                .hair_color
                .chars()
                .skip(1)
                .all(|c| c.is_ascii_hexdigit());
        let eye_color_valid = match self.eye_color {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        };
        let passport_id_valid =
            self.passport_id.len() == 9 && self.passport_id.chars().all(|c| c.is_ascii_digit());

        birth_year_valid
            && issue_year_valid
            && expiration_year_valid
            && height_valid
            && hair_color_valid
            && eye_color_valid
            && passport_id_valid
    }
}

pub struct Passport<'a> {
    fields: HashMap<&'a str, &'a str>,
}

impl<'a> Passport<'a> {
    fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    fn validate_keys(&self) -> Option<KeyedPassport<'a>> {
        Some(KeyedPassport {
            birth_year: self.fields.get("byr")?,
            issue_year: self.fields.get("iyr")?,
            expiration_year: self.fields.get("eyr")?,
            height: self.fields.get("hgt")?,
            hair_color: self.fields.get("hcl")?,
            eye_color: self.fields.get("ecl")?,
            passport_id: self.fields.get("pid")?,
        })
    }
}

impl<'a> Day<'a> for Day4 {
    type Input1 = Vec<Passport<'a>>;
    type Input2 = Vec<KeyedPassport<'a>>;
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 4;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        let mut passports = Vec::new();
        let lines: Vec<_> = raw_input.lines().collect();
        for passport_lines in lines.split(|line| line.is_empty()) {
            let mut passport = Passport::new();
            for line in passport_lines {
                for pair in line.split(' ') {
                    let (key, colon_value) = pair.split_at(3);
                    passport.fields.insert(key, &colon_value[1..]);
                }
            }
            passports.push(passport);
        }
        passports
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let keyed_passports: Vec<_> = input
            .iter()
            .filter_map(|passport| passport.validate_keys())
            .collect();
        let count = keyed_passports.len();
        (keyed_passports, count)
    }

    fn solve_part2(input: Self::Input2) -> Self::Output1 {
        input.iter().filter(|passport| passport.valid()).count()
    }
}
