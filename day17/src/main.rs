use itertools::iproduct;
use std::{collections::{HashSet, HashMap}, env, fs, ops::Add};

type CellTuple = (isize, isize, isize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Cell(isize, isize, isize);

impl Cell {
    fn adjacent(&self) -> Vec<Cell> {
        iproduct!(-1isize..=1, -1isize..=1, -1isize..=1)
            .filter(|ct| *ct != (0, 0, 0))
            .map(|ct| *self + ct)
            .collect()
    }
}

impl From<CellTuple> for Cell {
    fn from((x, y, z): (isize, isize, isize)) -> Self {
        Self(x, y, z)
    }
}

impl Add for Cell {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Cell(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Add<CellTuple> for Cell {
    type Output = Self;

    fn add(self, rhs: CellTuple) -> Self::Output {
        Cell(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

type AdjacencyCache = HashMap<Cell, Vec<Cell>>;

struct ConwayField {
    adjacency_map: AdjacencyCache,
    active_cells: HashSet<Cell>,
}

fn cached_get_adjacent<'a>(adjacency_map: &'a mut AdjacencyCache, cell: &Cell)
    -> &'a Vec<Cell> {
    adjacency_map.entry(*cell)
        .or_insert_with(|| cell.adjacent())
}

impl ConwayField {
    fn step(&mut self) {
        let mut adjacency_counts: HashMap<Cell, u8> = HashMap::new();
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

impl From<&str> for ConwayField {
    fn from(grid: &str) -> Self {
        let active_cubes = grid
            .lines()
            .enumerate()
            .flat_map(|(y, line)|
                line
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(x, _)| Cell(x as isize, y as isize, 0))
            ).collect();
        Self { adjacency_map: HashMap::new(), active_cells: active_cubes }
    }
}

fn part1(input: &'_ str) -> usize {
    let mut field: ConwayField = input.into();
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
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = include_str!("sample");

    #[test]
    fn part1_example() {
        assert_eq!(part1(SAMPLE), 112);
    }
}
