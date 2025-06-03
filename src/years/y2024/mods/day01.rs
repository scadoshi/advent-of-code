use std::collections::HashMap;

#[derive(Debug)]
struct Input {
    left_list: Vec<i32>,
    right_list: Vec<i32>,
}

impl Input {
    fn new() -> Self {
        let input = include_str!("../inputs/day01.txt");

        let (mut left_list, mut right_list): (Vec<i32>, Vec<i32>) = input
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                (
                    parts.next().unwrap().parse::<i32>().unwrap(),
                    parts.next().unwrap().parse::<i32>().unwrap(),
                )
            })
            .unzip();

        left_list.sort();
        right_list.sort();

        Input {
            left_list,
            right_list,
        }
    }
}

#[allow(dead_code)]
pub fn part_one() {
    let input = Input::new();
    let total: i32 = input
        .left_list
        .iter()
        .zip(input.right_list.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();
    println!("{}", total);
}

#[allow(dead_code)]
pub fn part_two() {
    let input = Input::new();

    let right_map: HashMap<i32, i32> =
        input
            .right_list
            .iter()
            .fold(HashMap::new(), |mut map, &num| {
                *map.entry(num).or_insert(0) += 1;
                map
            });

    let mut total = 0;
    for num in input.left_list {
        total += num * right_map.get(&num).unwrap_or(&0);
    }
    println!("{}", total);
}
