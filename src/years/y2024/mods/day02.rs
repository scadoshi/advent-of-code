#[allow(dead_code)]
fn delta_vector(vector: &Vec<i32>) -> Vec<i32> {
    vector.windows(2).map(|win| win[0] - win[1]).collect()
}

#[allow(dead_code)]
fn delta_vector_is_safe(delta_vector: Vec<i32>) -> bool {
    let first_sign = delta_vector[0].signum();
    for delta in delta_vector {
        let current_sign = delta.signum();
        // rules:
        // 1. sign can't change throughout the delta_vector (indicating report levels changed direction)
        // 2. delta can't be 0 (no change)
        // 3. delta can't be greater than 3 (too great of change)
        if current_sign != first_sign || delta == 0 || delta.abs() > 3 {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
fn get_reports_from_input() -> Vec<Vec<i32>> {
    include_str!("../inputs/day02.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

#[allow(dead_code)]
fn categorize_reports(reports: Vec<Vec<i32>>) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    reports.iter().fold(
        (Vec::new(), Vec::new()),
        |mut reports_categorized, report| {
            if delta_vector_is_safe(delta_vector(&report)) {
                reports_categorized.0.push(report.clone())
            } else {
                reports_categorized.1.push(report.clone())
            }
            reports_categorized
        },
    )
}

#[allow(dead_code)]
pub fn part_one() {
    let (safe_reports, _unsafe_reports) = categorize_reports(get_reports_from_input());
    println!("{}", safe_reports.len());
}

#[allow(dead_code)]
pub fn part_two() {
    let (safe_reports, unsafe_reports) = categorize_reports(get_reports_from_input());
    let mut newly_safe_reports: Vec<Vec<i32>> = Vec::new();
    for ri in 0..unsafe_reports.len() {
        for li in 0..unsafe_reports[ri].len() {
            let mut temp = unsafe_reports[ri].clone();
            temp.remove(li);
            if delta_vector_is_safe(delta_vector(&temp)) {
                newly_safe_reports.push(unsafe_reports[ri].clone());
                break;
            }
        }
    }
    println!("{}", safe_reports.len() + newly_safe_reports.len());
}
