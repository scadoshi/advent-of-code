use std::collections::HashMap;

fn input() -> Vec<Vec<char>> {
    include_str!("../inputs/day03.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn adjacents(r: isize, c: isize) -> [(isize, isize); 8] {
    [
        (r + 1, c),     // down
        (r - 1, c),     // up
        (r, c + 1),     // right
        (r, c - 1),     // left
        (r + 1, c - 1), // down left
        (r + 1, c + 1), // down right
        (r - 1, c + 1), // up right
        (r - 1, c - 1), // up left
    ]
}

#[allow(dead_code)]
fn is_part(r: isize, c: isize, input: &Vec<Vec<char>>) -> bool {
    let symbols = ['*', '@', '/', '=', '$', '%', '#', '-', '+', '&'];
    for (dr, dc) in adjacents(r, c) {
        // limits
        if dr < 0 || dc < 0 || dr >= input.len() as isize || dc >= input[0].len() as isize {
            continue;
        }
        if symbols.contains(&input[dr as usize][dc as usize]) {
            return true;
        }
    }
    false
}

fn gear_check(r: isize, c: isize, input: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (dr, dc) in adjacents(r, c) {
        // limits
        if dr < 0 || dc < 0 || dr >= input.len() as isize || dc >= input[0].len() as isize {
            continue;
        }
        if input[dr as usize][dc as usize] == '*' {
            return Some((dr as usize, dc as usize));
        }
    }
    None
}

#[allow(dead_code)]
pub fn part_one() {
    let input = input();
    let mut num: Vec<u32> = Vec::new();
    let mut valid_nums: Vec<u32> = Vec::new();
    let mut valid = false;
    let rows = input.len();
    let cols = input[0].len();
    for r in 0..rows {
        for c in 0..cols {
            if let Some(digit) = input[r][c].to_digit(10) {
                num.push(digit);
                valid = valid || is_part(r as isize, c as isize, &input);
            } else if valid {
                let valid_num = num
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
                valid_nums.push(valid_num);
                num.clear();
                valid = false;

                // // debug
                // if r == 51 && c >137 && c < 140
                // || r == 53 && c > 137 && c < 140 {
                //     println!("{}", valid_num);
                // }
                // // debug
            } else {
                num.clear();
                valid = false;
            }
        }
        // check num at end of line
        if valid {
            let valid_num = num
                .iter()
                .map(|x| x.to_string())
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
            valid_nums.push(valid_num);
            num.clear();
            valid = false;
        } else {
            num.clear();
            valid = false;
        }
    }
    println!("{}", valid_nums.iter().sum::<u32>());
}

#[allow(dead_code)]
pub fn part_two() {
    let input = input();
    let mut num: Vec<u32> = Vec::new();
    let mut valid = false;
    let mut gear_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let mut gear_poss: Vec<(usize, usize)> = Vec::new();
    let rows = input.len();
    let cols = input[0].len();
    for r in 0..rows {
        for c in 0..cols {
            if let Some(digit) = input[r][c].to_digit(10) {
                num.push(digit);

                if let Some(gear_pos) = gear_check(r as isize, c as isize, &input) {
                    gear_poss.push(gear_pos);
                }

                valid = valid || !gear_poss.is_empty();
            } else if valid {
                let valid_num = num
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
                gear_map
                    .entry(gear_poss.remove(0))
                    .or_default()
                    .push(valid_num);
                num.clear();
                valid = false;
                gear_poss.clear();

            // // debug
            // if r == 51 && c >137 && c < 140
            // || r == 53 && c > 137 && c < 140 {
            //     println!("{}", valid_num);
            // }
            // // debug
            } else {
                num.clear();
                valid = false;
                gear_poss.clear();
            }
        }
        // check num at end of line
        if valid {
            let valid_num = num
                .iter()
                .map(|x| x.to_string())
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
            gear_map
                .entry(gear_poss.remove(0))
                .or_default()
                .push(valid_num);
            num.clear();
            valid = false;
        } else {
            num.clear();
            valid = false;
        }
    }
    let result = gear_map
        .iter()
        .filter(|(_, num_list)| num_list.len() == 2)
        .map(|(_, num_list)| num_list.iter().product::<u32>())
        .sum::<u32>();

    println!("{}", result);
}
