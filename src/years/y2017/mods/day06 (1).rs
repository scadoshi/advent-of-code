use std::collections::HashSet;

fn memory_bank() -> Vec<i32> {
    include_str!("../inputs/day06.txt")
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse to u8"))
        .collect()
}

#[allow(dead_code)]
// does both parts
pub fn part_one() {
    let mut memory_bank = memory_bank();
    let mut seen: HashSet<Vec<i32>> = HashSet::new();
    let mut count = 0;

    while !seen.contains(&memory_bank) {
        count += 1;
        seen.insert(memory_bank.clone());
        let (mut i, mut distr) = memory_bank
            .clone()
            .into_iter()
            .enumerate()
            .rev()
            .max_by_key(|(_, x)| *x)
            .expect("Max not found");
        memory_bank[i] = 0;
        while distr > 0 {
            i = (i + 1) % memory_bank.len();
            memory_bank[i] += 1;
            distr -= 1;
        }
    }
    println!("{}", count);

    let repeat = memory_bank.clone();
    let mut started = false;
    let mut count = 0;

    while !started || memory_bank != repeat {
        started = true;
        count += 1;
        let (mut i, mut distr) = memory_bank
            .clone()
            .into_iter()
            .enumerate()
            .rev()
            .max_by_key(|(_, x)| *x)
            .expect("Max not found");
        memory_bank[i] = 0;
        while distr > 0 {
            i = (i + 1) % memory_bank.len();
            memory_bank[i] += 1;
            distr -= 1;
        }
    }
    println!("{}", count);
}

#[allow(dead_code)]
pub fn part_two() {}
