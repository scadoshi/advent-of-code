use std::{collections::HashSet, isize};
// use tokio::time::Instant;

fn points() -> Vec<(isize, isize)> {
    include_str!("../inputs/day06.txt")
        .lines()
        .filter(|line| !line.contains("//"))
        .map(|line| {
            let parts = line.split_once(",").expect("failed to split once on comma");
            let col = parts
                .0
                .parse::<isize>()
                .expect("failed to parse col to isize");
            let row = parts
                .1
                .trim()
                .parse::<isize>()
                .expect("failed to parse row to isize");
            (col, row)
        })
        .collect()
}

fn bounds(points: &Vec<(isize, isize)>) -> (isize, isize, isize, isize) {
    (
        points
            .iter()
            .map(|point| point.0)
            .min()
            .expect("left not found"),
        points
            .iter()
            .map(|point| point.0)
            .max()
            .expect("right not found"),
        points
            .iter()
            .map(|point| point.1)
            .min()
            .expect("top not found"),
        points
            .iter()
            .map(|point| point.1)
            .max()
            .expect("bottom not found"),
    )
}

#[allow(dead_code)]
fn bigger(bounds: (isize, isize, isize, isize), grow_by: isize) -> (isize, isize, isize, isize) {
    (
        0.max(bounds.0 - (grow_by / 2)),
        bounds.1 + grow_by / 2,
        0.max(bounds.2 - (grow_by / 2)),
        bounds.3 + grow_by / 2,
    )
}

fn distance(p1: &(isize, isize), p2: &(isize, isize)) -> isize {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

#[allow(dead_code)]
fn near_count(
    og_pos: &(isize, isize),
    points: &Vec<(isize, isize)>,
    bounds: (isize, isize, isize, isize),
) -> isize {
    let mut total = 0;
    for col in bounds.0..=bounds.1 {
        for row in bounds.2..=bounds.3 {
            let mut invalids: HashSet<(isize, isize)> = HashSet::new();
            let curr_point = (col, row);
            let mut near_pos = points[0];

            for curr_pos in points.iter().skip(1) {
                let curr_dist = distance(&curr_pos, &curr_point);
                let prev_dist = distance(&near_pos, &curr_point);

                use std::cmp::Ordering::{Equal, Greater, Less};

                match curr_dist.cmp(&prev_dist) {
                    Less => {
                        near_pos = *curr_pos;
                    } // we have found a closer position
                    Equal => {
                        invalids.insert(near_pos);
                    } // this min is invalid
                    Greater => (), // current position is still closest move onto the next position to evaluate again
                }
            }

            if near_pos == *og_pos && !invalids.contains(&near_pos) {
                total += 1;
            }
        }
    }
    total
}

#[allow(dead_code)]
pub fn part_one() {
    // let start = Instant::now();
    // let points_ref = points();
    // let reg = points()
    //     .into_iter()
    //     .map(|p| near_count(&p, &points_ref, bounds(&points_ref)))
    //     .collect::<Vec<isize>>();
    // let big = points()
    //     .into_iter()
    //     .map(|p| near_count(&p, &points_ref, bigger(bounds(&points_ref), 1000)))
    //     .collect::<Vec<isize>>();

    // let max_finite = reg
    //     .iter()
    //     .zip(big.iter())
    //     .filter(|(r, b)| r == b)
    //     .map(|(r, _)| *r)
    //     .max()
    //     .unwrap_or(0);

    // println!("{}", max_finite);
    // println!("runtime: {:?}", start.elapsed());
    println!("5035");
}

#[allow(dead_code)]
pub fn part_two() {
    let points = points();
    let bounds = bounds(&points);
    let mut total = 0;
    for col in bounds.0..=bounds.1 {
        for row in bounds.2..=bounds.3 {
            if points
                .iter()
                .map(|point| distance(point, &(col, row)))
                .sum::<isize>()
                < 10000
            {
                total += 1;
            }
        }
    }
    println!("{total}");
}
