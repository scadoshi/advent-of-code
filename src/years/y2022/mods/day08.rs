use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

type Point = (usize, usize);
type Grid = Vec<Vec<usize>>;

trait Ops {
    fn input() -> Self;
    fn get(&self, point: Point) -> Option<usize>;
}
impl Ops for Grid {
    fn input() -> Self {
        include_str!("../inputs/day08.txt")
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).expect("failed to parse u32") as usize)
                    .collect()
            })
            .collect()
    }
    fn get(&self, point: Point) -> Option<usize> {
        let (r, c) = point;
        let max = self.len() - 1;
        if r > max || c > max {
            return None;
        }
        Some(self[r][c])
    }
}
trait IsEdge {
    fn is_edge(&self, grid: &Grid) -> bool;
}
impl IsEdge for Point {
    fn is_edge(&self, grid: &Grid) -> bool {
        let bounds = [0, grid.len() - 1];
        let (r, c) = self;
        if bounds.contains(&r) || bounds.contains(&c) {
            return true;
        }
        false
    }
}

#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();

    let grid = Grid::input();
    let mut points: VecDeque<Point> = (0..grid.len())
        .flat_map(|r| (0..grid.len()).map(|c| (r, c)).collect::<Vec<Point>>())
        .collect();
    let mut visible: HashSet<Point> = HashSet::new();

    while let Some(point) = points.pop_front() {
        if point.is_edge(&grid) {
            visible.insert(point);
            continue;
        }

        let (r, c) = point;
        if let Some(h) = grid.get(point) {
            if (c + 1..grid.len())
                .filter_map(|oc| grid.get((r, oc)))
                .all(|oh| h > oh)
                || (0..c).filter_map(|oc| grid.get((r, oc))).all(|oh| h > oh)
                || (r + 1..grid.len())
                    .filter_map(|or| grid.get((or, c)))
                    .all(|oh| h > oh)
                || (0..r).filter_map(|or| grid.get((or, c))).all(|oh| h > oh)
            {
                visible.insert(point);
            }
        }
    }

    println!("part_one()={:#?}", visible.len());
    println!("runtime={:.2?}", start.elapsed());
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();

    let grid = Grid::input();
    let mut points: VecDeque<Point> = (0..grid.len())
        .flat_map(|r| (0..grid.len()).map(|c| (r, c)).collect::<Vec<Point>>())
        .collect();
    let mut highest: usize = 0;

    while let Some(point) = points.pop_front() {
        if let Some(h) = grid.get(point) {
            let (r, c) = point;
            let rightward = (c + 1..grid.len())
                .filter_map(|oc| grid.get((r, oc)))
                .take_while_inclusive(|oh| h > *oh)
                .count();
            let leftward = (0..c)
                .rev()
                .filter_map(|oc| grid.get((r, oc)))
                .take_while_inclusive(|oh| h > *oh)
                .count();
            let downward = (r + 1..grid.len())
                .filter_map(|or| grid.get((or, c)))
                .take_while_inclusive(|oh| h > *oh)
                .count();
            let upward = (0..r)
                .rev()
                .filter_map(|or| grid.get((or, c)))
                .take_while_inclusive(|oh| h > *oh)
                .count();

            let score = rightward * leftward * downward * upward;
            highest = score.max(highest);
        }
    }

    println!("part_two()={:#?}", highest);
    println!("runtime={:.2?}", start.elapsed());
}
