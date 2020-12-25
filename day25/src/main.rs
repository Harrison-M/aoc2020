use std::{env, iter::successors};

fn part1(card_key: usize, door_key: usize) -> usize {
    let get_subject_iter = |subject|
        successors(Some(1usize), move |n| Some(n * subject % 20201227));

    let (loop_count, first_match) = get_subject_iter(7)
        .enumerate()
        .find(|&(_, n)| n == card_key || n == door_key)
        .unwrap();

    if first_match == card_key {
        get_subject_iter(door_key)
            .nth(loop_count)
            .unwrap()
    } else {
        get_subject_iter(card_key)
            .nth(loop_count)
            .unwrap()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let card_key: usize = args[1].parse().unwrap();
    let door_key: usize = args[2].parse().unwrap();

    println!("Part 1: {}", part1(card_key, door_key));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(5764801, 17807724), 14897079);
    }
}
