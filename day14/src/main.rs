use regex::Regex;
use std::{collections::HashMap, env, fs};

const INSTRUCTION_RE_STR: &str = r"mem\[(\d+)\] = (\d+)";
const MASK_PREFIX: &str = "mask = ";

enum BinaryLookup<T: Copy> {
    Fork(Option<Box<BinaryLookup<T>>>, Option<Box<BinaryLookup<T>>>),
    Value(T),
}

impl<T: Copy> BinaryLookup<T> {
    fn new() -> Self {
        BinaryLookup::<T>::Fork(None, None)
    }

    fn boxed_new() -> Box<Self> {
        Box::new(Self::new())
    }

    fn set_at_addr(&mut self, address: &str, mask: &str, val: T) {
        match self {
            Self::Value(_) => panic!("Cannot perform lookup on a Value"),
            Self::Fork(zero, one) => match (address.split_at(1), mask.split_at(1)) {
                ((_, ""), ("X", "")) => {
                    zero.replace(Box::new(Self::Value(val)));
                    one.replace(Box::new(Self::Value(val)));
                }
                (("0", ""), ("0", "")) => { zero.replace(Box::new(Self::Value(val))); }
                ((_, ""), (_, "")) => { one.replace(Box::new(Self::Value(val))); }
                ((_, raddress), ("X", rmask)) => {
                    zero
                        .get_or_insert_with(Self::boxed_new)
                        .set_at_addr(raddress, rmask, val);
                    one
                        .get_or_insert_with(Self::boxed_new)
                        .set_at_addr(raddress, rmask, val);
                }
                (("0", raddress), ("0", rmask)) => {
                    zero
                        .get_or_insert_with(Self::boxed_new)
                        .set_at_addr(raddress, rmask, val);
                }
                ((_, raddress), (_, rmask)) => {
                    one
                        .get_or_insert_with(Self::boxed_new)
                        .set_at_addr(raddress, rmask, val);
                }
            }
        }
    }

    fn walk(&self) -> Vec<&T> {
        match self {
            Self::Value(val) => vec![val],
            Self::Fork(zero, one) => zero
                .as_ref()
                .map_or_else(|| vec![], |t| t.walk())
                .iter()
                .chain(
                    one
                        .as_ref()
                        .map_or_else(|| vec![], |t| t.walk())
                        .iter()
                )
                .copied()
                .collect()
        }
    }
}

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

fn part2<'a>(lines: impl Iterator<Item = &'a str>) -> usize {
    let instruction_re = Regex::new(INSTRUCTION_RE_STR).unwrap();
    let mut mask: &str = "";
    let mut memory: BinaryLookup<usize> = BinaryLookup::new();

    for line in lines {
        if let Some(new_mask) = line.strip_prefix(MASK_PREFIX) {
            mask = new_mask;
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

            let address = format!("{:036b}", location);
            memory.set_at_addr(address.as_str(), mask, value);
        }
    }

    memory.walk().into_iter().sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    println!("Part 1: {}", part1(contents.lines()));
    println!("Part 2: {}", part2(contents.lines()));
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = include_str!("sample");
    const SAMPLE2: &str = include_str!("sample2");

    #[test]
    fn part1_example() {
        assert_eq!(part1(SAMPLE.lines()), 165);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(SAMPLE2.lines()), 208);
    }
}
