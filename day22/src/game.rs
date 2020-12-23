use std::collections::VecDeque;

pub enum Player {
    Player1,
    Player2,
}

pub trait CombatGame {
    /// Get the deck for a given player
    fn get_deck(&self, player: Player) -> &VecDeque<u8>;

    /// Take a turn, returning a winner if the game is over
    fn turn(&mut self) -> Option<Player>;

    /// Play the game until a player wins, and return the winner
    fn play_until_win(&mut self) -> Player {
        loop {
            if let Some(player) = self.turn() {
                break player;
            }
        }
    }
}
