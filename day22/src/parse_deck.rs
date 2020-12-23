use std::collections::VecDeque;

pub fn parse_deck(deck: &str) -> VecDeque<u8> {
    deck
        .lines()
        .skip(1)
        .map(|card| card.parse().unwrap())
        .collect()
}
