use std::collections::HashSet;
use std::env;
use std::fs;

/// Find the number of unique responses per group and sum them
fn part1(responses: &String) -> usize {
    responses.split("\n\n")
        .fold(0, |acc, group| {
            let answer_set: HashSet<_> = group.chars()
                .filter(|c| c.is_alphabetic()).collect();
            acc + answer_set.len()
        })
}

/// Find the number of responses every member of a group marked
fn part2(responses: &String) -> usize {
    responses.split("\n\n")
        .fold(0, |acc, group| {
            group.lines()
                .fold(None, |group_acc: Option<HashSet<char>>, survey| {
                    let answer_set: HashSet<char> = survey.chars().collect();
                    match group_acc {
                        None => Some(answer_set),
                        Some(acc_set) =>
                            Some(
                                acc_set
                                    .intersection(&answer_set)
                                    .cloned()
                                    .collect()
                            )
                    }
                }).map_or(0, |s| s.len()) + acc
        })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("sample");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&SAMPLE.to_string()), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&SAMPLE.to_string()), 6);
    }
}
