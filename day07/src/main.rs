use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

const CONTAINED_RE_STR: &str = r"(?P<quantity>\d+) (?P<color>.+?) bag";
const CONTAINER_RE_STR: &str = r"^(?P<color>.+?) bags";

#[derive(Clone, Copy)]
struct ParentRelationship<'a> {
    color: &'a str,
    quantity: usize,
}

type ChildParentsMap<'a> = HashMap<&'a str, Vec<ParentRelationship<'a>>>;

fn generate_child_parents_map<'a>(specs: impl Iterator<Item=&'a str>)
    -> ChildParentsMap<'a> {
    let contained_re = Regex::new(CONTAINED_RE_STR).unwrap();
    let container_re = Regex::new(CONTAINER_RE_STR).unwrap();

    let mut child_parents_map: ChildParentsMap<'a> = HashMap::new();

    for spec in specs {
        let parent = container_re
            .captures(spec)
            .and_then(|c| c.name("color"))
            .unwrap()
            .as_str();

        for caps in contained_re.captures_iter(spec) {
            let child = caps.name("color").unwrap().as_str();
            let quantity: usize = caps
                .name("quantity")
                .and_then(|q| q.as_str().parse().ok())
                .unwrap();

            let relationship = ParentRelationship {
                    color: parent,
                    quantity,
            };
            child_parents_map
                .entry(child)
                .and_modify(|e| e.push(relationship))
                .or_insert(vec![relationship]);
        }
    }

    child_parents_map
}

fn ancestors<'a>(map: &ChildParentsMap<'a>, color: &'a str) -> HashSet<&'a str> {
    match map.get(color) {
        None => HashSet::new(),
        Some(parents) => {
            let parent_set: HashSet<&str> = parents
                .iter()
                .map(|p| p.color)
                .collect();
            parents.iter().fold(
                parent_set,
                |acc, parent| acc
                    .union(&ancestors(map, parent.color))
                    .cloned()
                    .collect()
            )
        }
    }
}

fn part1(map: &ChildParentsMap) -> usize {
    ancestors(map, "shiny gold").len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let map = generate_child_parents_map(contents.lines());

    println!("Part 1: {}", part1(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("sample");

    #[test]
    fn part1_example() {
        let map = generate_child_parents_map(SAMPLE.lines());
        assert_eq!(part1(&map), 4);
    }
}
