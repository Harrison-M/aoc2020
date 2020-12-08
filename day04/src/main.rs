use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;

const SPLIT_RE_STR: &str = r"\n\n";
const DATA_RE_STR: &str = r"(\w+):(\S+)";

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let profiles = get_profiles(&contents);

    println!("Part 1: {}", part1(&profiles));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let database = include_str!("sample").to_string();
        let profiles = get_profiles(&database);
        println!("{:#?}", profiles);
        assert_eq!(part1(&profiles), 2);
    }
}
