use std::collections::HashSet;
use std::env;
use std::fs;

/// Find the number of unique responses per group and sum them
fn part1(responses: String) -> usize {
    responses.split("\n\n")
        .fold(0, |acc, group| {
            let mut answer_set: HashSet<char> = HashSet::new();
            for c in group.chars() {
                if c.is_alphabetic() {
                    answer_set.insert(c);
                }
            }
            acc + answer_set.len()
        })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    println!("Part 1: {}", part1(contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(include_str!("sample").to_string()), 11);
    }
}
