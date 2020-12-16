use std::{env, fs, ops::{Add, AddAssign, Mul}};

enum RotationDir { LEFT, RIGHT }

/// 2D coordinates
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Position(isize, isize);

const CW_FACINGS: &[Position] = &[
    Position(1, 0),
    Position(0, -1),
    Position(-1, 0),
    Position(0, 1)
];

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<isize> for Position {
    type Output = Position;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

/// A representation of a moving ferry
struct MovingFerry {
    facing: Position,
    position: Position,
}

impl MovingFerry {
    fn new() -> Self {
        Self {
            facing: Position(1, 0),
            position: Position(0, 0),
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
            "N" => self.position += Position(0, 1) * value,
            "S" => self.position += Position(0, -1) * value,
            "E" => self.position += Position(1, 0) * value,
            "W" => self.position += Position(-1, 0) * value,
            "L" => self.rotate(RotationDir::LEFT, value),
            "R" => self.rotate(RotationDir::RIGHT, value),
            "F" => self.position += self.facing * value,
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    println!("Part 1: {}", part1(contents.lines()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        let instructions = vec!["F10", "N3", "F7", "R90", "F11"];
        assert_eq!(part1(instructions.into_iter()), 25);
    }
}
