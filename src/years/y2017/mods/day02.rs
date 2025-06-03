fn input() -> Vec<Vec<i32>> {
    include_str!("../inputs/day02.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

pub fn part_one() {
    println!(
        "{:?}",
        input().iter().fold(0, |mut total, nums| {
            total += (nums.iter().max().unwrap() - nums.iter().min().unwrap()).abs();
            total
        })
    )
}

pub fn part_two() {
    println!(
        "{:?}",
        input().iter().fold(0, |mut total, nums| {
            nums.iter().enumerate().for_each(|(i, num1)| {
                nums.iter().enumerate().for_each(|(j, num2)| {
                    if i != j && num1 % num2 == 0 {
                        total += num1 / num2;
                    }
                })
            });

            total
        })
    )
}
