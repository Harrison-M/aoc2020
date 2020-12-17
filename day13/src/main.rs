use itertools::Itertools;
use modinverse::modinverse;
use std::{env, iter, fs};

/// A parsed puzzle input
#[derive(Debug)]
struct ProblemSpec {
    arrival_time: i128,
    buses: Vec<(i128, i128)>,
}

impl ProblemSpec {
    fn from_string(input: String) -> Self {
        let mut lines = input.lines();
        let arrival_time = lines.next().unwrap().parse().unwrap();
        let buses = lines
            .next()
            .unwrap()
            .split(',')
            .enumerate()
            .filter_map(|(i, bus)| bus.parse().ok().map(|bus| (i as i128, bus)))
            .collect();
        Self { arrival_time, buses }
    }
}

/// Find which bus we'll take and how long we'll have to wait
fn part1(spec: &ProblemSpec) -> i128 {
    let (bus, time) = spec.buses
        .iter()
        .map(|&(_, bus)| iter::repeat(bus).zip((0i128..).step_by(bus as usize)))
        .kmerge_by(|&(_, time1), &(_, time2)| time1 < time2)
        .find(|&(_, time)| time >= spec.arrival_time)
        .unwrap();

    bus * (time - spec.arrival_time)
}

fn remainder_theorem(rem1: i128, mod1: i128, rem2: i128, mod2: i128) -> i128 {
    (rem2 * modinverse(mod1, mod2).unwrap() * mod1 +
        rem1 * modinverse(mod2, mod1).unwrap() * mod2) % (mod1 * mod2)
}

/// Find the time when buses will start leaving in time offsets according to their
/// position in the schedule. Since this can be expressed as time % bus_number = index,
/// and all bus numbers are prime, we can use the Chinese Remainder Theorem to find a
/// combined remainder and modulus, then use those to find a time that works in all the
/// modular spaces. See "Chinese Remainder Theorem" in
/// https://en.wikipedia.org/wiki/Modular_arithmetic#Properties
fn part2(spec: &ProblemSpec) -> i128 {
    let (rem, modulo) = spec.buses
        .iter()
        .copied()
        .fold1(|(rem1, mod1), (rem2, mod2)|
            (remainder_theorem(rem1, mod1, rem2, mod2), mod1 * mod2))
        .unwrap();
    modulo - rem
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let spec = ProblemSpec::from_string(contents);
    println!("Part 1: {}", part1(&spec));
    println!("Part 2: {}", part2(&spec));
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

    #[test]
    fn part2_example() {
        let samples = vec![
            "939\n7,13,x,x,59,x,31,19",
            "0\n17,x,13,19",
            "0\n67,7,59,61",
            "0\n67,x,7,59,61",
            "0\n67,7,x,59,61",
            "0\n1789,37,47,1889",
        ];

        let specs = samples
            .into_iter()
            .map(String::from)
            .map(ProblemSpec::from_string);

        let expectations = vec![
            1068781,
            3417,
            754018,
            779210,
            1261476,
            1202161486,
        ];

        for (spec, expectation) in specs.zip(expectations) {
            assert_eq!(part2(&spec), expectation);
        }
    }
}
