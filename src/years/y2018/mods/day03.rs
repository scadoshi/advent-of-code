use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Square {
    id: i32,
    col_start: i32,
    row_start: i32,
    width: i32,
    height: i32,
}

impl Square {
    #[allow(dead_code)]
    fn new(input: String) -> Self {
        let [id, col_start, row_start, width, height] = input
            .to_string()
            .replace('#', "")
            .replace(':', "")
            .replace(" @", "")
            .replace('x', ",")
            .split_whitespace()
            .flat_map(|x| {
                x.split(',')
                    .map(|y| y.parse().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();

        Self {
            id,
            col_start,
            row_start,
            width,
            height,
        }
    }
}

#[allow(dead_code)]
fn input() -> Vec<Square> {
    include_str!("../inputs/day03.txt")
        .lines()
        .map(|line| line.to_string())
        .map(|x| Square::new(x))
        .collect()
}

#[allow(dead_code)]
fn map_squares(input: &Vec<Square>) -> HashMap<(i32, i32), HashSet<i32>> {
    let mut map: HashMap<(i32, i32), HashSet<i32>> = HashMap::new();

    for square in input.iter() {
        for col_add in 0..square.width {
            for row_add in 0..square.height {
                let col = square.col_start + col_add;
                let row = square.row_start + row_add;
                map.entry((row, col)).or_default().insert(square.id);
            }
        }
    }
    map
}

#[allow(dead_code)]
pub fn part_one() {
    println!(
        "{:?}",
        map_squares(&input())
            .values()
            .filter(|x| x.len() > 1)
            .count()
    );
}

#[allow(dead_code)]
pub fn part_two() {
    let input = input();
    let map = map_squares(&input);

    for square in input {
        let non_overlapping_count = map
            .values()
            .filter(|ids| ids.contains(&square.id) && ids.len() == 1)
            .count() as i32;

        if non_overlapping_count == square.width * square.height {
            println!("{}", square.id);
            break;
        }
    }
}
