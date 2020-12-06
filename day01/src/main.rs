use std::env;
use std::fs;

const TARGET_SUM: u32 = 2020;

fn parse_numbers<'a>(num_strs: impl Iterator<Item=&'a str>) -> Vec<u32> {
    num_strs.map(|line| line.parse::<u32>().expect("Failed to parse number")).collect()
}

/// Given a series of numbers, find two that add up to target
fn part1<'a>(numbers: &mut Vec<u32>, target: u32) -> Option<(u32, u32)> {
    numbers.sort_unstable(); // Will reduce number of iterations needed

    return loop {
        let num1 = match numbers.pop() {
            Some(n) => n,
            None => break None
        };

        let mut num_iter = numbers.iter();

        let opt_num2 = loop {
            if let Some(candidate) = num_iter.next() {
                let sum = num1 + candidate;

                if sum == target { // Found it
                    break Some(candidate)
                } else if sum > target { // The rest will also be too large
                    break None
                }
            } else {
                break None
            }
        };

        if let Some(num2) = opt_num2 {
            break Some((num1, *num2))
        }
    }
}

/// As part1, but for three numbers and TARGET_SUM
fn part2<'a>(numbers: &mut Vec<u32>) -> Option<(u32, u32, u32)> {
    numbers.sort_unstable();

    return loop {
        let num1 = match numbers.pop() {
            Some(n) => n,
            None => break None
        };

        // We aren't provided any numbers larger than TARGET_SUM
        let opt_nums = part1(&mut numbers.clone(), TARGET_SUM - num1);

        if let Some((num2, num3)) = opt_nums {
            break Some((num1, num2, num3))
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");

    let mut numbers = parse_numbers(contents.lines());

    let (part1_num1, part1_num2) = part1(&mut numbers.clone(), TARGET_SUM).expect("No part 1 result found");
    println!("Part 1: {}", part1_num1 * part1_num2);

    let (part2_num1, part2_num2, part2_num3) = part2(&mut numbers).expect("No part 2 result found");
    println!("Part 2: {}", part2_num1 * part2_num2 * part2_num3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_matches() {
        // Sample list from prompt
        let mut sample = vec![
            1721,
            979,
            366,
            299,
            675,
            1456,
        ];

        let (num1, num2) = part1(&mut sample, TARGET_SUM).unwrap();
        assert_eq!(num1 * num2, 514579);
    }

    #[test]
    fn part2_example_matches() {
        let mut sample = vec![
            1721,
            979,
            366,
            299,
            675,
            1456,
        ];

        let (num1, num2, num3) = part2(&mut sample).unwrap();
        assert_eq!(num1 * num2 * num3, 241861950);
    }
}
