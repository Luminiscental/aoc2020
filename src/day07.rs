use crate::day::Day;
use std::collections::{HashMap, HashSet};

fn take_words(count: usize, string: &str) -> (&str, &str) {
    string
        .chars()
        .enumerate()
        .filter(|pair| pair.1 == ' ')
        .nth(count - 1)
        .map(|pair| (&string[..pair.0], &string[pair.0 + 1..]))
        .unwrap_or((string, ""))
}

#[derive(Debug)]
pub struct RuleSet<'a> {
    child_map: HashMap<&'a str, Vec<(usize, &'a str)>>,
    parent_map: HashMap<&'a str, HashSet<&'a str>>,
}

pub struct Day07 {}

impl<'a> Day<'a> for Day07 {
    type Input1 = RuleSet<'a>;
    type Input2 = RuleSet<'a>;
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 7;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        let mut child_map = HashMap::new();
        let mut parent_map = HashMap::new();
        for line in raw_input.lines().filter(|line| !line.is_empty()) {
            let (parent_color, rest) = take_words(2, line);
            let (_, children) = take_words(2, rest);
            if children.starts_with("no") {
                child_map.insert(parent_color, Vec::new());
            } else {
                let mut children = children;
                while !children.is_empty() {
                    let (child, remaining_children) = take_words(4, children);
                    let (child_count, rest) = take_words(1, child);
                    let child_count = child_count.parse::<usize>().unwrap();
                    let (child_color, _) = take_words(2, rest);
                    child_map
                        .entry(parent_color)
                        .or_insert_with(Vec::new)
                        .push((child_count, child_color));
                    parent_map
                        .entry(child_color)
                        .or_insert_with(HashSet::new)
                        .insert(parent_color);
                    children = remaining_children;
                }
            };
        }
        RuleSet {
            child_map,
            parent_map,
        }
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let mut ancestor_set = HashSet::new();
        let mut stack = vec!["shiny gold"];
        while let Some(color) = stack.pop() {
            ancestor_set.insert(color);
            if let Some(parents) = input.parent_map.get(color) {
                for parent_color in parents.iter() {
                    stack.push(parent_color);
                }
            }
        }
        (input, ancestor_set.len() - 1)
    }

    fn solve_part2(input: Self::Input2) -> Self::Output1 {
        let mut total_count = 0;
        let mut stack = vec![(1, "shiny gold")];
        while let Some((count, color)) = stack.pop() {
            total_count += count;
            if let Some(children) = input.child_map.get(color) {
                for (child_count, child_color) in children.iter() {
                    stack.push((child_count * count, child_color));
                }
            }
        }
        total_count - 1
    }
}
