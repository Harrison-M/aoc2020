use regex::Regex;
use std::{collections::{HashSet, HashMap}, env, fs, cell::RefCell, rc::Rc};

const RULE_RE_STR: &str = r"(\d+): (.+)";

enum Rule<'a> {
    RuleRef(Vec<Vec<u8>>),
    Char(&'a str),
}

use Rule::*;

type RulePossibilities = HashMap<u8, HashSet<String>>;
type RuleTable<'a> = HashMap<u8, Rule<'a>>;

fn build_rule_table<'a>(rules: &'a str) -> RuleTable<'a> {
    let rule_re = Regex::new(RULE_RE_STR).unwrap();
    rules
        .lines()
        .map(|line| {
            let caps = rule_re.captures(line).unwrap();
            let idx: u8 = caps
                .get(1)
                .and_then(|c| c.as_str().parse().ok())
                .unwrap();
            let rule_str = caps.get(2).unwrap().as_str();
            let rule = match rule_str
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"')) {
                    Some(c) => Char(c),
                    None => RuleRef(rule_str
                        .split(" | ")
                        .map(|set| set
                            .split(' ')
                            .map(|num| num.parse().unwrap())
                            .collect()
                        ).collect()
                    )
                };
            (idx, rule)
        }).collect()
}

fn combine_possibilities(sets: &[HashSet<String>]) -> HashSet<String>{
    match sets.split_first() {
        Some((set, &[])) => set.clone(),
        Some((set, next)) => {
            let next_possibilities = combine_possibilities(next);
            set
                .iter()
                .flat_map(|s| next_possibilities
                    .iter()
                    .map(move |suffix| s.to_owned() + suffix)
                )
                .collect()
        }
        None => panic!("Base case has one item in sets"),
    }
}

fn find_possibilities(idx: u8, rules: &RuleTable, cache: Rc<RefCell<RulePossibilities>>)
    -> HashSet<String> {
        let borrowed_cache = cache.borrow();
        let maybe_set = borrowed_cache.get(&idx).cloned();
        drop(borrowed_cache);

        if let Some(set) = maybe_set {
            set
        } else {
            match rules.get(&idx).unwrap() {
                Char(s) => {
                    let mut set = HashSet::new();
                    set.insert((*s).to_owned());
                    {
                        let mut borrowed_cache = cache.borrow_mut();
                        borrowed_cache.insert(idx, set.clone());
                        drop(borrowed_cache);
                    }
                    set
                },
                RuleRef(r) => {
                    let set: HashSet<_> = r.iter()
                        .flat_map(|seq| {
                            let possibility_sets: Vec<HashSet<String>> = seq
                                .iter()
                                .map(|&i|
                                    find_possibilities(i, rules, Rc::clone(&cache)).clone()
                                ).collect();

                            combine_possibilities(&possibility_sets[..])
                        }).collect();
                    {
                        let mut borrowed_cache = cache.borrow_mut();
                        borrowed_cache.insert(idx, set.clone());
                        drop(borrowed_cache);
                    }
                    
                    set
                }
            }
        }
}

fn part1(rules: &RuleTable, cache: Rc<RefCell<RulePossibilities>>, messages: &Vec<&str>) -> usize {
    let possibilities = find_possibilities(0, rules, cache);
    messages
        .iter()
        .filter(|m| possibilities.contains(&m.to_string()))
        .count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let mut sections = contents.split("\n\n");
    let rules = build_rule_table(sections.next().unwrap());
    let cache: RulePossibilities = HashMap::new();
    let messages: Vec<&str> = sections.next().unwrap().lines().collect();

    println!("Part 1: {}", part1(&rules, Rc::new(RefCell::new(cache)), &messages));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        let sample = include_str!("sample");
        let mut sections = sample.split("\n\n");
        let rules = build_rule_table(sections.next().unwrap());
        let cache: Rc<RefCell<RulePossibilities>> = Rc::new(RefCell::new(HashMap::new()));
        let messages: Vec<&str> = sections.next().unwrap().lines().collect();

        assert_eq!(part1(&rules, Rc::clone(&cache), &messages), 2);
    }
}
