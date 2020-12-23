use regex::Regex;
use std::{collections::{HashMap, HashSet}, env, fs};

const LIST_RE_STR: &str = r"(?m)^(.+) \(contains (.+)\)$";

type IngredientsAndAllergens<'a> = Vec<(HashSet<&'a str>, Vec<&'a str>)>;

fn ingredients_and_allergens(list: &str) -> IngredientsAndAllergens {
    let list_re = Regex::new(LIST_RE_STR).unwrap();

    list_re
        .captures_iter(list)
        .map(|caps| {
            let ingredients = caps
                .get(1)
                .unwrap()
                .as_str()
                .split(' ')
                .collect();

            let allergens = caps
                .get(2)
                .unwrap()
                .as_str()
                .split(", ")
                .collect();

            (ingredients, allergens)
        })
        .collect()
}

fn part1(i_and_a: &IngredientsAndAllergens) -> usize {
    let mut possible_allergen_sources: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (ingredients, allergens) in i_and_a.iter() {
        for &allergen in allergens.iter() {
            possible_allergen_sources
                .entry(allergen)
                .and_modify(|possibilities|
                    *possibilities = possibilities
                        .intersection(ingredients)
                        .copied()
                        .collect()
                )
                .or_insert(ingredients.clone());
        }
    }
    let all_possible_sources: HashSet<&str> = possible_allergen_sources
        .values()
        .flat_map(|possibilities| possibilities.iter())
        .copied()
        .collect();

    i_and_a
        .iter()
        .flat_map(|(ingredients, _)| ingredients)
        .filter(|&ingredient| !all_possible_sources.contains(ingredient))
        .count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let i_and_a = ingredients_and_allergens(&contents);

    println!("Part 1: {}", part1(&i_and_a));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        let sample = include_str!("sample");
        let i_and_a = ingredients_and_allergens(&sample);

        assert_eq!(part1(&i_and_a), 5);
    }
}
