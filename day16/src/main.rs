use regex::{Captures, Regex};
use std::{collections::HashMap, env, fs, ops::RangeInclusive, str::FromStr, fmt::Debug};

const RULE_RE_STR: &str = r"(.+): (\d+)-(\d+) or (\d+)-(\d+)";

type Ticket = Vec<usize>;
type TicketRules<'a> = HashMap<&'a str, [RangeInclusive<usize>; 2]>;

#[derive(Debug)]
struct Problem<'a> {
    my_ticket: Ticket,
    other_tickets: Vec<Ticket>,
    rules: TicketRules<'a>,
}

impl<'a> Problem<'a> {
    fn new(input: &'a str) -> Self {
        let mut sections = input.split("\n\n");
        let rules = build_rules(sections.next().unwrap());
        let my_ticket = build_tickets(sections.next().unwrap()).remove(0);
        let other_tickets = build_tickets(sections.next().unwrap());

        Self { my_ticket, other_tickets, rules }
    }
}

fn parse_match<T: FromStr>(caps: &Captures, idx: usize) -> T 
    where <T as FromStr>::Err: Debug {
    caps.get(idx).unwrap().as_str().parse().unwrap()
}

fn build_rules<'a>(rules: &'a str) -> TicketRules<'a> {
    let rule_re = Regex::new(RULE_RE_STR).unwrap();

    rules
        .lines()
        .map(|rule| {
            let caps = rule_re.captures(rule).unwrap();
            let name = caps.get(1).unwrap().as_str();
            let range1start = parse_match(&caps, 2);
            let range1end = parse_match(&caps, 3);
            let range2start = parse_match(&caps, 4);
            let range2end = parse_match(&caps, 5);
            (name, [range1start..=range1end, range2start..=range2end])
        }).collect()
}

fn build_tickets<'a>(tickets: &'a str) -> Vec<Ticket> {
    tickets
        .lines()
        .skip(1)
        .map(|ticket|
            ticket
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect()
        ).collect()
}

fn part1(problem: &Problem) -> usize {
    let all_ranges: Vec<&RangeInclusive<usize>> = problem.rules
        .values()
        .flat_map(|ranges| ranges.iter())
        .collect();

    problem.other_tickets
        .iter()
        .flat_map(|ticket| ticket.iter())
        .filter(|&num| !all_ranges
            .iter()
            .any(|range| range.contains(num))
        ).sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let problem = Problem::new(&contents);

    println!("My ticket: {:?}", problem.my_ticket);
    println!("Part 1: {}", part1(&problem));
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = include_str!("sample");

    #[test]
    fn part1_example() {
        let problem = Problem::new(SAMPLE);
        assert_eq!(part1(&problem), 71);
    }
}
