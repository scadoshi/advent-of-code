const ALL_MOVES: [(i32, i32); 8] = [
    (-1, 0),  // up
    (1, 0),   // down
    (0, -1),  // left
    (0, 1),   // right
    (-1, -1), // up-left
    (-1, 1),  // up-right
    (1, -1),  // down-left
    (1, 1),   // down-right
];

#[allow(dead_code)]
fn input() -> Vec<Vec<char>> {
    include_str!("../inputs/day04.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[allow(dead_code)]
pub fn part_one() {
    let input = input();
    let rows = input.len() as i32;
    let cols = input[0].len() as i32;

    let mut total = 0;

    for r in 0..rows {
        for c in 0..cols {
            let curr_char = input[r as usize][c as usize];
            if curr_char == 'X' {
                for (dr, dc) in ALL_MOVES {
                    // generate final position of direction change to make sure we're in bounds
                    let (final_r, final_c) = ((r + (dr * 3)), (c + (dc * 3)));
                    // in bounds
                    if final_r >= 0 && final_r < rows && final_c >= 0 && final_c < cols {
                        for steps in 1..4 {
                            // generate current position change coordinate
                            let (new_r, new_c) =
                                ((r + (dr * steps)) as usize, (c + (dc * steps)) as usize);
                            let new_char = input[new_r][new_c];
                            // when to move onto the next move in ALL_MOVES
                            if (steps == 1 && new_char != 'M')
                                || (steps == 2 && new_char != 'A')
                                || (steps == 3 && new_char != 'S')
                            {
                                break;
                            // when to increment total
                            } else if steps == 3 && new_char == 'S' {
                                total += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{:?}", total);
}

#[allow(dead_code)]
const MAIN_DIAG_MOVES: ((i32, i32), (i32, i32)) = (
    (-1, 1), // up-right
    (1, -1), // down-left
);

#[allow(dead_code)]
const ANTI_DIAG_MOVES: ((i32, i32), (i32, i32)) = (
    (-1, -1), // up-left
    (1, 1),   // down-right
);

#[allow(dead_code)]
pub fn part_two() {
    let input = input();
    let rows = input.len() as i32;
    let cols = input[0].len() as i32;

    let mut total = 0;

    for r in 0..rows {
        for c in 0..cols {
            let curr_char = input[r as usize][c as usize];
            if curr_char == 'A' {
                let mut diag_pass_count = 0;
                // iter through moves in both diagonals
                for (move1, move2) in [MAIN_DIAG_MOVES, ANTI_DIAG_MOVES] {
                    // generate new positions
                    let (new_pos1, new_pos2): ((i32, i32), (i32, i32)) =
                        ((r + move1.0, c + move1.1), (r + move2.0, c + move2.1));

                    // in bounds
                    if new_pos1.0 >= 0
                        && new_pos1.1 >= 0
                        && new_pos2.0 >= 0
                        && new_pos2.1 >= 0
                        && new_pos1.0 < rows
                        && new_pos1.1 < cols
                        && new_pos2.0 < rows
                        && new_pos2.1 < cols
                    {
                        // safe to generate characters
                        let (char1, char2) = (
                            input[new_pos1.0 as usize][new_pos1.1 as usize],
                            input[new_pos2.0 as usize][new_pos2.1 as usize],
                        );
                        // compare characters
                        if "MS".contains(char1) && "MS".contains(char2) && char1 != char2 {
                            diag_pass_count += 1;
                        }
                        if diag_pass_count == 2 {
                            total += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", total);
}
