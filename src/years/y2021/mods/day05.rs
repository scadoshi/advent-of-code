use std::collections::HashSet;

#[allow(dead_code)]
fn input() -> Vec<((i32, i32), (i32, i32))> {
    include_str!("../inputs/day05.txt")
        .lines()
        .map(|x| {
            let v: Vec<(i32, i32)> = x
                .split(" -> ")
                .map(|y| {
                    let v: Vec<i32> = y.split(',').map(|z| z.parse().unwrap()).collect();
                    let x = v[0];
                    let y = v[1];
                    (x, y)
                })
                .collect();
            let start = v[0];
            let end = v[1];
            (start, end)
        })
        .collect()
}

#[allow(dead_code)]
fn line_from_range(
    ((x1, y1), (x2, y2)): ((i32, i32), (i32, i32)),
    ignore_diag: bool,
) -> Vec<(i32, i32)> {
    let mut points: Vec<(i32, i32)> = Vec::new();
    // horizontal line
    if y1 == y2 {
        let start = if x1 < x2 { x1 } else { x2 };
        let dist = (x1 - x2).abs() as i32;
        for d in 0..=dist {
            points.push((start + d, y1));
        }
    }
    // vertical line
    else if x1 == x2 {
        let start = if y1 < y2 { y1 } else { y2 };
        let dist = (y1 - y2).abs() as i32;
        for d in 0..=dist {
            points.push((x1, start + d));
        }
    }
    // diagonal line
    else if !ignore_diag {
        let x_dir = (x2 - x1).signum();
        let y_dir = (y2 - y1).signum();
        let dist = (x1 - x2).abs();

        for d in 0..=dist {
            points.push((x1 + (x_dir * d), y1 + (y_dir * d)));
        }
    }
    points
}

#[allow(dead_code)]
pub fn part_one() {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut overlaps: HashSet<(i32, i32)> = HashSet::new();
    for range in input() {
        for point in line_from_range(range, true) {
            if visited.contains(&point) {
                overlaps.insert(point);
            } else {
                visited.insert(point);
            }
        }
    }
    println!("{}", overlaps.len());
}

#[allow(dead_code)]
pub fn part_two() {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut overlaps: HashSet<(i32, i32)> = HashSet::new();
    for range in input() {
        for point in line_from_range(range, false) {
            if visited.contains(&point) {
                overlaps.insert(point);
            } else {
                visited.insert(point);
            }
        }
    }
    println!("{}", overlaps.len());
    let range = ((1, 1), (3, 3));
    println!("{:?}", line_from_range(range, false));
}
