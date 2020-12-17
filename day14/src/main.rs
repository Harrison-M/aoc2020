use regex::Regex;
use std::{collections::HashMap, env, fs};

const INSTRUCTION_RE_STR: &str = r"mem\[(\d+)\] = (\d+)";
const MASK_PREFIX: &str = "mask = ";

fn part1<'a>(lines: impl Iterator<Item = &'a str>) -> usize {
    let instruction_re = Regex::new(INSTRUCTION_RE_STR).unwrap();
    let mut mask: usize = 0;
    let mut mask_value: usize = 0;
    let mut memory: HashMap<usize, usize> = HashMap::new();
    for line in lines {
        if let Some(mask_str) = line.strip_prefix(MASK_PREFIX) {
            mask = usize::from_str_radix(
                &mask_str.replace('1', "0").replace('X', "1")[..],
                2
            ).unwrap();
            mask_value = usize::from_str_radix(
                &mask_str.replace('X', "0")[..],
                2
            ).unwrap();
        } else {
            let captures = instruction_re.captures(line).unwrap();
            let location: usize = captures
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            let value: usize = captures
                .get(2)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();

            memory.insert(location, value & mask | mask_value);
        }
    }

    memory.values().sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    println!("Part 1: {}", part1(contents.lines()));
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = include_str!("sample");

    #[test]
    fn part1_example() {
        assert_eq!(part1(SAMPLE.lines()), 165);
    }
}
