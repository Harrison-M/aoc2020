use std::{collections::HashMap, env};

struct Game<'a> {
    init: &'a [usize],
    last_num: usize,
    last_seen_map: HashMap<usize, usize>,
    turn: usize,
}

impl<'a> Game<'a> {
    fn new(init: &'a [usize]) -> Self {
        Self {
            init,
            last_num: 0,
            last_seen_map: HashMap::new(),
            turn: 0,
        }
    }
}

impl<'a> Iterator for Game<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next_num = if self.turn < self.init.len() {
            self.init[self.turn]
        } else if let Some(last_seen) = self.last_seen_map.get(&self.last_num) {
            self.turn - last_seen - 1
        } else {
            0
        };

        if self.turn > 0 {
            self.last_seen_map.insert(self.last_num, self.turn - 1);
        }
        self.turn += 1;
        self.last_num = next_num;
        Some(next_num)
    }
}

fn part1(game: &mut Game) -> usize {
    game.nth(2019).unwrap()
}

fn part2(game: &mut Game) -> usize {
    game.nth(29999999).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let init_str = &args[1];
    let init: Vec<usize> = init_str.split(',').map(|s| s.parse().unwrap()).collect();
    println!("Part 1: {}", part1(&mut Game::new(init.as_slice())));
    println!("Part 2: {}", part2(&mut Game::new(init.as_slice())));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_examples() {
        let samples = vec![
            (&[0,3,6], 436),
            (&[1,3,2], 1),
            (&[2,1,3], 10),
            (&[1,2,3], 27),
            (&[2,3,1], 78),
            (&[3,2,1], 438),
            (&[3,1,2], 1836),
        ];

        for (init, expect) in samples {
            assert_eq!(part1(&mut Game::new(init)), expect);
        }
    }

    #[test]
    fn part2_examples() {
        // Painfully slow
        let samples = vec![
            (&[0,3,6], 175594),
            (&[1,3,2], 2578),
            (&[2,1,3], 3544142),
            (&[1,2,3], 261214),
            (&[2,3,1], 6895259),
            (&[3,2,1], 18),
            (&[3,1,2], 362),
        ];

        for (init, expect) in samples {
            assert_eq!(part2(&mut Game::new(init)), expect);
        }
    }
}
