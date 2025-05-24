#[allow(dead_code)]
pub fn part_one() {
    println!(
        "{:#?}",
        include_str!("../inputs/day01.txt")
            .lines()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
            .windows(2)
            .filter(|x| x[0] < x[1])
            .count()
    )
}

#[allow(dead_code)]
pub fn part_two() {
    println!(
        "{:#?}",
        include_str!("../inputs/day01.txt")
            .lines()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
            .windows(3)
            .map(|x| x.into_iter().sum::<i32>())
            .collect::<Vec<i32>>()
            .windows(2)
            .filter(|x| x[0] < x[1])
            .count()
    )
}
