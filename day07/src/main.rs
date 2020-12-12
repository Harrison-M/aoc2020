use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

const CONTAINED_RE_STR: &str = r"(?P<quantity>\d+) (?P<color>.+?) bag";
const CONTAINER_RE_STR: &str = r"^(?P<color>.+?) bags";

#[derive(Clone, Copy)]
struct BagRelationship<'a> {
    color: &'a str,
    quantity: usize,
}

type BagMap<'a> = HashMap<&'a str, Vec<BagRelationship<'a>>>;

/// Get a map of bags to the bags containing them, with counts of how many of this bag go into
/// them
fn generate_child_parents_map<'a>(specs: impl Iterator<Item=&'a str>)
    -> BagMap<'a> {
    let contained_re = Regex::new(CONTAINED_RE_STR).unwrap();
    let container_re = Regex::new(CONTAINER_RE_STR).unwrap();

    let mut child_parents_map: BagMap<'a> = HashMap::new();

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

            let relationship = BagRelationship {
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

/// Get a map of bags to the bags they contain and how many of each
fn generate_parent_children_map<'a>(specs: impl Iterator<Item=&'a str>)
    -> BagMap<'a> {
    let contained_re = Regex::new(CONTAINED_RE_STR).unwrap();
    let container_re = Regex::new(CONTAINER_RE_STR).unwrap();

    specs.map(|spec| {
        let parent = container_re
            .captures(spec)
            .and_then(|c| c.name("color"))
            .unwrap()
            .as_str();

        let children: Vec<BagRelationship<'a>> = contained_re
            .captures_iter(spec)
            .map(|caps| BagRelationship {
                color: caps.name("color").unwrap().as_str(),
                quantity: caps
                    .name("quantity")
                    .and_then(|q| q.as_str().parse().ok())
                    .unwrap()
            }).collect();

        (parent, children)
    }).collect()
}

/// Get a set of the bags that can contain this bag
fn ancestors<'a>(map: &BagMap<'a>, color: &'a str) -> HashSet<&'a str> {
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

/// Count the number of bags that have to be in a given bag color
fn child_count<'a>(map: &BagMap<'a>, color: &'a str) -> usize {
    map.get(color).map(|children| {
        children.iter().fold(0, |acc, child| {
            acc + child.quantity + child.quantity * child_count(map, child.color)
        })
    }).unwrap_or_default() // 0 if color is not in map
}

fn part1(map: &BagMap) -> usize {
    ancestors(map, "shiny gold").len()
}

fn part2(map: &BagMap) -> usize {
    child_count(map, "shiny gold")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let child_parent_map = generate_child_parents_map(contents.lines());

    println!("Part 1: {}", part1(&child_parent_map));

    let parent_child_map = generate_parent_children_map(contents.lines());
    println!("Part 2: {}", part2(&parent_child_map));
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

    #[test]
    fn part2_example1() {
        let map = generate_parent_children_map(SAMPLE.lines());
        assert_eq!(part2(&map), 32);
    }

    #[test]
    fn part2_example2() {
        let map = generate_parent_children_map(include_str!("sample2").lines());
        assert_eq!(part2(&map), 126)
    }
}
