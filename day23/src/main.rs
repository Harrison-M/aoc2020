use std::{cell::RefCell, collections::{HashSet, BTreeMap}, env, mem, rc::Rc, fmt::Display};

struct Cup {
    label: u8,
    next_cup: Option<Rc<RefCell<Cup>>>,
}

struct CupGame {
    cups: BTreeMap<u8, Rc<RefCell<Cup>>>,
    position: u8,
}

impl From<&str> for CupGame {
    fn from(initial_circle: &str) -> Self {
        let cups: Vec<_> = initial_circle
            .bytes()
            .map(|ascii|
                Rc::new(
                    RefCell::new(
                        Cup {
                            label: ascii - 48, // numerals start at ascii 48
                            next_cup: None,
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
            cups: cups.iter()
                .map(|cup| (cup.borrow().label, Rc::clone(cup)))
                .collect(),
            position,
        }
    }
}

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
    fn iter(&self) -> CupIterator {
        CupIterator {
            current: self.cups
                .get(&self.position)
                .map(Rc::clone)
        }
    }

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

        let removed_labels: HashSet<u8> = picked_up
            .iter()
            .map(|c| c.borrow().label)
            .collect();

        let destination_label = self.cups
            .keys()
            .rev()
            .cycle()
            .skip_while(|&&i| i != self.position)
            .skip(1)
            .find(|i| !removed_labels.contains(i))
            .unwrap();

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let steps = args[2].parse().unwrap();
    println!("Part 1: {}", part1(input, steps));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("389125467", 100), "67384529");
    }
}
