#[allow(dead_code)]
#[derive(Debug)]
struct Map {
    grid: Vec<Vec<usize>>,
    row_limit: usize,
    col_limit: usize,
}

impl Map {
    #[allow(dead_code)]
    fn from_string(input: String) -> Self {
        let grid: Vec<Vec<usize>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();

        let row_limit = grid.len();
        let col_limit = grid[0].len();

        Self {
            grid,
            row_limit,
            col_limit,
        }
    }

    #[allow(dead_code)]
    fn from_input() -> Self {
        Self::from_string(std::fs::read_to_string("src/years/y2024/inputs/day10.txt").unwrap())
    }

    #[allow(dead_code)]
    fn collect_start_positions(&self) -> Vec<Coordinate> {
        self.grid
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut start_positions, (row_i, row)| {
                start_positions.extend(row.iter().enumerate().filter_map(|(col_i, col)| {
                    if *col == 0 {
                        Some(Coordinate::new(row_i, col_i))
                    } else {
                        None
                    }
                }));
                start_positions
            })
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Delta {
    row_d: isize,
    col_d: isize,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
struct Coordinate {
    row_i: usize,
    col_i: usize,
}

impl Delta {
    #[allow(dead_code)]
    fn new(row_d: isize, col_d: isize) -> Self {
        Self { row_d, col_d }
    }

    #[allow(dead_code)]
    fn to_coordinate(&self, map: &Map) -> Option<Coordinate> {
        if self.row_d < 0
            || self.col_d < 0
            || self.row_d >= map.row_limit as isize
            || self.col_d >= map.col_limit as isize
        {
            None
        } else {
            Some(Coordinate::new(self.row_d as usize, self.col_d as usize))
        }
    }
}

impl Coordinate {
    #[allow(dead_code)]
    fn new(row_i: usize, col_i: usize) -> Self {
        Self { row_i, col_i }
    }

    #[allow(dead_code)]
    fn move_coordinate(&self, delta: Delta, map: &Map) -> Option<Coordinate> {
        let row_d = self.row_i as isize + delta.row_d;
        let col_d = self.col_i as isize + delta.col_d;
        Delta::new(row_d, col_d).to_coordinate(map)
    }

    #[allow(dead_code)]
    fn count_reaching_paths(&self, map: &Map) -> i32 {
        let mut total = 0;
        let current_height = self.get_height(map);

        let directions: [Delta; 4] = [
            Delta::new(-1, 0), // up
            Delta::new(1, 0),  // down
            Delta::new(0, -1), // left
            Delta::new(0, 1),  // right
        ];

        if current_height == 9 {
            return 1;
        }

        // recursion happens here
        for direction in directions {
            let new_coordinate = self.move_coordinate(direction, map);
            if new_coordinate.is_some()
                && current_height + 1 == new_coordinate.as_ref().unwrap().get_height(map)
            {
                total += new_coordinate.unwrap().count_reaching_paths(map);
            }
        }
        total
    }

    #[allow(dead_code)]
    fn count_reachable_points(&self, map: &Map) -> i32 {
        let mut visited = Vec::new();
        self.count_reachable_points_recursive(map, &mut visited)
    }

    #[allow(dead_code)]
    fn count_reachable_points_recursive(&self, map: &Map, visited: &mut Vec<Coordinate>) -> i32 {
        if visited.contains(self) {
            return 0;
        }

        let current_height = self.get_height(map);
        if current_height == 9 {
            visited.push(self.clone());
            return 1;
        }

        visited.push(self.clone());
        let mut total = 0;

        let directions = [
            Delta::new(-1, 0), // up
            Delta::new(1, 0),  // down
            Delta::new(0, -1), // left
            Delta::new(0, 1),  // right
        ];

        for direction in directions {
            if let Some(new_coordinate) = self.move_coordinate(direction, map) {
                if current_height + 1 == new_coordinate.get_height(map) {
                    total += new_coordinate.count_reachable_points_recursive(map, visited);
                }
            }
        }
        total
    }

    #[allow(dead_code)]
    fn get_height(&self, map: &Map) -> usize {
        map.grid[self.row_i][self.col_i]
    }
}

#[allow(dead_code)]
pub fn part_one() {
    let map = Map::from_input();
    let start_positions = map.collect_start_positions();
    let total_paths: i32 = start_positions
        .iter()
        .map(|start_position| start_position.count_reachable_points(&map))
        .sum();
    println!("{}", total_paths);
}

#[allow(dead_code)]
pub fn part_two() {
    let map = Map::from_input();
    let start_positions = map.collect_start_positions();
    let total_paths: i32 = start_positions
        .iter()
        .map(|start_position| start_position.count_reaching_paths(&map))
        .sum();
    println!("{}", total_paths);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[allow(dead_code)]
    fn test_start_positions() {
        let map = Map::from_string(
            "103\n\
                                                056\n\
                                                709\n\
                                                010\n\
                                                890"
            .to_string(),
        );
        let start_positions = map.collect_start_positions();
        assert_eq!(start_positions.len(), 6);
    }

    #[ignore]
    #[allow(dead_code)]
    fn test_count_reaching_paths() {
        let test_input = "012089000\n\
                                  083078890\n\
                                  804569000\n\
                                  765890000\n\
                                  006789000"
            .to_string();
        let map = Map::from_string(test_input);
        let start_positions = map.collect_start_positions();
        let total_paths: i32 = start_positions
            .iter()
            .map(|start_position| start_position.count_reaching_paths(&map))
            .sum();
        assert_eq!(total_paths, 6);
    }

    #[ignore]
    #[allow(dead_code)]
    fn test_basic_map() {
        let test_input = "0123\n\
                                  9994\n\
                                  9895\n\
                                  9876"
            .to_string();
        let map = Map::from_string(test_input);
        let start_positions = map.collect_start_positions();
        let total_paths: i32 = start_positions
            .iter()
            .map(|start_position| start_position.count_reaching_paths(&map))
            .sum();
        assert_eq!(total_paths, 1); // Only one path should reach a 9
    }

    #[ignore]
    #[allow(dead_code)]
    fn test_single_trailhead() {
        let test_input = "9990999\n\
                                  9991999\n\
                                  9992999\n\
                                  6543456\n\
                                  7999997\n\
                                  8899988\n\
                                  9999999"
            .to_string();
        let map = Map::from_string(test_input);
        let start_positions = map.collect_start_positions();
        let total_paths: i32 = start_positions
            .iter()
            .map(|start_position| start_position.count_reaching_paths(&map))
            .sum();
        assert_eq!(total_paths, 2); // One trailhead can reach two 9s
    }

    // This test might help identify if we're double-counting paths
    #[ignore]
    #[allow(dead_code)]
    fn test_simple_path() {
        let test_input = "01\n\
                                  23\n\
                                  45\n\
                                  67\n\
                                  89"
        .to_string();
        let map = Map::from_string(test_input);
        let start_positions = map.collect_start_positions();
        let total_paths: i32 = start_positions
            .iter()
            .map(|start_position| start_position.count_reaching_paths(&map))
            .sum();
        assert_eq!(total_paths, 0); // Only one possible path
    }

    #[ignore]
    #[allow(dead_code)]
    fn test_near_misses() {
        let test_input = "01234567800\n\
                                  09345078000\n\
                                  00406080000\n\
                                  08587090000\n\
                                  87678000000"
            .to_string();
        let map = Map::from_string(test_input);
        let start_positions = map.collect_start_positions();
        let total_paths: i32 = start_positions
            .iter()
            .map(|start_position| start_position.count_reaching_paths(&map))
            .sum();
        assert_eq!(total_paths, 1);
    }
}
