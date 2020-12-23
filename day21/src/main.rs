use itertools::Itertools;
use regex::Regex;
use std::{collections::{BTreeMap, HashMap, HashSet}, env, fs};

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

fn solutions(i_and_a: &IngredientsAndAllergens) -> Result<(usize, String), String> {
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

    let part1 = i_and_a
        .iter()
        .flat_map(|(ingredients, _)| ingredients)
        .filter(|&ingredient| !all_possible_sources.contains(ingredient))
        .count();

    // A sorted-key map of allergen to ingredient
    let mut allergen_sources: BTreeMap<&str, &str> = BTreeMap::new();

    while possible_allergen_sources.len() > 0 {
        let definite_sources: Vec<_> = {
            possible_allergen_sources
                .iter()
                .filter(|(_, set)| set.len() == 1)
                .map(|(allergen, set)| (allergen.clone(), set.clone()))
                .collect()
        };

        if definite_sources.is_empty() {
            return Err("Could not definitively source allergens".to_string());
        }

        for (allergen, set) in definite_sources {
            let ingredient = *set.iter().next().unwrap();
            allergen_sources.insert(allergen, ingredient);
            possible_allergen_sources.remove(allergen);
            for possibility_set in possible_allergen_sources.values_mut() {
                possibility_set.remove(ingredient);
            }
        }
    }

    let part2 = allergen_sources.values().join(",");

    Ok((part1, part2))
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let i_and_a = ingredients_and_allergens(&contents);

    let (part1, part2) = solutions(&i_and_a)?;
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let sample = include_str!("sample");
        let i_and_a = ingredients_and_allergens(&sample);

        assert_eq!(solutions(&i_and_a).unwrap(), (5, "mxmxvkd,sqjhc,fvjkl".to_string()));
    }
}
