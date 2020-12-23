use std::{collections::VecDeque, env, fs};

struct CombatGame {
    player1: VecDeque<u8>,
    player2: VecDeque<u8>,
}

impl From<&str> for CombatGame {
    fn from(initial_state: &str) -> Self {
        let mut player_iter = initial_state.split("\n\n");
        let player1 = player_iter
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .map(|card| card.parse().unwrap())
            .collect();

        let player2 = player_iter
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .map(|card| card.parse().unwrap())
            .collect();

        Self { player1, player2 }
    }
}

impl CombatGame {
    fn turn(&mut self) -> bool {
        if self.player1.is_empty() || self.player2.is_empty() {
            false
        } else {
            let p1_card = self.player1.pop_front().unwrap();
            let p2_card = self.player2.pop_front().unwrap();
            if p1_card > p2_card {
                self.player1.push_back(p1_card);
                self.player1.push_back(p2_card);
            } else {
                self.player2.push_back(p2_card);
                self.player2.push_back(p1_card);
            }
            true
        }
    }
}

fn part1(initial_state: &str) -> usize {
    let mut game: CombatGame = initial_state.into();
    loop {
        if !game.turn() {
            break;
        }
    };
    let winner = if game.player1.is_empty() {
        game.player2
    } else {
        game.player1
    };

    winner
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &card)| (i + 1) * (card as usize))
        .sum()
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

    #[test]
    fn part1_example() {
        let sample = include_str!("sample");
        assert_eq!(part1(sample), 306);
    }
}
