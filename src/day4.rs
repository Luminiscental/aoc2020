use crate::day::Day;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub struct Day4 {}

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
        lazy_static! {
            static ref HEIGHT_REGEX: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
            static ref HAIR_COLOR_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            static ref EYE_COLOR_REGEX: Regex =
                Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            static ref ID_REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
        }
        let birth_year_valid = self
            .birth_year
            .parse::<usize>()
            .map(|yr| 1920 <= yr && yr <= 2002)
            .unwrap_or(false);
        let issue_year_valid = self
            .issue_year
            .parse::<usize>()
            .map(|yr| 2010 <= yr && yr <= 2020)
            .unwrap_or(false);
        let expiration_year_valid = self
            .expiration_year
            .parse::<usize>()
            .map(|yr| 2020 <= yr && yr <= 2030)
            .unwrap_or(false);
        let height_valid = match HEIGHT_REGEX.captures(self.height) {
            None => false,
            Some(captures) => {
                let number = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                match captures.get(2).unwrap().as_str() {
                    "cm" => 150 <= number && number <= 193,
                    "in" => 59 <= number && number <= 76,
                    _ => unreachable!(),
                }
            }
        };
        let hair_color_valid = HAIR_COLOR_REGEX.is_match(self.hair_color);
        let eye_color_valid = EYE_COLOR_REGEX.is_match(self.eye_color);
        let passport_id_valid = ID_REGEX.is_match(self.passport_id);

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
        let birth_year = self.fields.get("byr")?;
        let issue_year = self.fields.get("iyr")?;
        let expiration_year = self.fields.get("eyr")?;
        let height = self.fields.get("hgt")?;
        let hair_color = self.fields.get("hcl")?;
        let eye_color = self.fields.get("ecl")?;
        let passport_id = self.fields.get("pid")?;

        Some(KeyedPassport {
            birth_year,
            issue_year,
            expiration_year,
            height,
            hair_color,
            eye_color,
            passport_id,
        })
    }
}

impl<'a> Day<'a> for Day4 {
    type Input = Vec<Passport<'a>>;
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 4;

    fn parse(raw_input: &'a str) -> Self::Input {
        let mut passports = Vec::new();
        let mut current_passport = Passport::new();
        for line in raw_input.lines() {
            if line.is_empty() {
                passports.push(current_passport);
                current_passport = Passport::new();
            } else {
                for pair in line.split(' ') {
                    let parts: Vec<_> = pair.split(':').collect();
                    current_passport.fields.insert(parts[0], parts[1]);
                }
            }
        }
        passports
    }

    fn solve_part1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .filter(|passport| passport.validate_keys().is_some())
            .count()
    }

    fn solve_part2(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .filter_map(|passport| passport.validate_keys())
            .filter(|passport| passport.valid())
            .count()
    }
}
