use crate::{day::Day, set};
use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet};

pub enum Rule {
    TerminalA,
    TerminalB,
    Alternatives(Vec<Vec<usize>>),
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum TerminalOrRule {
    TerminalA,
    TerminalB,
    Rule(usize),
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

    fn language_strings(&self, rules: &HashMap<usize, Rule>) -> HashSet<String> {
        self.language(rules, &[])
            .into_iter()
            .map(|vec| {
                vec.into_iter()
                    .map(|tor| match tor {
                        TerminalOrRule::TerminalA => 'a',
                        TerminalOrRule::TerminalB => 'b',
                        TerminalOrRule::Rule(_) => unreachable!(),
                    })
                    .collect()
            })
            .collect()
    }

    fn language(
        &self,
        rules: &HashMap<usize, Rule>,
        dont_expand: &[usize],
    ) -> HashSet<Vec<TerminalOrRule>> {
        match self {
            Rule::TerminalA => set! {vec![TerminalOrRule::TerminalA]},
            Rule::TerminalB => set! {vec![TerminalOrRule::TerminalB]},
            Rule::Alternatives(disj) => disj
                .iter()
                .map(|conj| {
                    conj.iter()
                        .map(|idx| {
                            if dont_expand.contains(idx) {
                                set! {vec![TerminalOrRule::Rule(*idx)]}
                            } else {
                                rules[idx].language(rules, dont_expand)
                            }
                        })
                        .fold1(|lang, more| {
                            iproduct!(lang.iter(), more.iter())
                                .map(|pair| pair.0.iter().chain(pair.1.iter()).cloned().collect())
                                .collect()
                        })
                        .unwrap()
                })
                .fold(HashSet::new(), |mut lang, more| {
                    lang.extend(more);
                    lang
                }),
        }
    }
}

pub struct Day19 {}

impl<'a> Day<'a> for Day19 {
    type Input1 = (HashMap<usize, Rule>, Vec<&'a str>);
    type Input2 = (HashMap<usize, Rule>, Vec<&'a str>);
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
        let language = rules[&0].language_strings(&rules);
        let valid = passwords
            .iter()
            .filter(|&&pass| language.contains(pass))
            .count();
        ((rules, passwords), valid)
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        let (mut rules, passwords) = input;
        // 8 is any non-empty sequence of 42's
        rules.insert(8, Rule::Alternatives(vec![vec![42], vec![42, 8]]));
        // 11 is any non-empty sequence of 42's followed by an equal number of 31's
        rules.insert(11, Rule::Alternatives(vec![vec![42, 31], vec![42, 11, 31]]));
        let lang0 = rules[&0].language(&rules, &[8, 11]);
        let lang42: HashSet<String> = rules[&42].language_strings(&rules);
        let lang31 = rules[&31].language_strings(&rules);
        assert!(lang0 == set! {vec![TerminalOrRule::Rule(8), TerminalOrRule::Rule(11)]});
        assert!(lang42.intersection(&lang31).count() == 0);
        assert!(
            lang42
                .iter()
                .chain(lang31.iter())
                .map(|s| s.len())
                .collect::<HashSet<_>>()
                .len()
                == 1
        );
        passwords
            .into_iter()
            .filter(|pass| {
                let mut pass = *pass;
                let mut count42 = 0;
                while let Some(rest) = lang42.iter().filter_map(|s| pass.strip_prefix(s)).next() {
                    pass = rest;
                    count42 += 1;
                }
                let mut count31 = 0;
                while let Some(rest) = lang31.iter().filter_map(|s| pass.strip_prefix(s)).next() {
                    pass = rest;
                    count31 += 1;
                }
                pass.is_empty() && count42 > count31 && count31 > 0
            })
            .count()
    }
}
