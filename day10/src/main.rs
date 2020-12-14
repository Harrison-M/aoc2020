use std::{env, fs, collections::HashMap};

fn part1(adapters: &Vec<usize>) -> usize {
    let mut all_adapters = adapters.clone();
    all_adapters.push(0); // Account for charging outlet
    all_adapters.sort_unstable();

    let diff_iter = all_adapters
        .windows(2)
        .map(|window| window.get(1).unwrap() - window.get(0).unwrap());

    let mut frequencies: HashMap<usize, usize> = HashMap::new();
    for diff in diff_iter {
        frequencies
            .entry(diff)
            .and_modify(|f| *f += 1)
            .or_insert(1);
    }

    frequencies
        .get(&1)
        .copied()
        .unwrap_or(0) *
        (frequencies
            .get(&3)
            .copied()
            .unwrap_or(0)
            + 1) // Include the device
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let numbers: Vec<usize> = contents.lines().map(|l| l.parse().unwrap()).collect();
    println!("Part 1: {}", part1(&numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        let sample1 = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let sample2 = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49,
                           45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4,
                           2, 34, 10, 3];

        assert_eq!(part1(&sample1), 35);
        assert_eq!(part1(&sample2), 220);
    }
}
