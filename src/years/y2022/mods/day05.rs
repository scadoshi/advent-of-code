#[allow(unused_imports)]
use std::collections::{HashMap, VecDeque};

#[allow(dead_code)]
fn krates() -> HashMap<usize, VecDeque<String>> {
    include_str!("../inputs/day05.txt")
        .lines()
        .into_iter()
        .take(9)
        .map(|x| {
            x.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|x| x.iter().collect::<String>().trim().to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
        .into_iter()
        .rev()
        .collect::<Vec<Vec<String>>>()
        .iter()
        .fold(
            HashMap::new(),
            |mut krates: HashMap<usize, VecDeque<String>>, line| {
                // handle the first row (labels for crate columns)
                if krates.is_empty() {
                    for x in line {
                        krates.insert(x.parse().unwrap(), VecDeque::new());
                    }
                } else {
                    for (i, x) in line.iter().enumerate() {
                        if !x.is_empty() {
                            let col: usize = i as usize + 1;
                            krates.entry(col).or_default().push_front(x.to_string());
                        }
                    }
                }
                krates
            },
        )
}

#[allow(dead_code)]
fn top_krates(krates: &HashMap<usize, VecDeque<String>>) -> String {
    let mut top_krates: Vec<(usize, String)> = krates
        .into_iter()
        .filter_map(|(i, stack)| {
            if let Some(krate) = stack.iter().next() {
                Some((
                    *i,
                    krate
                        .chars()
                        .filter(|char| !"[]".contains(*char))
                        .collect::<String>(),
                ))
            } else {
                None
            }
        })
        .collect();
    top_krates.sort_by(|a, b| a.0.cmp(&b.0));
    top_krates.into_iter().map(|x| x.1).collect::<String>()
}

#[allow(dead_code)]
fn instructions() -> Vec<Instr> {
    include_str!("../inputs/day05.txt")
        .lines()
        .skip(10)
        .map(|x| {
            let instr_components = x
                .split_whitespace()
                .filter_map(|y| y.parse::<usize>().ok())
                .collect::<Vec<usize>>();
            let (quantity, from, to) = (
                instr_components[0],
                instr_components[1],
                instr_components[2],
            );
            Instr::new(quantity, from, to)
        })
        .collect()
}

#[allow(dead_code)]
#[derive(Debug)]
struct Instr {
    quantity: usize,
    from: usize,
    to: usize,
}

impl Instr {
    #[allow(dead_code)]
    pub fn new(quantity: usize, from: usize, to: usize) -> Self {
        Self { quantity, from, to }
    }
}

#[allow(dead_code)]
pub fn part_one() {
    let mut krates = krates();
    for instr in instructions() {
        for _ in 0..instr.quantity {
            if let Some(crate_to_move) = krates.entry(instr.from).or_default().pop_front() {
                krates
                    .entry(instr.to)
                    .or_default()
                    .push_front(crate_to_move);
            }
        }
    }
    // println!("{:?}", &krates);
    println!("{}", top_krates(&krates));
}

#[allow(dead_code)]
pub fn part_two() {
    let mut krates = krates();
    for instr in instructions() {
        let krates_to_move: Vec<String> = krates
            .entry(instr.from)
            .or_default()
            .drain(0..instr.quantity)
            .collect();
        for krate in krates_to_move.into_iter().rev() {
            krates.entry(instr.to).or_default().push_front(krate);
        }
    }
    // println!("{:?}", &krates);
    println!("{}", top_krates(&krates));
}
