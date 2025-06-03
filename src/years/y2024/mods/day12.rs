#![allow(dead_code)]
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Map {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let height = grid.len();
        let width = grid[0].len();
        Map {
            grid,
            height,
            width,
        }
    }

    fn from_input() -> Self {
        Map::new(
            std::fs::read_to_string("src/years/y2024/inputs/day12.txt")
                .expect("Error reading file")
                .lines()
                .map(|line| line.chars().collect())
                .collect(),
        )
    }

    fn get_char(&self, coordinate: &Coordinate) -> char {
        self.grid[coordinate.row][coordinate.col]
    }

    fn build_regions(&self) -> Vec<HashSet<Coordinate>> {
        let directions = [
            Delta::new(-1, 0), // up
            Delta::new(1, 0),  // down
            Delta::new(0, -1), // left
            Delta::new(0, 1),  // right
        ];

        let mut seen: HashSet<Coordinate> = HashSet::new();
        let mut regions: Vec<HashSet<Coordinate>> = Vec::new();

        for r in 0..self.height {
            for c in 0..self.width {
                if let Some(coord) = Coordinate::new(r as isize, c as isize, self) {
                    if seen.contains(&coord) {
                        continue;
                    }
                    seen.insert(coord.clone());

                    let mut region: HashSet<Coordinate> = HashSet::from([coord.clone()]);
                    let mut q: VecDeque<Coordinate> = VecDeque::from([coord.clone()]);

                    while let Some(q_coord) = q.pop_front() {
                        for eval_delta in directions.iter() {
                            if let Some(eval_coord) = q_coord.traverse(&eval_delta, self) {
                                if region.contains(&eval_coord) {
                                    continue;
                                }
                                if self.get_char(&q_coord) != self.get_char(&eval_coord) {
                                    continue;
                                }

                                region.insert(eval_coord.clone());
                                q.push_back(eval_coord.clone());
                            }
                        }
                    }
                    regions.push(region);
                }
            }
        }
        regions
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Coordinate {
    row: usize,
    col: usize,
}

impl Coordinate {
    fn new(row: isize, col: isize, map: &Map) -> Option<Coordinate> {
        Delta::new(row, col).to_coordinate(map)
    }

    fn to_delta(&self) -> Delta {
        Delta::new(self.row as isize, self.col as isize)
    }

    fn traverse(&self, delta: &Delta, map: &Map) -> Option<Coordinate> {
        self.to_delta().add_delta(delta).to_coordinate(map)
    }

    fn perimeter(&self, map: &Map) -> usize {
        let directions: [Delta; 4] = [
            Delta::new(-1, 0), // up
            Delta::new(1, 0),  // down
            Delta::new(0, -1), // left
            Delta::new(0, 1),  // right
        ];
        let start_char = map.get_char(self);
        let mut total = 0;
        for delta in directions {
            if let Some(eval_coord) = self.traverse(&delta, map) {
                let eval_char = map.get_char(&eval_coord);
                if start_char != eval_char {
                    // different character
                    total += 1;
                }
            } else {
                // off map
                total += 1;
            }
        }
        total
    }

    fn adjacent(&self, coordinate: &Coordinate) -> bool {
        let row_diff = (self.row as isize - coordinate.row as isize).abs();
        let col_diff = (self.col as isize - coordinate.col as isize).abs();

        if row_diff == 1 && col_diff == 0 || row_diff == 0 && col_diff == 1 {
            true
        } else {
            false
        }
    }
}

struct Delta {
    row: isize,
    col: isize,
}

impl Delta {
    fn new(row: isize, col: isize) -> Self {
        Delta { row, col }
    }

    fn to_coordinate(&self, map: &Map) -> Option<Coordinate> {
        if self.row >= 0
            && self.row < map.height as isize
            && self.col >= 0
            && self.col < map.width as isize
        {
            Some(Coordinate {
                row: self.row as usize,
                col: self.col as usize,
            })
        } else {
            None
        }
    }

    fn add_delta(&self, delta: &Delta) -> Self {
        Delta {
            row: self.row + delta.row,
            col: self.col + delta.col,
        }
    }
}

pub fn part_one() {
    let map = Map::from_input();
    let regions = map.build_regions();

    let total: usize = regions
        .iter()
        .map(|region| {
            region
                .iter()
                .map(|coord| coord.perimeter(&map))
                .sum::<usize>()
        })
        .sum();
    println!("{:#?}", total);
}

pub fn part_two() {}
