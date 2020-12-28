use ndarray::{Axis, Array2, Array1, concatenate, s};
use std::{collections::{HashSet, HashMap}, env, fs, fmt::Display, mem::swap, rc::Rc, cell::RefCell};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Edge {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Tile {
    id: usize,
    image: Array2<u8>,
    height: usize,
    width: usize,
    up: Option<(usize, Edge)>,
    right: Option<(usize, Edge)>,
    down: Option<(usize, Edge)>,
    left: Option<(usize, Edge)>,
}

use Edge::*;

#[derive(Debug)]
struct Borders {
    top: Array1<u8>,
    right: Array1<u8>,
    bottom: Array1<u8>,
    left: Array1<u8>,
}

impl Borders {
    fn iter(&self) -> impl Iterator<Item=(&Array1<u8>, Edge)> {
        vec![(&self.top, Top), (&self.right, Right), (&self.bottom, Bottom), (&self.left, Left)].into_iter()
    }
}

/// Rotate an array 90 degrees counter-clockwise, shout out to
/// https://github.com/rust-ndarray/ndarray/issues/866#issuecomment-748724635
fn rot90ccw(arr: &mut Array2<u8>) {
    arr.swap_axes(0, 1);
    arr.invert_axis(Axis(0));
}

/// Displays a tile in a format call as a tile's id and borders
/// Unnecessary for solution, but useful for debugging
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tile {}\nBorders:", self.id)?;
        for border in self.borders().iter() {
            write!(f, "\n")?;
            for c in border.0.iter().map(|&b| b as char) {
                write!(f, "{}", c)?;
            }
        }
        Ok(())
    }
}

impl Tile {
    fn from_spec(spec: &str) -> Self {
        let mut line_iter = spec.lines();
        let id: usize = line_iter
            .next()
            .unwrap()
            .trim_start_matches("Tile ")
            .trim_end_matches(":")
            .parse()
            .unwrap();
        let byte_lines: Vec<&[u8]> = line_iter.map(|l| l.as_bytes()).collect();
        let width = byte_lines.first().unwrap().len();
        let height = byte_lines.len();
        let bytes: Vec<u8> = byte_lines.into_iter().flatten().copied().collect();
        let image = Array2::from_shape_vec((width, height), bytes).unwrap();

        Self { id, image, height, width, up: None, right: None, down: None, left: None }
    }

    fn borders(&self) -> Borders {
        let top = self.image.row(0).into_owned();
        let right = self.image.column(self.width - 1).into_owned();
        let bottom = self.image.row(self.height - 1).into_owned();
        let left = self.image.column(0).into_owned();

        Borders { top, right, bottom, left }
    }

    fn rot90ccw(&mut self)
    {
        rot90ccw(&mut self.image);
        swap(&mut self.left, &mut self.up);
        swap(&mut self.up, &mut self.right);
        swap(&mut self.right, &mut self.down);
    }

    fn flip_horizontal(&mut self) {
        self.image.invert_axis(Axis(1));
        swap(&mut self.left, &mut self.right);
    }

    fn flip_vertical(&mut self) {
        self.image.invert_axis(Axis(0));
        swap(&mut self.up, &mut self.down);
    }
}

type BorderMap = HashMap<Array1<u8>, HashSet<(usize, Edge)>>;

/// Produces a map of borders to the tiles they can be found on.
fn make_border_finder(tiles: &HashMap<usize, Rc<RefCell<Tile>>>) -> BorderMap {
    let mut finder: BorderMap = HashMap::new();
    for tile in tiles.values() {
        for (border, edge) in tile.borrow().borders().iter() {
            finder
                .entry(border.clone())
                .or_insert_with(|| HashSet::new())
                .insert((tile.borrow().id, edge));
        }
    }
    finder
}

/// Find the tile id (if any) that matches the given border
fn find_match<'a>(cur_id: usize, borders: &Borders, edge: Edge, finder: &BorderMap) -> Option<(usize, Edge)> {
    let border = match edge {
        Top => &borders.top,
        Right => &borders.right,
        Bottom => &borders.bottom,
        Left => &borders.left,
    };
    let mut rev = border.clone();
    rev.invert_axis(Axis(0));
    finder
        .get(&border)
        .unwrap_or(&HashSet::new())
        .union(
            finder.get(&rev)
            .unwrap_or(&HashSet::new())
        )
        .filter(|&&(found_tile_id, _)| found_tile_id != cur_id)
        .copied()
        .next()
}

