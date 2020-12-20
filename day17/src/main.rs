mod cell;
mod hypercell;
mod relative;

use crate::cell::Cell;
use crate::hypercell::HyperCell;
use crate::relative::Relative;
use std::{collections::{HashSet, HashMap}, env, fs, hash::Hash};

type AdjacencyCache<T> = HashMap<T, Vec<T>>;

struct ConwayField<T: Eq + Hash + Relative> {
    adjacency_map: AdjacencyCache<T>,
    active_cells: HashSet<T>,
}

fn cached_get_adjacent<'a, T>(adjacency_map: &'a mut AdjacencyCache<T>, cell: &T) -> &'a Vec<T> 
    where T: Copy + Eq + Hash + Relative{
    adjacency_map.entry(*cell)
        .or_insert_with(|| cell.adjacent())
}

impl<T: Copy + Eq + Hash + Relative> ConwayField<T> {
    fn step(&mut self) {
        let mut adjacency_counts: HashMap<T, u8> = HashMap::new();
        for active_cell in self.active_cells.iter() {
            for adjacent_cell in cached_get_adjacent(&mut self.adjacency_map, active_cell) {
                adjacency_counts.entry(*adjacent_cell)
                    .and_modify(|n| *n += 1)
                    .or_insert(1);
            }
        }

        self.active_cells.retain(|cell|
            (2..=3).contains(adjacency_counts.get(cell).unwrap_or(&0))
        );

        for (cell, count) in adjacency_counts {
            if count == 3 {
                self.active_cells.insert(cell);
            }
        }
    }
}

impl<T> From<&str> for ConwayField<T>
    where T: Eq + From<(isize, isize)> + Hash + Relative {
    fn from(grid: &str) -> Self {
        let active_cubes = grid
            .lines()
            .enumerate()
            .flat_map(|(y, line)|
                line
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(x, _)| (x as isize, y as isize).into())
            ).collect();
        Self { adjacency_map: HashMap::new(), active_cells: active_cubes }
    }
}

fn part1(input: &'_ str) -> usize {
    let mut field: ConwayField<Cell> = input.into();
    for _ in 0..6 {
        field.step();
    }

    field.active_cells.len()
}

fn part2(input: &'_ str) -> usize {
    let mut field: ConwayField<HyperCell> = input.into();
    for _ in 0..6 {
        field.step();
    }

    field.active_cells.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = include_str!("sample");

    #[test]
    fn part1_example() {
        assert_eq!(part1(SAMPLE), 112);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(SAMPLE), 848);
    }
}
