use std::collections::HashSet;

#[allow(dead_code)]
pub fn part_one() {
    println!(
        "{}",
        include_str!("../inputs/day04.txt")
            .lines()
            .filter(|line| {
                let reg: Vec<&str> = line.split_whitespace().collect();
                let adj: HashSet<&str> = HashSet::from_iter(reg.clone().into_iter());

                reg.len() == adj.len()
            })
            .count()
    )
}

#[allow(dead_code)]
pub fn part_two() {
    println!(
        "{}",
        include_str!("../inputs/day04.txt")
            .lines()
            .filter(|line| {
                let reg: Vec<&str> = line.split_whitespace().collect();
                let adj: HashSet<String> =
                    HashSet::from_iter(reg.clone().into_iter().map(|word| {
                        let mut letters: Vec<char> = word
                            .chars()
                            .collect::<HashSet<char>>()
                            .into_iter()
                            .collect::<Vec<char>>();
                        letters.sort();
                        letters.into_iter().collect::<String>()
                    }));

                reg.len() == adj.len()
            })
            .count()
    );
}
