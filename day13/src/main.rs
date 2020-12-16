use itertools::Itertools;
use std::{env, iter, fs};

/// A parsed puzzle input
struct ProblemSpec {
    arrival_time: usize,
    buses: Vec<usize>,
}

impl ProblemSpec {
    fn from_string(input: String) -> Self {
        let mut lines = input.lines();
        let arrival_time = lines.next().unwrap().parse().unwrap();
        let buses = lines
            .next()
            .unwrap()
            .split(',')
            .filter_map(|bus| bus.parse().ok())
            .collect();
        Self { arrival_time, buses }
    }
}

/// Find which bus we'll take and how long we'll have to wait
fn part1(spec: &ProblemSpec) -> usize {
    let (bus, time) = spec.buses.iter()
        .map(|&bus| iter::repeat(bus).zip((0usize..).step_by(bus)))
        .kmerge_by(|&(_, time1), &(_, time2)| time1 < time2)
        .find(|&(_, time)| time >= spec.arrival_time)
        .unwrap();

    bus * (time - spec.arrival_time)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let spec = ProblemSpec::from_string(contents);
    println!("Part 1: {}", part1(&spec));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let sample = "939\n7,13,x,x,59,x,31,19";
        let spec = ProblemSpec::from_string(sample.to_string());
        assert_eq!(part1(&spec), 295);
    }
}
