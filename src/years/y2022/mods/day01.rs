pub fn part_one() {
    println!(
        "{:#?}",
        include_str!("../inputs/day01.txt")
            .lines()
            .map(|x| x.parse().unwrap_or(0))
            .collect::<Vec<i32>>()
            .split(|x| *x == 0)
            .map(|slice| slice.to_vec())
            .collect::<Vec<Vec<i32>>>()
            .iter()
            .map(|group| group.iter().sum::<i32>())
            .max()
            .unwrap()
    )
}
pub fn part_two() {
    let mut elves = include_str!("../inputs/day01.txt")
        .lines()
        .map(|x| x.parse().unwrap_or(0))
        .collect::<Vec<i32>>()
        .split(|x| *x == 0)
        .map(|slice| slice.to_vec())
        .collect::<Vec<Vec<i32>>>()
        .iter()
        .map(|group| group.iter().sum::<i32>())
        .collect::<Vec<i32>>();

    elves.sort_by(|a, b| b.cmp(&a));

    println!("{}", elves.iter().take(3).sum::<i32>())
}
