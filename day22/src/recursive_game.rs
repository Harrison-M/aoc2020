use crate::game::{CombatGame, Player, Player::*};
use crate::parse_deck::parse_deck;
use std::collections::{HashSet, VecDeque};

pub struct RecursiveGame {
    previous_turns: HashSet<(VecDeque<u8>, VecDeque<u8>)>,
    player1: VecDeque<u8>,
    player2: VecDeque<u8>,
}

impl From<&str> for RecursiveGame {
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

        Self { player1, player2, previous_turns: HashSet::new() }
    }
}

impl CombatGame for RecursiveGame {
    fn get_deck(&self, player: Player) -> &VecDeque<u8> {
        match player {
            Player1 => &self.player1,
            Player2 => &self.player2,
        }
    }

    fn turn(&mut self) -> Option<Player> {
        let state = (self.player1.clone(), self.player2.clone());
        if !self.previous_turns.insert(state) {
            Some(Player1)
        } else if self.player1.is_empty() {
            Some(Player2)
        } else if self.player2.is_empty() {
            Some(Player1)
        } else {
            let p1_card = self.player1.pop_front().unwrap();
            let p2_card = self.player2.pop_front().unwrap();
            let p1_card_us = p1_card as usize;
            let p2_card_us = p2_card as usize;
            let round_winner =
                if p1_card_us <= self.player1.len() && p2_card_us <= self.player2.len() {
                    let mut sub_game = Self {
                        previous_turns: HashSet::new(),
                        player1: self.player1
                            .iter()
                            .take(p1_card_us)
                            .copied()
                            .collect(),
                        player2: self.player2
                            .iter()
                            .take(p2_card_us)
                            .copied()
                            .collect(),
                    };
                    sub_game.play_until_win()
                } else if p1_card > p2_card {
                    Player1
                } else {
                    Player2
                };

            match round_winner {
                Player1 => {
                    self.player1.push_back(p1_card);
                    self.player1.push_back(p2_card);
                }
                Player2 =>{
                    self.player2.push_back(p2_card);
                    self.player2.push_back(p1_card);
                }
            }

            None
        }
    }
}
