use std::collections::HashSet;

#[allow(dead_code)]
#[derive(Debug)]
struct Movement {
    direction: char,
    distance: i32,
}

impl Movement {
    fn new(input_str: &str) -> Self {
        let direction = input_str
            .chars()
            .next()
            .expect(format!("error finding direction from {}", input_str).as_str());
        let distance = input_str
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<i32>()
            .expect(format!("error finding direction from {}", input_str).as_str());
        Self {
            direction,
            distance,
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(self, movement: &Movement) -> Self {
        match movement.direction {
            'R' => match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
            'L' => match self {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
            },
            _ => {
                println!("f");
                self
            }
        }
    }
}

fn manhattan_distance(x: i32, y: i32) -> i32 {
    x.abs() + y.abs()
}

/// both parts
pub fn part_one() {
    let instructions: Vec<Movement> = include_str! {"../inputs/day01.txt"}
        .replace(',', "")
        .split_whitespace()
        .map(|x| Movement::new(x))
        .collect();

    let (mut x, mut y) = (0, 0);
    let mut direction = Direction::Up;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut twice_visited: Option<(i32, i32)> = None;
    let mut twice_visited_distance: Option<i32> = None;

    for movement in instructions {
        // println!("{:?}", movement);
        direction = direction.turn(&movement);

        for _ in 0..movement.distance {
            (x, y) = match direction {
                Direction::Up => (x, y + 1),
                Direction::Right => (x + 1, y),
                Direction::Down => (x, y - 1),
                Direction::Left => (x - 1, y),
            };

            if visited.contains(&(x, y)) && twice_visited.is_none() {
                // println!("{:?} was visited for the second time here", (x, y));
                twice_visited = Some((x, y));
                twice_visited_distance = Some(manhattan_distance(x, y));
            }
            visited.insert((x, y));
        }

        // println!("{:?} {}", direction, movement.distance);
        // println!("{:?}", (x, y));
    }

    let final_distance = manhattan_distance(x, y);

    println!("{final_distance}");
    println!("{}", twice_visited_distance.unwrap());
}
pub fn part_two() {}
