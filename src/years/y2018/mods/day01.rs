use std::collections::HashSet;

#[allow(dead_code)]
fn input() -> Vec<i32> {
    include_str!("../inputs/day01.txt")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn part_one() {
    println!("{:?}", input().into_iter().sum::<i32>());
}

#[allow(dead_code)]
pub fn part_two() {
    let nums = input();
    let mut frequency = 0;
    let mut seen: HashSet<i32> = HashSet::new();
    let mut p = 0;

    while !seen.contains(&frequency) {
        seen.insert(frequency);
        frequency += nums[p];
        p = (p + 1) % nums.len();
    }

    println!("{}", frequency);
}
