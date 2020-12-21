use crate::{day::Day, util::Ignore};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}

pub struct Day21 {}

impl<'a> Day<'a> for Day21 {
    type Input1 = Vec<Food<'a>>;
    type Input2 = HashMap<&'a str, HashSet<&'a str>>;
    type Output1 = usize;
    type Output2 = String;

    const INDEX: usize = 21;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        raw_input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut parts = line.split('(');
                let ingredients = parts
                    .next()
                    .unwrap()
                    .split(' ')
                    .filter(|w| !w.is_empty())
                    .collect();
                let allergens = parts
                    .next()
                    .map(|s| s[9..s.len() - 1].split(", ").collect())
                    .unwrap_or_else(HashSet::new);
                Food {
                    ingredients,
                    allergens,
                }
            })
            .collect()
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let foods = input;
        let mut allergen_containment_map: HashMap<&str, HashSet<&str>> = HashMap::new();
        for food in foods.iter() {
            for allergen in food.allergens.iter() {
                match allergen_containment_map.get_mut(allergen) {
                    Some(ingredient_set) => {
                        ingredient_set.retain(|ingredient| food.ingredients.contains(ingredient))
                    }
                    None => allergen_containment_map
                        .insert(allergen, food.ingredients.clone())
                        .ignore(),
                }
            }
        }
        let possibly_dangerous = allergen_containment_map
            .values()
            .fold(HashSet::new(), |a, b| a.union(b).cloned().collect());
        let safe_count = foods
            .iter()
            .flat_map(|food| food.ingredients.iter())
            .filter(|ingredient| !possibly_dangerous.contains(*ingredient))
            .count();
        (allergen_containment_map, safe_count)
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        let mut allergen_containment_map = input;
        let mut known_allergens: HashMap<&str, &str> = HashMap::new();
        while known_allergens.len() < allergen_containment_map.len() {
            for (allergen, ingredients) in allergen_containment_map.iter_mut() {
                if known_allergens.contains_key(allergen) {
                    continue;
                }
                ingredients.retain(|ingredient| {
                    known_allergens
                        .values()
                        .find(|&i| i == ingredient)
                        .is_none()
                });
                if ingredients.len() == 1 {
                    known_allergens.insert(allergen, ingredients.iter().next().unwrap());
                }
            }
        }
        let mut known_allergens: Vec<_> = known_allergens.iter().collect();
        known_allergens.sort_by_key(|allergen_ingredient| allergen_ingredient.0);
        known_allergens
            .iter()
            .map(|allergen_ingredient| allergen_ingredient.1)
            .join(",")
    }
}
