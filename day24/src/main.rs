use point_2d::Point2D;
use regex::Regex;
use std::{env, fs, collections::{HashMap, HashSet}};

const DIR_RE_STR: &str = r"[ns]?[ew]";

type Tile = Point2D<isize>;
type AdjacencyCache = HashMap<Tile, Vec<Tile>>;

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

fn hexagonally_adjacent(tile: &Tile) -> Vec<Tile> {
    vec![(-1, 1), (0, 1), (-1, 0), (1, 0), (0, -1), (1, -1)]
        .into_iter()
        .map(|motion| tile + motion)
        .collect()
}

fn cached_get_adjacent<'a>(adjacency_map: &'a mut AdjacencyCache, tile: &Tile) -> &'a Vec<Tile> {
    adjacency_map.entry(*tile)
        .or_insert_with(|| hexagonally_adjacent(tile))
}

fn part1(directions: &str) -> HashSet<Tile> {
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
    black_tiles
}

fn part2(mut black_tiles: HashSet<Tile>) -> usize {
    // Largely copied from day 17. Could probably make a library for this.
    let mut adjacency_cache: AdjacencyCache = HashMap::new();
    for _ in 0..100 {
        let mut adjacency_counts: HashMap<Tile, u8> = HashMap::new();
        for black_tile in black_tiles.iter() {
            for adjacent_cell in cached_get_adjacent(&mut adjacency_cache, black_tile) {
                adjacency_counts.entry(*adjacent_cell)
                    .and_modify(|n| *n += 1)
                    .or_insert(1);
            }
        }

        black_tiles.retain(|tile|
            (1..=2).contains(adjacency_counts.get(tile).unwrap_or(&0))
        );

        for (tile, count) in adjacency_counts {
            if count == 2 {
                black_tiles.insert(tile);
            }
        }
    }

    black_tiles.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let init_black_tiles = part1(&contents);
    println!("Part 1: {}", init_black_tiles.len());
    println!("Part 2: {}", part2(init_black_tiles));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let sample = include_str!("sample");
        let init_black_tiles = part1(sample);
        assert_eq!(init_black_tiles.len(), 10);
        assert_eq!(part2(init_black_tiles), 2208);
    }
}
