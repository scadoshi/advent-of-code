use std::collections::HashMap;

#[allow(dead_code)]
pub fn part_one() {
    println!(
        "{:?}",
        include_str!("../inputs/day01.txt")
            .lines()
            .map(|line| {
                let left = line
                    .chars()
                    .find(|char| char.to_digit(10).is_some())
                    .expect(format!("error fining left number in {line}").as_str());
                let right = line
                    .chars()
                    .rev()
                    .find(|char| char.to_digit(10).is_some())
                    .expect(format!("error fining right number in {line}").as_str());
                (left.to_string() + right.to_string().as_str())
                    .parse::<i32>()
                    .unwrap()
            })
            .sum::<i32>()
    );
}

#[allow(dead_code)]
fn nums() -> HashMap<String, char> {
    HashMap::from([
        ("one".to_string(), '1'),
        ("two".to_string(), '2'),
        ("three".to_string(), '3'),
        ("four".to_string(), '4'),
        ("five".to_string(), '5'),
        ("six".to_string(), '6'),
        ("seven".to_string(), '7'),
        ("eight".to_string(), '8'),
        ("nine".to_string(), '9'),
    ])
}

#[allow(dead_code)]
fn find_left(x: &str) -> Option<i32> {
    let nums = nums();

    for i in 0..x.len() {
        let slice = &x[i..];
        for (k, v) in nums.iter() {
            if slice.starts_with(k) || slice.starts_with(*v) {
                return Some(v.to_digit(10).unwrap() as i32);
            }
        }
    }
    None
}

#[allow(dead_code)]
fn find_right(x: &str) -> Option<i32> {
    let nums = nums();

    for i in 0..x.len() {
        let j = x.len() - 1 - i;
        let slice = &x[j..];
        for (k, v) in nums.iter() {
            if slice.starts_with(k) || slice.starts_with(*v) {
                return Some(v.to_digit(10).unwrap() as i32);
            }
        }
    }
    None
}

#[allow(dead_code)]
pub fn part_two() {
    println!(
        "{:?}",
        include_str!("../inputs/day01.txt")
            .lines()
            .map(|line| {
                let left =
                    find_left(line).expect(format!("error finding left number in {line}").as_str());
                let right = find_right(line)
                    .expect(format!("error finding right number in {line}").as_str());
                (left.to_string() + right.to_string().as_str())
                    .parse::<i32>()
                    .unwrap()
            })
            .sum::<i32>()
    );
}
