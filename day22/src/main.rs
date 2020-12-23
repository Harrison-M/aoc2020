mod basic_game;
mod game;
mod parse_deck;
mod recursive_game;

use basic_game::BasicGame;
use game::CombatGame;
use recursive_game::RecursiveGame;
use std::{env, fs};

fn play_and_score<T: CombatGame>(mut game: T) -> usize {
    let winner = game.play_until_win();
    let winning_deck = game.get_deck(winner);
    winning_deck
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &card)| (i + 1) * (card as usize))
        .sum()
}

fn part1(initial_state: &str) -> usize {
    let game: BasicGame = initial_state.into();
    play_and_score(game)
}

fn part2(initial_state: &str) -> usize {
    let game: RecursiveGame = initial_state.into();
    play_and_score(game)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = include_str!("sample");

    #[test]
    fn part1_example() {
        assert_eq!(part1(SAMPLE), 306);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(SAMPLE), 291);
    }
}
