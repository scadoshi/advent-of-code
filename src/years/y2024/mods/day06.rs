use std::collections::HashMap;

/// Represents a validated position on the grid
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Coordinate {
    row: usize,
    col: usize,
}

/// Represents a position that hasn't been bounds-checked yet
#[allow(dead_code)]
struct UncheckedCoordinate {
    row: i32,
    col: i32,
}

/// Represents the four possible directions the guard can face/move
#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    /// Returns the new direction after turning 90 degrees right
    #[allow(dead_code)]
    fn turn_right(current_dir: &Direction) -> Direction {
        match current_dir {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }
}

#[derive(Clone)]
#[allow(dead_code)]
struct Guard {
    pos: Coordinate,
    dir: Direction,
}

/// Represents a grid where a guard patrols and tracks their movement
#[allow(dead_code)]
struct Grid {
    tiles: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    guard: Option<Guard>,
}

impl Grid {
    #[allow(dead_code)]
    fn new() -> Self {
        // Read the input file and convert each line into a vector of characters
        // Each line becomes a row, and each character becomes a column element
        let tiles: Vec<Vec<char>> = include_str!("../inputs/day06.txt")
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        // Store grid dimensions for boundary checking later during guard movement
        let rows = tiles.len();
        let cols = tiles[0].len();

        // Locate the guard's initial position by finding the '^' character
        // This represents the guard facing upward at the start
        let mut pos = None;
        for r in 0..rows {
            for c in 0..cols {
                if tiles[r][c] == '^' {
                    pos = Some(Coordinate { row: r, col: c });
                }
            }
        }

        // The guard always starts facing upward according to puzzle rules
        let dir = Some(Direction::Up);

        // Construct the guard with its initial position and direction
        // We can safely unwrap since the input is guaranteed to contain
        // exactly one guard marked by '^'
        let guard = Some(Guard {
            pos: pos.unwrap(),
            dir: dir.unwrap(),
        });

        // Return the fully initialized grid with the guard in place
        Grid {
            tiles,
            rows,
            cols,
            guard,
        }
    }

    /// Calculates the guard's next position and direction based on current state
    #[allow(dead_code)]
    fn next_guard(&self) -> Option<Guard> {
        let current_guard = self.guard.as_ref()?; // Early return if guard is None

        // Calculate potential next position based on current direction
        let next_pos: UncheckedCoordinate = match current_guard.dir {
            Direction::Left => UncheckedCoordinate {
                row: current_guard.pos.row as i32,
                col: current_guard.pos.col as i32 - 1,
            },
            Direction::Right => UncheckedCoordinate {
                row: current_guard.pos.row as i32,
                col: current_guard.pos.col as i32 + 1,
            },
            Direction::Up => UncheckedCoordinate {
                row: current_guard.pos.row as i32 - 1,
                col: current_guard.pos.col as i32,
            },
            Direction::Down => UncheckedCoordinate {
                row: current_guard.pos.row as i32 + 1,
                col: current_guard.pos.col as i32,
            },
        };

        // Return None if next position is out of bounds
        if next_pos.row < 0
            || next_pos.col < 0
            || next_pos.row >= self.rows as i32
            || next_pos.col >= self.cols as i32
        {
            return None;
        }

        // Convert checked position to regular Coordinate
        let next_pos = Coordinate {
            row: next_pos.row as usize,
            col: next_pos.col as usize,
        };

        // Handle collision with obstacle (#)
        let next_char = self.tiles[next_pos.row][next_pos.col];
        if next_char == '#' {
            // Return turned guard
            Some(Guard {
                pos: current_guard.pos.clone(),
                dir: Direction::turn_right(&current_guard.dir),
            })
        } else {
            // Return moved guard
            Some(Guard {
                pos: next_pos,
                dir: current_guard.dir.clone(),
            })
        }
    }

    /// Simulates a guard's movement through a grid and counts unique positions visited
    ///
    /// Takes ownership of a Grid and returns a tuple containing:
    /// - A HashMap mapping visited coordinates to the directions faced at that coordinate
    /// - A boolean indicating if simulation ended due to infinite loop (true) or moving off grid (false)
    ///
    /// The simulation ends when either:
    /// - The guard moves off the grid
    /// - An infinite loop is detected (guard visits same position facing same direction)
    #[allow(dead_code)]
    fn simulate_guard(mut grid: Grid) -> (HashMap<Coordinate, Vec<Direction>>, bool) {
        let mut visited: HashMap<Coordinate, Vec<Direction>> = HashMap::new();

        loop {
            // Get the current guard state
            let current_guard = match &grid.guard {
                Some(guard) => guard.clone(),
                None => break,
            };

            // Exit loop when guard visits same position in same direction
            // Indicating infinite loop
            if visited
                .entry(current_guard.pos.clone())
                .or_default()
                .contains(&current_guard.dir)
            {
                return (visited, true);
            }

            // Mark current position as visited
            visited
                .entry(current_guard.pos.clone())
                .or_default()
                .push(current_guard.dir.clone());

            // Mark current position as blank space
            grid.tiles[current_guard.pos.row][current_guard.pos.col] = '.';

            // Get next position and direction
            grid.guard = Grid::next_guard(&grid);

            // If guard is still on grid, update their position marker
            if let Some(guard) = &grid.guard {
                grid.tiles[guard.pos.row][guard.pos.col] = '^';
            } else {
                break; // Exit loop when guard moves off grid
            }
        }
        (visited, false)
    }
}

/// Solves part one: counts the number of unique positions visited by the guard
#[allow(dead_code)]
pub fn part_one() {
    println!("{}", Grid::simulate_guard(Grid::new()).0.len());
}

#[allow(dead_code)]
pub fn part_two() {
    let visited = Grid::simulate_guard(Grid::new()).0;
    let mut loop_obstacle_positions: Vec<Coordinate> = Vec::new();

    for visited_entry in visited {
        let pos = visited_entry.0;
        let mut grid = Grid::new();
        grid.tiles[pos.row][pos.col] = '#';
        if Grid::simulate_guard(grid).1 {
            loop_obstacle_positions.push(pos);
        }
    }
    println!("{}", loop_obstacle_positions.len());
}
