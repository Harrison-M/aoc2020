use std::{cell::RefCell, collections::{HashSet, HashMap}, env, mem, rc::Rc, fmt::Display, iter::FromIterator};

/// Represents a single cup, linked to the next cup in the circle
struct Cup {
    label: usize,
    next_cup: Option<Rc<RefCell<Cup>>>,
}

/// Represents a circle of cups
struct CupGame {
    count: usize,
    cups: HashMap<usize, Rc<RefCell<Cup>>>,
    position: usize,
}

impl FromIterator<usize> for CupGame {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let cups: Vec<_> = iter
            .into_iter()
            .map(|label|
                Rc::new(
                    RefCell::new(
                        Cup {
                            label,
                            next_cup: None
                        }
                    )
                )
            )
            .collect();

        let count = cups.len();

        for i in 0..count {
            let cup1 = &cups[i];
            let cup2 = &cups[(i + 1) % count];
            cup1.borrow_mut().next_cup = Some(Rc::clone(cup2));
        }

        let position = cups[0].borrow().label;

        Self {
            count,
            cups: cups.iter()
                .map(|cup| (cup.borrow().label, Rc::clone(cup)))
                .collect(),
            position,
        }
    }
}

impl From<&str> for CupGame {
    fn from(initial_circle: &str) -> Self {
        initial_circle
            .bytes()
            .map(|ascii| (ascii - 48) as usize) // numerals start at ascii 48
            .collect()
    }
}

/// An iterator that continues indefinitely clockwise around a cup cirle
struct CupIterator {
    current: Option<Rc<RefCell<Cup>>>
}

impl Iterator for CupIterator {
    type Item = Rc<RefCell<Cup>>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.current
            .as_ref()
            .and_then(|c|
                c.borrow().next_cup
                .as_ref()
                .map(Rc::clone)
            );
        mem::replace(&mut self.current, next)
    }
}

impl CupGame {
    /// Get an iterator starting from the current position
    fn iter(&self) -> CupIterator {
        CupIterator {
            current: self.cups
                .get(&self.position)
                .map(Rc::clone)
        }
    }

    /// Play one step of the game, in-place
    fn step(&mut self) {
        let mut picked_up: Vec<_> = self.iter()
            .skip(1)
            .take(4)
            .collect();

        let new_next = picked_up.pop();
        self.cups
            .get(&self.position)
            .unwrap()
            .borrow_mut()
            .next_cup = new_next;

        let removed_labels: HashSet<usize> = picked_up
            .iter()
            .map(|c| c.borrow().label)
            .collect();

        let mut destination_label = self.position - 1;

        loop {
            if destination_label == 0 {
                destination_label = self.count;
            }
            if !removed_labels.contains(&destination_label) {
                break;
            }
            destination_label -= 1;
        };

        let destination = self.cups
            .get(&destination_label)
            .unwrap();

        picked_up.last().unwrap().borrow_mut().next_cup = destination
            .borrow()
            .next_cup
            .as_ref()
            .map(Rc::clone);

        destination
            .borrow_mut()
            .next_cup = picked_up.first().map(Rc::clone);

        self.position = self.cups
            .get(&self.position)
            .unwrap()
            .borrow()
            .next_cup
            .as_ref()
            .unwrap()
            .borrow()
            .label;
    }
}

impl Display for CupGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let iter = self.iter()
            .map(|c| c.borrow().label)
            .skip_while(|&l| l != 1)
            .skip(1)
            .take_while(|&l| l != 1);
        for l in iter {
            write!(f, "{}", l)?;
        }
        Ok(())
    }
}

fn part1(input: &str, steps: usize) -> String {
    let mut game: CupGame = input.into();
    for _ in 0..steps {
        game.step();
    }
    game.to_string()
}

fn part2(input: &str) -> usize {
    let initial_cups: Vec<_> = input
        .bytes()
        .map(|ascii| (ascii - 48) as usize) // numerals start at ascii 48
        .collect();

    let next = initial_cups.iter().max().unwrap() + 1;

    let mut game: CupGame = initial_cups
        .into_iter()
        .chain(next..=1000000)
        .collect();

    // Maybe there's a more clever way to do this, but brute-force works
    for i in 0..10000000 {
        if i % 1000000 == 0 {
            println!("Reached step {}", i); }
        game.step();
    }

    let one = game.cups.get(&1).unwrap().borrow();
    let second = one.next_cup.as_ref().unwrap().borrow();
    let third = second.next_cup.as_ref().unwrap().borrow();
    second.label * third.label
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let steps = args[2].parse().unwrap();
    println!("Part 1: {}", part1(input, steps));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("389125467", 100), "67384529");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("389125467"), 149245887792);
    }
}
