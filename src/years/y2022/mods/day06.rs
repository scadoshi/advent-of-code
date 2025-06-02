use std::collections::HashSet;

fn unique_length_after(length: usize) -> usize {
    include_str!("../inputs/day06.txt")
        .chars()
        .enumerate()
        .collect::<Box<[(usize, char)]>>()
        .windows(length)
        .find(|window| window.len() == window.iter().map(|x| x.1).collect::<HashSet<char>>().len())
        .expect("signal not found")[length - 1]
        .0
        + 1
}

#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();
    println!(
        "part_one={:?} ... runtime={:?}",
        unique_length_after(4),
        start.elapsed()
    );
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();
    println!(
        "part_two={:?} ... runtime={:?}",
        unique_length_after(14),
        start.elapsed()
    );
}