/// Use the knowledge that each border in my (our?) input has at most one match
/// to put together the image. A general solution would be more complex.
fn connect_tiles<'a>(tiles: &mut HashMap<usize, Rc<RefCell<Tile>>>, finder: &BorderMap) -> Array2<u8> {
    for tile_cell in tiles.values() {
        let mut tile = tile_cell.borrow_mut();
        let borders = tile.borders();
        tile.up = find_match(tile.id, &borders, Top, finder);
        tile.right = find_match(tile.id, &borders, Right, finder);
        tile.down = find_match(tile.id, &borders, Bottom, finder);
        tile.left = find_match(tile.id, &borders, Left, finder);
    }

    // Find the top-left corner and use it as our first row starter
    let top_left = tiles
        .values()
        .find(|tile_cell| {
            let tile = tile_cell.borrow();
            tile.left.is_none() && tile.up.is_none()
        })
        .unwrap();

    let mut row_start_cell = top_left;
    let mut current = row_start_cell.borrow_mut();

    loop {
        while let Some((id, edge)) = current.right {
            let mut next = tiles.get(&id).unwrap().borrow_mut();
            let rotations = match edge {
                Left => 0,
                Top => 1,
                Right => 2,
                Bottom => 3,
            };

            for _ in 0..rotations {
                next.rot90ccw();
            }
            let right = current.borders().right;
            let next_left = next.borders().left;
            if right != next_left {
                // Needs to be flipped upside down
                next.flip_vertical();
            }
            current = next;
        }

        if let Some((id, edge)) = row_start_cell.borrow().down {
            let next_row_start_cell = tiles.get(&id).unwrap();
            let mut next_row_start = next_row_start_cell.borrow_mut();
            let rotations = match edge {
                Top => 0,
                Right => 1,
                Bottom => 2,
                Left => 3,
            };

            for _ in 0..rotations {
                next_row_start.rot90ccw();
            }

            let bottom = row_start_cell.borrow().borders().bottom;
            let next_row_top = next_row_start.borders().top;
            if bottom != next_row_top {
                // Needs to be flipped horizontally
                next_row_start.flip_horizontal();
            }
            row_start_cell = next_row_start_cell;
            current = next_row_start;
        } else {
            break;
        }
    }

    // Assemble image
    let trim = s![1..-1, 1..-1];
    let mut image_opt: Option<Array2<u8>> = None;
    row_start_cell = top_left;
    current = row_start_cell.borrow_mut();
    loop {
        let mut row: Array2<u8> = current.image.slice(trim).into_owned();
        while let Some((id, _)) = current.right {
            let next = tiles.get(&id).unwrap().borrow_mut();
            row = concatenate![Axis(1), row, next.image.slice(trim)];
            current = next;
        }

        if let Some(image) = image_opt {
            image_opt = Some(concatenate![Axis(0), image, row]);
        } else {
            image_opt = Some(row);
        }

        if let Some((id, _)) = row_start_cell.borrow().down {
            row_start_cell = tiles.get(&id).unwrap();
            current = row_start_cell.borrow_mut();
        } else {
            break;
        }
    }

    image_opt.unwrap()
}

/// Find the corner pieces to answer part 1.
/// The phrase "the outermost edges won't line up with any other tiles" means
/// we can use this as a guaranteed corner check
fn part1(tiles: &HashMap<usize, Rc<RefCell<Tile>>>, finder: &BorderMap) -> usize {
    tiles
        .values()
        .filter(|tile| {
            tile.borrow().borders()
                .iter()
                .filter(|&(border, _)| {
                    let mut rev = border.clone();
                    rev.invert_axis(Axis(0));
                    finder
                        .get(border)
                        .unwrap_or(&HashSet::new())
                        .union(
                            finder.get(&rev)
                            .unwrap_or(&HashSet::new())
                        )
                        .filter(|&(t, _)| tile.borrow().id != *t)
                        .next()
                        .is_none()
                })
                .count() == 2
        })
        .map(|tile| tile.borrow().id)
        .product()
}

fn check_array(image: &Array2<u8>) -> Option<usize> {
    let monsters = image.windows((3, 20))
        .into_iter()
        .filter(|w|
            vec![
                w[[0, 18]],
                w[[1, 0]],
                w[[1, 5]],
                w[[1, 6]],
                w[[1, 11]],
                w[[1, 12]],
                w[[1, 17]],
                w[[1, 18]],
                w[[1, 19]],
                w[[2, 1]],
                w[[2, 4]],
                w[[2, 7]],
                w[[2, 10]],
                w[[2, 13]],
                w[[2, 16]],
            ].into_iter()
            .all(|b| b == b'#')
        )
        .count();
    if monsters > 0 {
        let char_image = image.mapv(|byte| byte as char);
        for row in char_image.genrows() {
            for c in row {
                print!("{}", c);
            }
            print!("\n");
        }
        Some(image.iter().filter(|&&b| b == b'#').count() - monsters * 15)
    } else {
        None
    }
}

fn check_array_rotations(image: &mut Array2<u8>) -> Option<usize> {
    let mut rotations = 0;
    loop {
        let check = check_array(&image);
        if check.is_some() {
            break check;
        }
        rot90ccw(image);
        rotations += 1;
        if rotations == 4 {
            break None;
        }
    }
}

fn part2(tiles: &mut HashMap<usize, Rc<RefCell<Tile>>>, finder: &BorderMap) -> usize {
    let mut image = connect_tiles(tiles, finder);
    if let Some(result) = check_array_rotations(&mut image) {
        result
    } else {
        image.invert_axis(Axis(1));
        if let Some(result) = check_array_rotations(&mut image) {
            result
        } else {
            image.invert_axis(Axis(0));
            if let Some(result) = check_array_rotations(&mut image) {
                result
            } else {
                image.invert_axis(Axis(1));
                check_array_rotations(&mut image).expect("Could not find monsters")
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let mut tiles: HashMap<_, _> = contents
        .trim()
        .split("\n\n")
        .map(|spec| {
            let tile = Tile::from_spec(spec);
            (tile.id, Rc::new(RefCell::new(tile)))
        })
        .collect();
    let finder = make_border_finder(&tiles);

    println!("Part 1: {}", part1(&tiles, &finder));
    println!("Part 2: {}", part2(&mut tiles, &finder)); // 2632 is too high
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let sample = include_str!("sample");
        let mut tiles: HashMap<_, _> = sample
            .trim()
            .split("\n\n")
            .map(|spec| {
                let tile = Tile::from_spec(spec);
                (tile.id, Rc::new(RefCell::new(tile)))
            })
            .collect();
        let finder = make_border_finder(&tiles);

        assert_eq!(part1(&tiles, &finder), 20899048083289);
        assert_eq!(part2(&mut tiles, &finder), 273);
    }
}
