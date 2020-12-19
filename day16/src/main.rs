use regex::{Captures, Regex};
use std::{collections::{HashMap, HashSet}, env, fs, ops::RangeInclusive, str::FromStr, fmt::Debug};

const RULE_RE_STR: &str = r"(.+): (\d+)-(\d+) or (\d+)-(\d+)";

type Ticket = Vec<usize>;
type TicketRules<'a> = Vec<(&'a str, [RangeInclusive<usize>; 2])>;

#[derive(Debug)]
struct Problem<'a> {
    invalid_tickets: Vec<Ticket>,
    my_ticket: Ticket,
    other_tickets: Vec<Ticket>,
    rules: TicketRules<'a>,
    valid_tickets: Vec<Ticket>,
}

impl<'a> Problem<'a> {
    fn new(input: &'a str) -> Self {
        let mut sections = input.split("\n\n");
        let rules = build_rules(sections.next().unwrap());
        let my_ticket = build_tickets(sections.next().unwrap()).remove(0);
        let other_tickets = build_tickets(sections.next().unwrap());

        let all_ranges: Vec<&RangeInclusive<usize>> = rules
            .iter()
            .flat_map(|(_, ranges)| ranges.iter())
            .collect();

        let (valid_tickets, invalid_tickets) = other_tickets
            .iter()
            .cloned()
            .partition(|ticket|
                ticket
                    .iter()
                    .all(|num|
                        all_ranges
                            .iter()
                            .any(|range| range.contains(num))
                    )
            );

        Self { invalid_tickets, my_ticket, other_tickets, rules, valid_tickets }
    }
}

/// Parses regex match at idx with unwraps
fn parse_match<T: FromStr>(caps: &Captures, idx: usize) -> T 
    where <T as FromStr>::Err: Debug {
    caps.get(idx).unwrap().as_str().parse().unwrap()
}

/// Parses rules into names and ranges
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

/// Parses a list of tickets
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
        .iter()
        .flat_map(|(_, ranges)| ranges.iter())
        .collect();

    problem.invalid_tickets
        .iter()
        .flat_map(|ticket| ticket.iter())
        .filter(|&num| !all_ranges
            .iter()
            .any(|range| range.contains(num))
        ).sum()
}

/// Eliminate possibilities for field matches by seeing what rules a ticket fails to match
fn eliminate_possibilities(
    problem: &Problem,
    ticket: &Ticket,
    possible_fields_by_pos: &mut Vec<HashSet<usize>>,
) {
    for (num, possible_fields) in ticket
        .iter()
        .zip(possible_fields_by_pos.iter_mut()) {
            possible_fields.retain(|field| {
                let (_, [range1, range2]) = problem.rules.get(*field).unwrap();
                range1.contains(num) || range2.contains(num)
            })
        }
}

fn part2(problem: &Problem) -> usize {
    let field_count = problem.my_ticket.len();
    let possible_field_template: HashSet<usize> = (0..field_count).collect();
    let mut possible_fields_by_pos: Vec<HashSet<usize>> = (0..field_count)
        .map(|_| possible_field_template.clone())
        .collect();

    // Check own ticket
    eliminate_possibilities(problem, &problem.my_ticket, &mut possible_fields_by_pos);
    for ticket in problem.valid_tickets.iter() {
        eliminate_possibilities(problem, ticket, &mut possible_fields_by_pos);
    }

    let mut final_possibilities: Vec<_> = possible_fields_by_pos
        .into_iter()
        .enumerate()
        .collect();

    final_possibilities.sort_unstable_by_key(|(_, set)| set.len());

    let mut idx_to_field: HashMap<usize, &str> = HashMap::new();
    let (first_idx, first_set) = final_possibilities.first().unwrap();
    idx_to_field.insert(
        *first_idx,
        first_set
            .iter()
            .next()
            .and_then(|field| problem.rules.get(*field))
            .unwrap().0
    );
    for window in final_possibilities.windows(2) {
        let set1 = &window.get(0).unwrap().1;
        let (idx, set2) = window.get(1).unwrap();
        idx_to_field.insert(
            *idx,
            set2.difference(set1)
                .next()
                .and_then(|field| problem.rules.get(*field))
                .unwrap().0
        );
    }

    idx_to_field
        .into_iter()
        .filter(|(_, field)| field.starts_with("departure"))
        .map(|(idx, _)| problem.my_ticket.get(idx).unwrap())
        .product()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let problem = Problem::new(&contents);

    println!("My ticket: {:?}", problem.my_ticket);
    println!("Part 1: {}", part1(&problem));
    println!("Part 2: {}", part2(&problem));
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

    // Need to separate field detection from calculation to test part 2
}
