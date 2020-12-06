use regex::Regex;
use std::env;
use std::fs;

const RE_STR: &str = r"(?P<param0>\d+)-(?P<param1>\d+) (?P<character>[a-z]): (?P<password>[a-z]+)";

/// Counts valid passwords for a set of policy & password lines
fn part1(passwords: Vec<&str>) -> usize {
    let re = Regex::new(RE_STR).unwrap();

    passwords.iter().filter(|entry| {
        let captures = re.captures(entry).expect("Line failed to match");

        let min: usize = captures.name("param0").map(|min_str| min_str.as_str().parse().unwrap()).unwrap();
        let max: usize = captures.name("param1").map(|max_str| max_str.as_str().parse().unwrap()).unwrap();
        let character = captures.name("character").unwrap().as_str().chars().next().unwrap();
        let password = captures.name("password").unwrap().as_str();

        let char_count = password.chars().filter(|c| *c == character).count();
        min <= char_count && char_count <= max
    }).count()
}

/// Counts valid passwords using the second validation method
fn part2(passwords: Vec<&str>) -> usize {
    let re = Regex::new(RE_STR).unwrap();

    passwords.iter().filter(|entry| {
        let captures = re.captures(entry).expect("Line failed to match");

        let position0: usize = captures.name("param0").map(|min_str| min_str.as_str().parse::<usize>().unwrap()).unwrap() - 1;
        let position1: usize = captures.name("param1").map(|max_str| max_str.as_str().parse::<usize>().unwrap()).unwrap() - 1;
        // We can turn these into byte arrays since our input is all in ascii range
        let character = captures.name("character").unwrap().as_str().as_bytes()[0];
        let password = captures.name("password").unwrap().as_str().as_bytes();

        (password[position0] == character) != (password[position1] == character)
    }).count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    println!("Part 1: {}", part1(contents.lines().collect()));
    println!("Part 2: {}", part2(contents.lines().collect()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        let sample = vec![
            "1-3 a: abcde",
            "1-3 b: cdefg",
            "2-9 c: ccccccccc",
        ];

        assert_eq!(part1(sample), 2);
    }

    #[test]
    fn part2_example() {
        let sample = vec![
            "1-3 a: abcde",
            "1-3 b: cdefg",
            "2-9 c: ccccccccc",
        ];

        assert_eq!(part2(sample), 1);
    }
}
