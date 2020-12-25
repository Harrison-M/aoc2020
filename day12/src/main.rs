use point_2d::Point2D;
use std::{env, fs};

enum RotationDir { LEFT, RIGHT }

/// 2D coordinates
type Position = Point2D<isize>;

const CW_FACINGS: &[Position] = &[
    Point2D(1, 0),
    Point2D(0, -1),
    Point2D(-1, 0),
    Point2D(0, 1)
];

/// A representation of a moving ferry
struct MovingFerry {
    facing: Position,
    position: Position,
}

impl MovingFerry {
    fn new() -> Self {
        Self {
            facing: Point2D(1, 0),
            position: Point2D(0, 0),
        }
    }

    /// Rotate the ship, assuming the facing is locked to a cardinal direction
    fn rotate(&mut self, direction: RotationDir, degrees: isize) {
        let dir_iter: Box<dyn Iterator<Item = &Position>> = match direction {
            RotationDir::LEFT => Box::new(CW_FACINGS.iter().rev().cycle()),
            RotationDir::RIGHT => Box::new(CW_FACINGS.iter().cycle()),
        };

        let facing_skip = degrees / 90;
        self.facing = *dir_iter
            .skip_while(|&&facing| facing != self.facing)
            .skip(facing_skip as usize)
            .next()
            .unwrap();

    }

    /// Update the ferry according to the given instruction
    fn step(&mut self, instruction: &str) {
        let (action, value_str) = instruction.split_at(1);
        let value: isize = value_str.parse().unwrap();
        match action {
            "N" => self.position += Point2D(0, 1) * value,
            "S" => self.position += Point2D(0, -1) * value,
            "E" => self.position += Point2D(1, 0) * value,
            "W" => self.position += Point2D(-1, 0) * value,
            "L" => self.rotate(RotationDir::LEFT, value),
            "R" => self.rotate(RotationDir::RIGHT, value),
            "F" => self.position += self.facing * value,
            a => panic!("Unrecognized action {}", a),
        }
    }
}

struct WaypointFerry {
    position: Position,
    waypoint: Position,
}

impl WaypointFerry {
    fn new() -> Self {
        Self {
            position: Point2D(0, 0),
            waypoint: Point2D(10, 1),
        }
    }

    /// Rotate the ship, assuming the facing is locked to a cardinal direction
    fn rotate(&mut self, direction: RotationDir, degrees: isize) {
        let facing_skip = degrees / 90;
        for _ in 0..facing_skip {
            let Point2D(x, y) = self.waypoint;
            match direction {
                RotationDir::LEFT => self.waypoint = Point2D(-1 * y, x),
                RotationDir::RIGHT => self.waypoint = Point2D(y, -1 * x),
            }
        }
    }

    /// Update the ferry according to the given instruction
    fn step(&mut self, instruction: &str) {
        let (action, value_str) = instruction.split_at(1);
        let value: isize = value_str.parse().unwrap();
        match action {
            "N" => self.waypoint += Point2D(0, 1) * value,
            "S" => self.waypoint += Point2D(0, -1) * value,
            "E" => self.waypoint += Point2D(1, 0) * value,
            "W" => self.waypoint += Point2D(-1, 0) * value,
            "L" => self.rotate(RotationDir::LEFT, value),
            "R" => self.rotate(RotationDir::RIGHT, value),
            "F" => self.position += self.waypoint * value,
            a => panic!("Unrecognized action {}", a),
        }
    }
}

/// Find the manhattan distance traveled after a ferry follows the instructions
fn part1<'a>(instructions: impl Iterator<Item = &'a str>) -> isize {
    let mut ferry = MovingFerry::new();
    for instruction in instructions {
        ferry.step(instruction);
    }
    ferry.position.0.abs() + ferry.position.1.abs()
}

/// Find the manhattan distance traveled after a ferry follows the *real* instructions
fn part2<'a>(instructions: impl Iterator<Item = &'a str>) -> isize {
    let mut ferry = WaypointFerry::new();
    for instruction in instructions {
        ferry.step(instruction);
    }
    ferry.position.0.abs() + ferry.position.1.abs()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    println!("Part 1: {}", part1(contents.lines()));
    println!("Part 2: {}", part2(contents.lines()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        let instructions = vec!["F10", "N3", "F7", "R90", "F11"];
        assert_eq!(part1(instructions.into_iter()), 25);
    }

    #[test]
    fn part2_example() {
        let instructions = vec!["F10", "N3", "F7", "R90", "F11"];
        assert_eq!(part2(instructions.into_iter()), 286);
    }
}
