use lazy_static::lazy_static;
use regex::Regex;
use thousands::Separable;

lazy_static! {
    static ref MUL_PATTERN: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)$").unwrap();
    static ref DO_PATTERN: Regex = Regex::new(r"do\(\)$").unwrap();
    static ref DONT_PATTERN: Regex = Regex::new(r"don\'t\(\)$").unwrap();
}

#[allow(dead_code)]
fn input() -> Vec<char> {
    include_str!("../inputs/day03.txt")
        .chars()
        .collect::<Vec<char>>()
}

#[allow(dead_code)]
fn mul_match_incr(win: &String) -> i32 {
    if let Some(caps) = MUL_PATTERN.captures(&win) {
        let (num1, num2) = (
            caps[1].parse::<i32>().unwrap(),
            caps[2].parse::<i32>().unwrap(),
        );
        num1 * num2
    } else {
        0
    }
}

#[allow(dead_code)]
fn intake_check(win: &String, curr_state: &bool) -> bool {
    if DONT_PATTERN.is_match(win) {
        false
    } else if DO_PATTERN.is_match(win) {
        true
    } else {
        *curr_state
    }
}

#[allow(dead_code)]
pub fn part_one() {
    let mut total = 0;

    for win in input()
        .windows(12)
        .map(|win| win.iter().collect::<String>())
    {
        total += mul_match_incr(&win);
    }

    println!("{}", total.separate_with_commas());
}

#[allow(dead_code)]
pub fn part_two() {
    let mut total = 0;
    let mut intaking = true;

    for win in input()
        .windows(12)
        .map(|win| win.iter().collect::<String>())
    {
        intaking = intake_check(&win, &intaking);

        if intaking {
            total += mul_match_incr(&win);
        }
    }

    println!("{}", total.separate_with_commas());
}
