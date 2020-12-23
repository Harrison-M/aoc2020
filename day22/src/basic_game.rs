use crate::game::{CombatGame, Player, Player::*};
use crate::parse_deck::parse_deck;
use std::collections::VecDeque;

pub struct BasicGame {
    player1: VecDeque<u8>,
    player2: VecDeque<u8>,
}

impl From<&str> for BasicGame {
    fn from(initial_state: &str) -> Self {
        let mut player_iter = initial_state.split("\n\n");
        let player1 = parse_deck(
            player_iter
                .next()
                .unwrap()
        );

        let player2 = parse_deck(
            player_iter
                .next()
                .unwrap()
        );

        Self { player1, player2 }
    }
}

impl CombatGame for BasicGame {
    fn get_deck(&self, player: Player) -> &VecDeque<u8> {
        match player {
            Player1 => &self.player1,
            Player2 => &self.player2,
        }
    }

    fn turn(&mut self) -> Option<Player> {
        if self.player1.is_empty() {
            Some(Player2)
        } else if self.player2.is_empty() {
            Some(Player1)
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
            None
        }
    }
}
