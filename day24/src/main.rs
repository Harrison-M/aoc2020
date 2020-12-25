use point_2d::Point2D;
use regex::Regex;
use std::{env, fs, collections::HashSet};

const DIR_RE_STR: &str = r"[ns]?[ew]";

type Tile = Point2D<isize>;

/// Get the 2D coordinate motion related with a direction.
/// We can accomplish this by viewing the axes as as
/// (SW->NE diagonal, W->E row). I've included my notebook
/// doodle of this at the root of day24.
fn get_motion(direction: &str) -> (isize, isize) {
    match direction {
        "nw" => (-1, 1),
        "ne" => (0, 1),
        "w" => (-1, 0),
        "e" => (1, 0),
        "sw" => (0, -1),
        "se" => (1, -1),
        _ => panic!("Invalid direction {}", direction),
    }
}

fn part1(directions: &str) -> usize {
    let dir_re = Regex::new(DIR_RE_STR).unwrap();
    let mut black_tiles: HashSet<Tile> = HashSet::new();
    for direction_set in directions.lines() {
        let mut current_tile = Point2D(0, 0);
        for direction in dir_re.find_iter(direction_set) {
            current_tile += get_motion(direction.as_str());
        }
        if !black_tiles.remove(&current_tile) {
            black_tiles.insert(current_tile);
        }
    }
    black_tiles.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    println!("Part 1: {}", part1(&contents));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        let sample = include_str!("sample");
        assert_eq!(part1(sample), 10);
    }
}
