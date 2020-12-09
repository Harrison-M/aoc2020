use std::env;
use std::fs;

enum SpaceHalf {
    High,
    Low,
}

/// From a series of partitions, get a space coordinate
fn position_from_partitions(partitions: Vec<SpaceHalf>) -> usize {
    partitions.iter().rev()
        .scan(1, |state, half| {
            let next = Some(match half {
                SpaceHalf::High => *state,
                SpaceHalf::Low => 0
            });

            *state = *state * 2;
            next
        }).fold(0, |acc, n| acc + n)
}

/// Compute the seat id from a row and column
fn seat_id_from_position((row, col): (usize, usize)) -> usize {
    row * 8 + col
}

/// From a seat string, get the seat's row and column
fn seat_position_from_string(seat: &str) -> (usize, usize) {
    let (row_str, col_str) = seat.split_at(7);
    let row_partitions: Vec<SpaceHalf> = row_str.chars()
        .map(|c| match c {
            'F' => SpaceHalf::Low,
            'B' => SpaceHalf::High,
            _ => panic!("Unexpected row char {}", c)
        }).collect();
    let col_partitions: Vec<SpaceHalf> = col_str.chars()
        .map(|c| match c {
            'L' => SpaceHalf::Low,
            'R' => SpaceHalf::High,
            _ => panic!("Unexpected col char {}", c)
        }).collect();
    (
        position_from_partitions(row_partitions),
        position_from_partitions(col_partitions)
    )
}

fn part1<'a>(seats: impl Iterator<Item=&'a str>) -> usize {
    seats
        .map(seat_position_from_string)
        .map(seat_id_from_position)
        .max()
        .unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    println!("Part 1: {}", part1(contents.lines()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_position_from_string() {
        assert_eq!(seat_position_from_string("BFFFBBFRRR"), (70, 7));
        assert_eq!(seat_position_from_string("FFFBBBFRRR"), (14, 7));
        assert_eq!(seat_position_from_string("BBFFBBFRLL"), (102, 4));
    }

    #[test]
    fn seat_ids_convert_correctly() {
        assert_eq!(seat_id_from_position((70, 7)), 567);
        assert_eq!(seat_id_from_position((14, 7)), 119);
        assert_eq!(seat_id_from_position((102, 4)), 820);
    }
}
