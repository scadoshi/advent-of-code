use std::collections::HashMap;

fn input() -> Vec<String> {
    include_str!("../inputs/day06.txt")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

#[allow(dead_code)]
// does both parts
pub fn part_one() {
    let input = input();
    let word_length = input[0].len();
    let map: HashMap<usize, HashMap<char, i32>> = input.into_iter().fold(
        HashMap::new(),
        |mut map: HashMap<usize, HashMap<char, i32>>, line| {
            for (i, char) in line.chars().enumerate() {
                *map.entry(i).or_default().entry(char).or_default() += 1;
            }
            map
        },
    );
    let mut word = String::new();
    for i in 0..word_length {
        if let Some(x) = map.get(&i) {
            let most_common = x
                .iter()
                .max_by(|a, b| a.1.cmp(&b.1))
                .expect("Max not found")
                .0;
            word += most_common.to_string().as_str();
        }
    }
    println!("{}", word);
    let mut word = String::new();
    for i in 0..word_length {
        if let Some(x) = map.get(&i) {
            let most_common = x
                .iter()
                .min_by(|a, b| a.1.cmp(&b.1))
                .expect("Max not found")
                .0;
            word += most_common.to_string().as_str();
        }
    }
    println!("{}", word);
}

#[allow(dead_code)]
pub fn part_two() {}
