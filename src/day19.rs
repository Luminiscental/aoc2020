use crate::{day::Day, set};
use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Debug)]
pub enum Rule {
    TerminalA,
    TerminalB,
    Alternatives(Vec<Vec<usize>>),
}

impl Rule {
    fn parse(string: &str) -> (usize, Self) {
        let mut parts = string.split(": ");
        let key = parts.next().unwrap().parse().unwrap();
        let value = parts.next().unwrap();

        let rule = match value {
            "\"a\"" => Rule::TerminalA,
            "\"b\"" => Rule::TerminalB,
            value => Rule::Alternatives(
                value
                    .split(" | ")
                    .map(|conj| conj.split(' ').map(|idx| idx.parse().unwrap()).collect())
                    .collect(),
            ),
        };
        (key, rule)
    }
}

fn build_languages(
    root: usize,
    rules: &HashMap<usize, Rule>,
    languages: &mut HashMap<usize, HashSet<String>>,
) {
    let lang = match &rules[&root] {
        Rule::TerminalA => set! {"a".to_owned()},
        Rule::TerminalB => set! {"b".to_owned()},
        Rule::Alternatives(disj) => disj
            .iter()
            .map(|conj| {
                conj.iter()
                    .map(|idx| {
                        languages.get(idx).cloned().unwrap_or_else(|| {
                            build_languages(*idx, rules, languages);
                            languages[idx].clone()
                        })
                    })
                    .fold1(|lang, more| {
                        iproduct!(lang.iter(), more.iter())
                            .map(|pair| format!("{}{}", pair.0, pair.1))
                            .collect()
                    })
                    .unwrap()
            })
            .fold1(|mut lang, more| {
                lang.extend(more);
                lang
            })
            .unwrap(),
    };
    languages.insert(root, lang);
}

pub struct Day19 {}

impl<'a> Day<'a> for Day19 {
    type Input1 = (HashMap<usize, Rule>, Vec<&'a str>);
    type Input2 = (
        HashMap<usize, Rule>,
        HashMap<usize, HashSet<String>>,
        Vec<&'a str>,
    );
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 19;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        let lines: Vec<_> = raw_input.lines().collect();
        let mut chunks = lines.split(|line| line.is_empty());
        let rules = chunks.next().unwrap();
        let passwords = chunks.next().unwrap();
        (
            rules.iter().map(|line| Rule::parse(line.trim())).collect(),
            passwords.to_vec(),
        )
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let (rules, passwords) = input;
        let mut languages = HashMap::new();
        build_languages(0, &rules, &mut languages);
        let valid = passwords
            .iter()
            .filter(|&&pass| languages[&0].contains(pass))
            .count();
        ((rules, languages, passwords), valid)
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        let (rules, languages, passwords) = input;
        // we now have that:
        // 8 is any non-empty sequence of 42's
        // 11 is any non-empty sequence of 42's followed by an equal number of 31's
        assert!(rules[&0] == Rule::Alternatives(vec![vec![8, 11]]));
        assert!(languages[&42].intersection(&languages[&31]).count() == 0);
        assert!(
            languages[&42]
                .iter()
                .chain(languages[&31].iter())
                .map(|s| s.len())
                .collect::<HashSet<_>>()
                .len()
                == 1
        );
        let length = languages[&42].iter().next().unwrap().len();
        // and with these assertions we can match 0 as follows
        let many = |lang: &HashSet<String>, string: &'a str| -> (usize, &'a str) {
            let mut string = string;
            let mut count = 0;
            while string.len() >= length && lang.contains(&string[0..length]) {
                count += 1;
                string = &string[length..];
            }
            (count, string)
        };
        passwords
            .into_iter()
            .filter(|pass| {
                let (n42, pass) = many(&languages[&42], pass);
                let (n31, pass) = many(&languages[&31], pass);
                pass.is_empty() && n42 > n31 && n31 > 0
            })
            .count()
    }
}
