#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Scratcher {
    id: u32,
    win_nums: Vec<u32>,
    play_nums: Vec<u32>,
}

impl Scratcher {
    #[allow(dead_code)]
    fn new(id: u32, win_nums: Vec<u32>, play_nums: Vec<u32>) -> Self {
        Self {
            id,
            win_nums,
            play_nums,
        }
    }

    #[allow(dead_code)]
    fn input() -> Vec<Self> {
        include_str!("../inputs/day04.txt").lines().fold(
            Vec::new(),
            |mut scratchers: Vec<Scratcher>, line| {
                let id: u32 = line
                    .split(':')
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse()
                    .unwrap();

                let win_nums: Vec<u32> = line
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .split('|')
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();

                let play_nums: Vec<u32> = line
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .split('|')
                    .nth(1)
                    .unwrap()
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();

                scratchers.push(Self::new(id, win_nums, play_nums));
                scratchers
            },
        )
    }

    #[allow(dead_code)]
    fn winners(&self) -> u32 {
        self.play_nums
            .iter()
            .filter(|x| self.win_nums.contains(x))
            .count() as u32
    }

    #[allow(dead_code)]
    fn points(&self) -> u32 {
        let winners: u32 = self.winners();
        if winners == 0 {
            0
        } else {
            2_u32.pow(winners - 1)
        }
    }
}

#[allow(dead_code)]
pub fn part_one() {
    println!(
        "{:?}",
        Scratcher::input().iter().map(|x| x.points()).sum::<u32>()
    );
}

// use std::collections::HashMap;

#[allow(dead_code)]
pub fn part_two() {
    let mut scratchers = Scratcher::input();

    let mut i = 0;

    while i < scratchers.len() {
        let winners = scratchers[i].winners();
        if winners > 0 {
            for other_id in scratchers[i].id + 1..=scratchers[i].id + winners {
                scratchers.push(
                    scratchers
                        .iter()
                        .find(|x| x.id == other_id)
                        .unwrap()
                        .clone(),
                )
            }
        }
        i += 1;
    }
    println!("{}", scratchers.len());
}
