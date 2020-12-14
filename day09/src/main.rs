// This one's a bit too clever for its own good, but it was a fun exercise
// in trying out and practicing smart pointers.
use std::{
    collections::{HashSet, HashMap},
    env,
    fs,
    ops::Deref,
    rc::{Weak, Rc},
};
use weak_table::WeakHashSet;

// Adapted from the Symbol example for WeakHashSet
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct ValidSum(Rc<usize>);

impl Deref for ValidSum {
    type Target = usize;
    fn deref(&self) -> &usize {
        self.0.deref()
    }
}

/// A set of in-use interned integers, to track pair sums in the current window.
#[derive(Debug, Default)]
struct ValidSumTable(WeakHashSet<Weak<usize>>);

/// A map of numbers to sets of sums for which the number is the first number
/// contributing to the sum
type SumOriginMap = HashMap<usize, HashSet<ValidSum>>;

impl ValidSumTable {
    fn new() -> Self {
        Self::default()
    }

    /// Ensure we're counting references correctly by confining each number to a single reference
    fn intern(&mut self, sum: usize) -> ValidSum {
        if let Some(rc) = self.0.get(&sum) {
            ValidSum(rc)
        } else {
            let rc = Rc::from(sum);
            self.0.insert(Rc::clone(&rc));
            ValidSum(rc)
        }
    }
}

/// Interns a number and adds it to the map and interning table, updating the sum sets
/// of previous numbers.
fn add_to_window(sum_map: &mut SumOriginMap, sum_table: &mut ValidSumTable, num: usize) {
    for (add, set) in sum_map.iter_mut() {
        set.insert(sum_table.intern(add + num));
    }

    sum_map.insert(num, HashSet::new());
}

fn part1(numbers: &Vec<usize>, window: usize) -> usize {
    let mut sum_map: SumOriginMap = HashMap::new();
    let mut sum_table = ValidSumTable::new();

    // Get initial sums
    let preamble = &numbers[0..window];
    for num in preamble.iter() {
        add_to_window(&mut sum_map, &mut sum_table, *num);
    }

    numbers
        .clone()
        .iter()
        .zip(numbers.iter().skip(window)) // Pair numbers leaving and entering window as it slides
        .find_map(|(exiting, entering)| {
            if !sum_table.0.contains(entering) {
                Some(entering)
            } else {
                // Time for side effects!
                sum_map.remove(exiting);
                add_to_window(&mut sum_map, &mut sum_table, *entering);
                None
            }
        })
        .copied()
        .unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");
    let numbers: Vec<usize> = contents.lines().map(|l| l.parse().unwrap()).collect();
    println!("Part 1: {}", part1(&numbers, 25));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let sample = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102,
            117, 150, 182, 127, 219, 299, 277, 309, 576,
        ];

        assert_eq!(part1(&sample, 5), 127);
    }
}
