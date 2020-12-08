use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

const COLOR_RE_STR: &str = r"^#[0-9a-f]{6}$";
const DATA_RE_STR: &str = r"(\w+):(\S+)";
const HEIGHT_RE_STR: &str = r"^(\d+)(cm|in)$";
const PASSPORT_RE_STR: &str = r"^\d{9}$";
const SPLIT_RE_STR: &str = r"\n\n";

type Profiles<'a> = Vec<HashMap<&'a str, &'a str>>;

/// From a traveler database String, create a vector of profiles (HashMaps)
fn get_profiles<'a>(database: &'a String) -> Profiles<'a> {
    let split_re = Regex::new(SPLIT_RE_STR).unwrap();
    let data_re = Regex::new(DATA_RE_STR).unwrap();

    split_re.split(&database[..]).map(|profile_str| {
        let mut profile: HashMap<&str, &str> = HashMap::new();

        for attr in data_re.captures_iter(profile_str) {
            profile.insert(
                attr.get(1).unwrap().as_str(),
                attr.get(2).unwrap().as_str(),
            );
        }

        profile
    }).collect()
}

/// Count valid passports in the profile list
fn part1(profiles: &Profiles) -> usize {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    profiles.iter().filter(|profile|
        required_fields.iter().all(|key| profile.contains_key(key))
    ).count()
}

fn opt_parse(num_s: &&str) -> Option<usize> {
    num_s.parse().map_or(None, |num| Some(num))
}

fn part2(profiles: &Profiles) -> usize {
    let height_re = Regex::new(HEIGHT_RE_STR).unwrap();
    let color_re = Regex::new(COLOR_RE_STR).unwrap();
    let passport_re = Regex::new(PASSPORT_RE_STR).unwrap();
    let mut valid_eye_colors: HashSet<&str> = HashSet::new();
    for color in vec![
        "amb",
        "blu",
        "brn",
        "gry",
        "grn",
        "hzl",
        "oth",
    ].into_iter() {
        valid_eye_colors.insert(color);
    }

    profiles.iter().filter(|profile| {
        profile.get("byr")
            .and_then(opt_parse)
            .map_or(false, |val| val >= 1920 && val <= 2002) &&
        profile.get("iyr")
            .and_then(opt_parse)
            .map_or(false, |val| val >= 2010 && val <= 2020) &&
        profile.get("eyr")
            .and_then(opt_parse)
            .map_or(false, |val| val >= 2020 && val <= 2030) &&
        profile.get("hgt")
            .and_then(|sval| height_re.captures(sval))
            .and_then(|captures| captures.get(2).zip(captures.get(1)))
            .and_then(|(unit, num_s)| opt_parse(&num_s.as_str())
                .map(|num| match unit.as_str() {
                    "cm" => num >= 150 && num <= 193,
                    "in" => num >= 59 && num <= 76,
                    _ => panic!("Regex should have missed"),
                })
            ).unwrap_or(false) &&
        profile.get("hcl")
            .map_or(false, |hcl| color_re.is_match(hcl)) &&
        profile.get("ecl")
            .map_or(false, |sval| valid_eye_colors.contains(sval)) &&
        profile.get("pid")
            .map_or(false, |pid| passport_re.is_match(pid))
    }).count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let profiles = get_profiles(&contents);

    println!("Part 1: {}", part1(&profiles));
    println!("Part 2: {}", part2(&profiles));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let database = include_str!("sample").to_string();
        let profiles = get_profiles(&database);
        assert_eq!(part1(&profiles), 2);
    }

    #[test]
    fn part2_example() {
        let database = include_str!("sample2").to_string();
        let profiles = get_profiles(&database);
        assert_eq!(part2(&profiles), 4);
    }
}
