use itertools::Itertools;
use std::{env, fs, collections::{HashMap, HashSet}};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Seat(isize, isize);

impl Seat {
    /// List all possible adjacent seat locations. May return out of bounds seats.
    fn adjacent_seats(&self) -> Vec<Seat> {
        let &Seat(x, y) = self;
        vec![
            Seat(x - 1, y - 1),
            Seat(x - 1, y),
            Seat(x - 1, y + 1),
            Seat(x, y - 1),
            Seat(x, y + 1),
            Seat(x + 1, y - 1),
            Seat(x + 1, y),
            Seat(x + 1, y + 1),
        ]
    }
}

/// A set of seats, with a map to easily find seats adjacent to another
struct Ferry {
    seats: HashSet<Seat>,
    adjacents_for_seats: HashMap<Seat, Vec<Seat>>,
    visibles_for_seats: HashMap<Seat, Vec<Seat>>,
}

impl Ferry {
    /// Create a Ferry from a puzzle input
    fn from_string(input: String) -> Self {
        let width = input.lines().next().unwrap().len() as isize;
        let height = input.lines().count() as isize;

        let seats: HashSet<Seat> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)|
                line
                    .char_indices()
                    .filter_map(move |(x, c)|
                        if c == 'L' {
                            Some(Seat(x as isize, y as isize))
                        } else {
                            None
                        })
            ).collect();

        let adjacents_for_seats = seats
            .iter()
            .map(|&seat|
                (seat,
                 seat
                     .adjacent_seats()
                     .into_iter()
                     .filter(|adj_seat| seats.contains(adj_seat))
                     .collect()
                )
            ).collect();

        let visibles_for_seats =
            seats
                .iter()
                .map(|&seat| {
                    let Seat(x, y) = seat;
                    let dir_range = -1isize..=1isize;
                    (seat,
                    dir_range
                        .clone()
                        .cartesian_product(dir_range)
                        .filter_map(|(x_dir, y_dir)| {
                            if x_dir == 0 && y_dir == 0 {
                                return None;
                            }

                            let mut check_x = x + x_dir;
                            let mut check_y = y + y_dir;
                            loop {
                                let viewed_seat = Seat(check_x, check_y);
                                if check_x < 0 ||
                                   check_x >= width ||
                                   check_y < 0 ||
                                   check_y >= height {
                                       break None;
                                } else if seats.contains(&viewed_seat){
                                    break Some(viewed_seat);
                                } else {
                                    check_x += x_dir;
                                    check_y += y_dir;
                                }
                            }
                        })
                        .collect())
                }).collect();

        Self { seats, adjacents_for_seats, visibles_for_seats }
    }

    /// Given a set of occupied seats, find out what seats will be occupied in the next step
    /// based on the first model (adjacent seats)
    fn step1(&self, occupied_seats: &HashSet<Seat>) -> HashSet<Seat> {
        // If no seats are occupied, all seats are filled
        if occupied_seats.is_empty() {
            return self.seats.clone();
        }

        self.adjacents_for_seats
            .iter()
            .filter_map(|(seat, adjacents)| {
                if occupied_seats.contains(seat) { // Occupied
                    let stays_occupied = adjacents.len() < 4 || // Will always be occupied
                        adjacents
                            .iter()
                            .filter(|adj| occupied_seats.contains(adj))
                            .count() < 4;

                    if stays_occupied {
                        Some(seat)
                    } else {
                        None
                    }
                } else if adjacents.iter().all(|adj| !occupied_seats.contains(adj)) {
                    // All surrounding seats are open
                    Some(seat)
                } else {
                    None
                }
            })
            .copied()
            .collect()
    }

    /// Given a set of occupied seats, find out what seats will be occupied in the next step
    /// based on the second model (visible seats)
    fn step2(&self, occupied_seats: &HashSet<Seat>) -> HashSet<Seat> {
        // If no seats are occupied, all seats are filled
        if occupied_seats.is_empty() {
            return self.seats.clone();
        }

        self.visibles_for_seats
            .iter()
            .filter_map(|(seat, visible)| {
                if occupied_seats.contains(seat) { // Occupied
                    let stays_occupied = visible.len() < 5 || // Will always be occupied
                        visible
                            .iter()
                            .filter(|adj| occupied_seats.contains(adj))
                            .count() < 5;

                    if stays_occupied {
                        Some(seat)
                    } else {
                        None
                    }
                } else if visible.iter().all(|adj| !occupied_seats.contains(adj)) {
                    // All surrounding seats are open
                    Some(seat)
                } else {
                    None
                }
            })
            .copied()
            .collect()
    }
}

/// Find the number of occupied seats when the state stabilizes
fn part1(ferry: &Ferry) -> usize {
    let mut state: HashSet<Seat> = HashSet::new();
    loop {
        let next_state = ferry.step1(&state);
        if state == next_state {
            break;
        }
        state = next_state;
    }
    state.len()
}

/// Find the number of occupied seats when the state stabilizes (model 2)
fn part2(ferry: &Ferry) -> usize {
    let mut state: HashSet<Seat> = HashSet::new();
    loop {
        let next_state = ferry.step2(&state);
        if state == next_state {
            break;
        }
        state = next_state;
    }
    state.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let ferry = Ferry::from_string(contents);
    println!("Part 1: {}", part1(&ferry));
    println!("Part 2: {}", part2(&ferry));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("sample");

    #[test]
    fn part1_examples() {
        let ferry = Ferry::from_string(SAMPLE.to_string());
        assert_eq!(part1(&ferry), 37);
    }

    #[test]
    fn part2_examples() {
        let ferry = Ferry::from_string(SAMPLE.to_string());
        assert_eq!(part2(&ferry), 26);
    }
}
