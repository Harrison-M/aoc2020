use std::collections::HashSet;
use std::env;
use std::fs;

const TREE: char = '#';

struct TreePattern {
    tree_index_rows: Vec<HashSet<usize>>,
    row_width: usize
}

fn process_pattern<'a>(lines: impl Iterator<Item=&'a str>) -> TreePattern {
    let mut peekable_lines = lines.peekable();
    let row_width = peekable_lines.peek().unwrap().len();
    let tree_index_rows: Vec<HashSet<usize>> = peekable_lines.map(|line| 
        line.char_indices().filter_map(|(idx, space)| {
            if space == TREE { Some(idx) }
            else { None }
        }).collect()
    ).collect();

    TreePattern { tree_index_rows, row_width }
}

/// Going right three and down 1, find the number of trees on the way to the bottom
fn part1(pattern: &TreePattern) -> usize {
    let mut column: usize = 0;
    let mut trees: usize = 0;

    for row in pattern.tree_index_rows.iter() {
        if row.contains(&(column % pattern.row_width)) {
            trees += 1;
        }
        column += 3;
    }

    trees
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let pattern = process_pattern(contents.lines());

    println!("Part 1: {}", part1(&pattern));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let sample = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ];

        let pattern = process_pattern(sample.into_iter());
        assert_eq!(part1(&pattern), 7);
    }
}
