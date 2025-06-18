use std::collections::HashMap;

#[allow(dead_code)]
fn input() -> i32 {
    include_str!("../inputs/day03.txt")
        .parse()
        .expect("error converting input to i32")
}

#[allow(dead_code)]
enum Direction {
    Right,
    Left,
    Down,
    Up,
}

impl Direction {
    #[allow(dead_code)]
    fn turn_left(&self) -> Self {
        match self {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        }
    }
}

#[allow(dead_code)]
fn mmove(x: i32, y: i32, dir: &Direction) -> (i32, i32) {
    match dir {
        Direction::Right => (x + 1, y),
        Direction::Up => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Down => (x, y - 1),
    }
}

#[allow(dead_code)]
fn adj_sum(x: i32, y: i32, visited: &HashMap<(i32, i32), i32>) -> i32 {
    let adj_points = [
        (x + 1, y),
        (x - 1, y),
        (x, y + 1),
        (x, y - 1),
        (x + 1, y + 1),
        (x - 1, y - 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
    ];

    adj_points
        .iter()
        .fold(0, |total, point| total + visited.get(point).unwrap_or(&0))
}

pub fn part_one() {
    let (mut x, mut y) = (0, 0);
    let target = input();
    let mut num = 1;
    let mut dir = Direction::Right;
    let mut move_length = 1;

    'main: loop {
        for _ in 0..2 {
            for _ in 0..move_length {
                (x, y) = mmove(x, y, &dir);
                num += 1;
                if num == target {
                    break 'main;
                }
            }
            dir = dir.turn_left();
        }
        move_length += 1;
    }

    println!("{}", x.abs() + y.abs());
}

pub fn part_two() {
    let (mut x, mut y) = (0, 0);
    let target = input();
    let mut num = 1;
    let mut dir = Direction::Right;
    let mut move_length = 1;
    let mut visited: HashMap<(i32, i32), i32> = HashMap::new();

    'main: loop {
        for _ in 0..2 {
            for _ in 0..move_length {
                visited.insert((x, y), num);
                (x, y) = mmove(x, y, &dir);
                num = adj_sum(x, y, &visited);
                // println!("{} ({}, {}) {:#?}", num, x, y, visited); // debug
                // std::thread::sleep(std::time::Duration::from_secs(1)); // debug
                if num > target {
                    break 'main;
                }
            }
            dir = dir.turn_left();
        }
        move_length += 1;
    }

    println!("{}", num);
}
