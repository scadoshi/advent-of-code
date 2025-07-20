use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Entry {
    signal: Vec<HashSet<char>>,
    output: Vec<HashSet<char>>,
}

impl From<&str> for Entry {
    fn from(value: &str) -> Self {
        let (signal, output) = value.split_once("|").expect("failed to split once");
        let signal: Vec<HashSet<char>> = signal
            .split_whitespace()
            .map(|x| x.chars().collect())
            .collect();
        let output: Vec<HashSet<char>> = output
            .split_whitespace()
            .map(|x| x.chars().collect())
            .collect();
        Self { signal, output }
    }
}

type Entries = Vec<Entry>;
trait EntriesMethods {
    fn input() -> Self;
}
impl EntriesMethods for Entries {
    fn input() -> Self {
        include_str!("../inputs/day08.txt")
            .lines()
            .map(|x| Entry::from(x))
            .collect()
    }
}

#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();
    let result: usize = Entries::input()
        .iter()
        .map(|x| x
            .output
            .iter()
            .filter(|x| [2, 3, 4, 7].contains(&x.len()))
            .count())
        .sum();

    println!("part_one()={:#?}", result);
    println!("runtime={:#?}", start.elapsed());
}

impl Entry {
    fn deduce_key(&self) -> HashMap<String, usize> {
        // deduction by segment length
        let one: HashSet<char> = self
            .signal
            .iter()
            .find(|x| x.len() == 2)
            .expect("one not found")
            .to_owned();
        let seven: HashSet<char> = self
            .signal
            .iter()
            .find(|x| x.len() == 3)
            .expect("seven not found")
            .to_owned();
        let four: HashSet<char> = self
            .signal
            .iter()
            .find(|x| x.len() == 4)
            .expect("four not found")
            .to_owned();
        let eight: HashSet<char> = self
            .signal
            .iter()
            .find(|x| x.len() == 7)
            .expect("eight not found")
            .to_owned();

        // deduction by length and containment
        let six: HashSet<char> = self
            .signal
            .iter()
            // six is the only length = 6 containing one
            .find(|x| x.len() == 6 && !x.is_superset(&one))
            .expect("six not found")
            .to_owned();
        let nine: HashSet<char> = self
            .signal
            .iter()
            // nine is the only length = 6 containing four
            .find(|x| x.len() == 6 && x.is_superset(&four))
            .expect("nine not found")
            .to_owned();
        let three: HashSet<char> = self
            .signal
            .iter()
            // nine is the only length = 5 containing one
            .find(|x| x.len() == 5 && x.is_superset(&one))
            .expect("three not found")
            .to_owned();
        let five: HashSet<char> = self
            .signal
            .iter()
            // five is the only length = 5 which six contains
            .find(|x| x.len() == 5 && x.is_subset(&six))
            .expect("five not found")
            .to_owned();

        // deduction by elimination
        let zero: HashSet<char> = self
            .signal
            .iter()
            .find(|x| 
                // 0 is the last length = 6
                x.len() == 6 && ![&six, &nine].contains(&&x)
            )
            .expect("zero not found")
            .to_owned();
        let two: HashSet<char> = self
            .signal
            .iter()
            // two is the last length = 5
            .find(|x| x.len() == 5 && ![&three, &five].contains(&&x))
            .expect("zero not found")
            .to_owned();

        [
            (zero, 0),
            (one, 1),
            (two, 2),
            (three, 3),
            (four, 4),
            (five, 5),
            (six, 6),
            (seven, 7),
            (eight, 8),
            (nine, 9),
        ]
        .into_iter()
        .map(|(k, v)| (k.into_iter().sorted().collect(), v))
        .collect()
    }
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();
    let result = Entries::input().into_iter().fold(0, |total, entry| {
        let key = entry.deduce_key();
        let increment: usize = entry
            .output
            .iter()
            .map(|x| {
                let sorted: String = x.iter().sorted().collect();
                key.get(&sorted)
                    .expect("output digit not found in key")
                    .to_string()
            })
            .collect::<String>()
            .parse()
            .expect("failed to parse usize");

        total + increment
    });

    println!("part_two()={:#?}", result);
    println!("runtime={:#?}", start.elapsed());
}
