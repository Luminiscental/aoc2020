use crate::day::Day;
use std::collections::HashMap;

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
}

pub struct Day7 {}

impl<'a> Day<'a> for Day7 {
    type Input1 = RuleSet<'a>;
    type Input2 = RuleSet<'a>;
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 7;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        RuleSet {
            child_map: raw_input
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| {
                    let (parent_color, rest) = take_words(2, line);
                    let (_, children) = take_words(2, rest);
                    let children = if children.starts_with("no") {
                        Vec::new()
                    } else {
                        let mut children = children;
                        let mut vec = Vec::new();
                        while !children.is_empty() {
                            let (child, remaining_children) = take_words(4, children);
                            let (child_count, rest) = take_words(1, child);
                            let (child_color, _) = take_words(2, rest);
                            vec.push((child_count.parse::<usize>().unwrap(), child_color));
                            children = remaining_children;
                        }
                        vec
                    };
                    (parent_color, children)
                })
                .collect(),
        }
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let mut contains_map = HashMap::new();
        contains_map.insert("shiny gold", true);
        while contains_map.len() < input.child_map.len() {
            for (color, children) in input.child_map.iter() {
                if !contains_map.contains_key(color) {
                    if children.is_empty() {
                        contains_map.insert(color, false);
                    } else if let Some(containment) = children
                        .iter()
                        .map(|pair| contains_map.get(&pair.1))
                        // if any Some(true), result is Some(true)
                        // if all Some(false), result is Some(false)
                        // else result is None
                        .fold(Some(false), |acc, child| match (acc, child) {
                            (Some(true), _) => Some(true),
                            (_, Some(true)) => Some(true),
                            (acc, Some(false)) => acc,
                            (_, None) => None,
                        })
                    {
                        contains_map.insert(color, containment);
                    }
                }
            }
        }
        let count = contains_map.iter().filter(|pair| *pair.1).count() - 1;
        (input, count)
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
