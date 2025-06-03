use std::collections::HashSet;

#[derive(Debug)]
struct Grid {
    tiles: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    #[allow(dead_code)]
    fn new() -> Self {
        let tiles: Vec<Vec<char>> = include_str!("../inputs/day08.txt")
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let rows = tiles.len();
        let cols = tiles[0].len();
        Grid { tiles, rows, cols }
    }
}

#[derive(Debug)]
struct Delta {
    row: i32,
    col: i32,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Coordinate {
    row: usize,
    col: usize,
}

impl Coordinate {
    #[allow(dead_code)]
    fn new(grid: &Grid, row: i32, col: i32) -> Option<Self> {
        let unchecked_coordinate = Delta { row, col };
        if unchecked_coordinate.row < 0
            || unchecked_coordinate.row >= grid.rows as i32
            || unchecked_coordinate.col < 0
            || unchecked_coordinate.col >= grid.cols as i32
        {
            return None;
        }
        Some(Self {
            row: unchecked_coordinate.row as usize,
            col: unchecked_coordinate.col as usize,
        })
    }

    fn antidelta(coordinate1: &Coordinate, coordinate2: &Coordinate) -> Delta {
        let row = coordinate1.row as i32 - coordinate2.row as i32;
        let col = coordinate1.col as i32 - coordinate2.col as i32;

        Delta { row, col }
    }

    fn traverse(grid: &Grid, coordinate: &Coordinate, delta: &Delta, steps: i32) -> Option<Self> {
        let row = coordinate.row as i32 + (delta.row * steps);
        let col = coordinate.col as i32 + (delta.col * steps);

        Coordinate::new(grid, row, col)
    }

    #[allow(dead_code)]
    fn nearest_antinode_position(
        pivot_coordinate: &Coordinate,
        evaluate_coordinate: &Coordinate,
        grid: &Grid,
    ) -> Option<Self> {
        let antidelta = Self::antidelta(pivot_coordinate, evaluate_coordinate);

        Self::traverse(grid, pivot_coordinate, &antidelta, 1)
    }

    #[allow(dead_code)]
    fn all_antinode_positions(
        pivot_coordinate: &Coordinate,
        evaluate_coordinate: &Coordinate,
        grid: &Grid,
    ) -> Option<HashSet<Self>> {
        let mut antinodes: HashSet<Self> =
            HashSet::from([pivot_coordinate.clone(), evaluate_coordinate.clone()]);
        let mut steps = 1;
        let antidelta = Self::antidelta(pivot_coordinate, evaluate_coordinate);

        while let Some(antinode) = Self::traverse(grid, pivot_coordinate, &antidelta, steps) {
            antinodes.insert(antinode);
            steps += 1;
        }

        Some(antinodes)
    }
}

#[allow(dead_code)]
pub fn part_one() {
    let grid = Grid::new();
    let mut antinodes: HashSet<Coordinate> = HashSet::new();

    // traverse grid evaluating non '.'
    for r in 0..grid.rows {
        for c in 0..grid.cols {
            let pivot_position = Coordinate::new(&grid, r as i32, c as i32).unwrap();
            let pivot_char = grid.tiles[pivot_position.row][pivot_position.col];

            // evaluation occurs here
            if pivot_char != '.' {
                // traverse grid again evaluating matching frequencies at other positions
                for rr in 0..grid.rows {
                    for cc in 0..grid.cols {
                        let evaluate_position =
                            Coordinate::new(&grid, rr as i32, cc as i32).unwrap();
                        let evaluate_char =
                            grid.tiles[evaluate_position.row][evaluate_position.col];

                        // evaluation occurs here
                        if pivot_char == evaluate_char && pivot_position != evaluate_position {
                            if let Some(antinode) = Coordinate::nearest_antinode_position(
                                &pivot_position,
                                &evaluate_position,
                                &grid,
                            ) {
                                antinodes.insert(antinode);
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", antinodes.len());
}

#[allow(dead_code)]
pub fn part_two() {
    let grid = Grid::new();
    let mut antinodes: HashSet<Coordinate> = HashSet::new();

    // traverse grid evaluating non '.'
    for r in 0..grid.rows {
        for c in 0..grid.cols {
            let pivot_position = Coordinate::new(&grid, r as i32, c as i32).unwrap();
            let pivot_char = grid.tiles[pivot_position.row][pivot_position.col];

            // evaluation occurs here
            if pivot_char != '.' {
                // traverse grid again evaluating matching frequencies at other positions
                for rr in 0..grid.rows {
                    for cc in 0..grid.cols {
                        let evaluate_position =
                            Coordinate::new(&grid, rr as i32, cc as i32).unwrap();
                        let evaluate_char =
                            grid.tiles[evaluate_position.row][evaluate_position.col];

                        // evaluation occurs here
                        if pivot_char == evaluate_char && pivot_position != evaluate_position {
                            if let Some(new_antinodes) = Coordinate::all_antinode_positions(
                                &pivot_position,
                                &evaluate_position,
                                &grid,
                            ) {
                                antinodes.extend(new_antinodes.clone());
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", antinodes.len());
}
