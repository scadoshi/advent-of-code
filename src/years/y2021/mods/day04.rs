#[allow(dead_code)]
fn nums() -> Vec<u32> {
    include_str!("../inputs/day04.txt")
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn boards() -> Vec<Vec<Vec<(u32, bool)>>> {
    let mut boards: Vec<Vec<Vec<(u32, bool)>>> = Vec::new();
    let mut board: Vec<Vec<(u32, bool)>> = Vec::new();

    for line in include_str!("../inputs/day04.txt").lines().skip(2) {
        if line.trim().is_empty() {
            boards.push(board.clone());
            board.clear();
        } else {
            board.push(
                line.split_whitespace()
                    .map(|x| (x.parse().unwrap(), false))
                    .collect(),
            );
        }
    }
    if !board.is_empty() {
        boards.push(board.clone());
    }
    boards
}

#[allow(dead_code)]
fn is_bingo(board: &Vec<Vec<(u32, bool)>>) -> bool {
    if board.is_empty() {
        return false;
    }

    // row
    if board
        .iter()
        .any(|line| line.iter().all(|(_, hit)| *hit == true))
    {
        return true;
    }

    // col
    for i in 0..5 {
        if board.iter().all(|line| line[i].1 == true) {
            return true;
        }
    }

    false
}

#[allow(dead_code)]
pub fn part_one() {
    let mut boards = boards();
    // println!("{:?}", boards);
    let mut last_num;

    'main: for num in nums() {
        last_num = num;
        for board in boards.iter_mut() {
            // find position of line with matching num
            if let Some(line_i) = board
                .iter()
                .position(|line| line.iter().any(|(board_num, _)| *board_num == num))
            {
                // find position of col with matching num
                if let Some(col_i) = board[line_i]
                    .iter()
                    .position(|(board_num, _)| *board_num == num)
                {
                    board[line_i][col_i].1 = true;
                }
            }

            if is_bingo(&board) {
                let sum_of_non_hits = board
                    .iter()
                    .map(|line| {
                        line.iter()
                            .filter(|(_, hit)| !hit)
                            .map(|(board_num, _)| board_num)
                            .sum::<u32>()
                    })
                    .sum::<u32>();

                // println!("{:?}", board);
                println!("{}", sum_of_non_hits * last_num);
                break 'main;
            }
        }
    }
}

#[allow(dead_code)]
pub fn part_two() {
    let mut total_wins = 0;

    let mut boards = boards();
    let final_win = boards.len();
    let mut last_num;

    'main: for num in nums() {
        last_num = num;
        for board in boards.iter_mut() {
            // skip if board already won
            if is_bingo(&board) {
                continue;
            }

            // find position of line with matching num
            if let Some(line_i) = board
                .iter()
                .position(|line| line.iter().any(|(board_num, _)| *board_num == num))
            {
                // find position of col with matching num
                if let Some(col_i) = board[line_i]
                    .iter()
                    .position(|(board_num, _)| *board_num == num)
                {
                    board[line_i][col_i].1 = true;
                }
            }

            if is_bingo(&board) {
                total_wins += 1;
                if total_wins == final_win {
                    let sum_of_non_hits = board
                        .iter()
                        .map(|line| {
                            line.iter()
                                .filter(|(_, hit)| !hit)
                                .map(|(board_num, _)| board_num)
                                .sum::<u32>()
                        })
                        .sum::<u32>();

                    // println!("{:?}", board);
                    println!("{}", sum_of_non_hits * last_num);
                    break 'main;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::is_bingo;

    #[ignore]
    #[allow(dead_code)]
    fn is_bingo_test_row() {
        let input: Vec<Vec<(u32, bool)>> = vec![
            vec![(1, true), (1, true), (1, true), (1, true), (1, true)],
            vec![(1, false), (1, false), (1, false), (1, false), (1, false)],
            vec![(1, false), (1, false), (1, false), (1, false), (1, false)],
            vec![(1, false), (1, false), (1, false), (1, false), (1, false)],
            vec![(1, false), (1, false), (1, false), (1, false), (1, false)],
        ];
        let result = is_bingo(&input);
        assert_eq!(result, true);
    }
    #[ignore]
    #[allow(dead_code)]
    fn is_bingo_test_col() {
        let input: Vec<Vec<(u32, bool)>> = vec![
            vec![(1, true), (1, false), (1, false), (1, false), (1, false)],
            vec![(1, true), (1, false), (1, false), (1, false), (1, false)],
            vec![(1, true), (1, false), (1, false), (1, false), (1, false)],
            vec![(1, true), (1, false), (1, false), (1, false), (1, false)],
            vec![(1, true), (1, false), (1, false), (1, false), (1, false)],
        ];
        let result = is_bingo(&input);
        assert_eq!(result, true);
    }

    #[ignore]
    #[allow(dead_code)]
    fn is_bingo_test_none() {
        let input: Vec<Vec<(u32, bool)>> = vec![
            vec![(1, false), (1, false), (1, false), (1, false), (1, false)],
            vec![(1, true), (1, false), (1, false), (1, false), (1, false)],
            vec![(1, true), (1, false), (1, false), (1, false), (1, false)],
            vec![(1, true), (1, false), (1, false), (1, false), (1, false)],
            vec![(1, true), (1, false), (1, false), (1, false), (1, false)],
        ];
        let result = is_bingo(&input);
        assert_eq!(result, false);
    }
}
