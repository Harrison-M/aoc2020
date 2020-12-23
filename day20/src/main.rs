use std::{collections::{HashSet, HashMap}, env, fs, fmt::Display, ops::Deref};

/// Utilty structure for printing a byte collection as ascii text
struct AsciiRowDisplay<T: Deref<Target = [u8]>>(T);

impl<T: Deref<Target = [u8]>> Display for AsciiRowDisplay<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in self.0.iter() {
            write!(f, "{}", *byte as char)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Tile<'a> {
    id: usize,
    byte_lines: Vec<&'a[u8]>,
    borders: Vec<Vec<u8>>,
}

/// Displays a tile in a format call as a tile's id and borders
/// Unnecessary for solution, but useful for debugging
impl<'a> Display for Tile<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tile {}\nBorders:", self.id)?;
        for border in self.borders.iter() {
            write!(f, "\n{}", AsciiRowDisplay(&border[..]))?;
        }
        Ok(())
    }
}

impl<'a> Tile<'a> {
    fn from_spec(spec: &'a str) -> Self {
        let mut line_iter = spec.lines();
        let id: usize = line_iter
            .next()
            .unwrap()
            .trim_start_matches("Tile ")
            .trim_end_matches(":")
            .parse()
            .unwrap();
        let mut byte_lines: Vec<&[u8]> = line_iter.map(|l| l.as_bytes()).collect();
        let text_grid = &mut byte_lines[..];

        // Borders
        let top: Vec<_> = text_grid.first().unwrap().to_owned().into();
        let right: Vec<_> = text_grid
            .iter()
            .map(|l| l.last().unwrap())
            .copied()
            .collect();
        let mut bottom: Vec<_> = text_grid.last().unwrap().to_owned().into();
        bottom.reverse();
        let left: Vec<_> = text_grid
            .iter()
            .rev()
            .map(|l| l.first().unwrap())
            .copied()
            .collect();
        Self { id, byte_lines, borders: vec![top, right, bottom, left] }
    }
}

type BorderMap<'a, 'b> = HashMap<Vec<u8>, HashSet<&'b Tile<'a>>>;

/// Produces a map of borders to the tiles they can be found on.
fn make_border_finder<'a, 'b>(tiles: &'b Vec<Tile<'a>>) -> BorderMap<'a, 'b> {
    let mut finder: HashMap<Vec<u8>, HashSet<&Tile>> = HashMap::new();
    for tile in tiles {
        for border in tile.borders.iter() {
            finder
                .entry(border.clone())
                .or_insert_with(|| HashSet::new())
                .insert(tile);
        }
    }
    finder
}

/// Find the corner pieces to answer part 1.
/// The phrase "the outermost edges won't line up with any other tiles" means
/// we can use this as a guaranteed corner check
fn part1(tiles: &Vec<Tile>, finder: &BorderMap) -> usize {
    tiles
        .iter()
        .filter(|tile| {
            tile.borders
                .iter()
                .filter(|&border| {
                    let mut rev = border.clone();
                    rev.reverse();
                    finder
                        .get(border)
                        .unwrap_or(&HashSet::new())
                        .union(
                            finder.get(&rev)
                            .unwrap_or(&HashSet::new())
                        )
                        .filter(|&t| tile != t)
                        .next()
                        .is_none()
                })
                .count() == 2
        })
        .map(|tile| tile.id)
        .product()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let tiles: Vec<_> = contents
        .trim()
        .split("\n\n")
        .map(|spec| Tile::from_spec(spec))
        .collect();
    let finder = make_border_finder(&tiles);

    println!("Part 1: {}", part1(&tiles, &finder));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        let sample = include_str!("sample");
        let tiles: Vec<_> = sample
            .trim()
            .split("\n\n")
            .map(|spec| Tile::from_spec(spec))
            .collect();
        let finder = make_border_finder(&tiles);

        assert_eq!(part1(&tiles, &finder), 20899048083289);
    }
}
