#[derive(Debug)]
struct Grid {
    _points: Box<[Point]>,
    _bounds: Bounds,
}

trait FindBounds {
    fn find_bounds(&self) -> Bounds;
}
impl FindBounds for Box<[Point]> {
    fn find_bounds(&self) -> Bounds {
        let mut left: Option<usize> = None;
        let mut right: Option<usize> = None;
        let mut top: Option<usize> = None;
        let mut bottom: Option<usize> = None;

        for point in self.iter() {
            if left.is_none() || left.unwrap() > point.col {
                left = Some(point.col);
            }
            if right.is_none() || right.unwrap() < point.col {
                right = Some(point.col);
            }
            if top.is_none() || top.unwrap() > point.row {
                top = Some(point.row);
            }
            if bottom.is_none() || bottom.unwrap() < point.col {
                bottom = Some(point.row);
            }
        }

        let [left, right, top, bottom] = [left, right, top, bottom]
            .into_iter()
            .map(|option| option.expect("failed to find bound"))
            .collect::<Vec<usize>>()
            .try_into()
            .expect("failed to convert to array");

        Bounds {
            left,
            right,
            top,
            bottom,
        }
    }
}

#[derive(Debug, Default)]
struct Bounds {
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Point {
    col: usize,
    row: usize,
    point_type: PointType,
    is_edging: bool,
    nearest_position: NPT,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
enum PointType {
    Position,
    #[default]
    EmptySpace,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
enum NearestPositionType {
    Position(Box<Point>),
    #[default]
    NotApplicable,
}

use NearestPositionType as NPT;

impl Point {
    #[allow(dead_code)]
    fn distance_to(&self, other_point: &Point) -> usize {
        (self.col as isize - other_point.col as isize).abs() as usize
            + (self.row as isize - other_point.row as isize).abs() as usize
    }

    fn is_edging(&self, bounds: &Bounds) -> bool {
        [bounds.left, bounds.right].contains(&self.col)
            || [bounds.top, bounds.bottom].contains(&self.row)
    }

    fn find_nearest_position(&self, positions: &[Point], bounds: &Bounds) -> NPT {
        positions
            .iter()
            .fold(NPT::NotApplicable, |nearest, position| {
                let current_distance = position.distance_to(self);

                match &nearest {
                    NPT::Position(prev_position) => {
                        let prev_distance = prev_position.distance_to(self);

                        use std::cmp::Ordering::{Equal, Greater, Less};

                        match current_distance.cmp(&prev_distance) {
                            Less => NPT::Position(Box::new(position.clone())),
                            Equal => NPT::NotApplicable,
                            Greater => nearest,
                        }
                    }
                    NPT::NotApplicable => NPT::Position(Box::new(Point {
                        col: position.col,
                        row: position.row,
                        point_type: PointType::Position,
                        nearest_position: NPT::NotApplicable,
                        is_edging: position.is_edging(&bounds),
                    })),
                }
            })
    }
}

impl Grid {
    fn from_input() -> Self {
        // get positions from input text
        let positions: Box<[Point]> = include_str!("../inputs/day06.txt")
            .lines()
            .filter(|l| !l.starts_with("//"))
            .map(|l| {
                let parts = l.split_once(",").expect("failed to split once on comma");

                let col = parts
                    .0
                    .trim()
                    .parse::<usize>()
                    .expect("failed to parse col to usize");

                let row = parts
                    .1
                    .trim()
                    .parse::<usize>()
                    .expect("failed to parse row to usize");

                Point {
                    col,
                    row,
                    ..Default::default()
                }
            })
            .collect::<Box<[Point]>>();

        // calculate bounds from input positions
        let bounds = positions.find_bounds();

        // generate all points and accompanying attributes
        let all_points: Box<[Point]> = (bounds.left..=bounds.right)
            .into_iter()
            .flat_map(|col| {
                (bounds.top..=bounds.bottom)
                    .into_iter()
                    .map(|row| {
                        let current_point = Point {
                            col,
                            row,
                            ..Default::default()
                        };

                        let is_edging = current_point.is_edging(&bounds);

                        let point_type = match positions.contains(&current_point) {
                            true => PointType::Position,
                            false => PointType::EmptySpace,
                        };

                        let nearest_position = match point_type {
                            PointType::Position => NPT::NotApplicable,
                            PointType::EmptySpace => {
                                current_point.find_nearest_position(&positions, &bounds)
                            }
                        };

                        Point {
                            col,
                            row,
                            point_type,
                            is_edging,
                            nearest_position,
                        }
                    })
                    .collect::<Vec<Point>>()
            })
            .collect();

        Self {
            _points: all_points,
            _bounds: bounds,
        }
    }
}

#[allow(dead_code)]
pub fn part_one() {
    let grid = Grid::from_input();
    println!("{:#?}", grid);
}

#[allow(dead_code)]
pub fn part_two() {}

#[allow(dead_code)]
#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    fn fnp_one() {
        let positions: Box<[Point]> = Box::new([
            Point {
                col: 3,
                row: 4,
                ..Default::default()
            },
            Point {
                col: 6,
                row: 7,
                ..Default::default()
            },
        ]);

        let bounds = positions.find_bounds();

        let point = Point {
            col: 1,
            row: 4,
            ..Default::default()
        };

        let result = point.find_nearest_position(&positions, &bounds);

        let expected = NPT::Position(Box::new(Point {
            col: 3,
            row: 4,
            ..Default::default()
        }));

        assert_eq!(result, expected)
    }

    #[ignore]
    fn fnp_two() {
        let positions: Box<[Point]> = Box::new([
            Point {
                col: 3,
                row: 4,
                ..Default::default()
            },
            Point {
                col: 1,
                row: 6,
                ..Default::default()
            },
        ]);

        let bounds = positions.find_bounds();

        let point = Point {
            col: 1,
            row: 4,
            ..Default::default()
        };

        let result = point.find_nearest_position(&positions, &bounds);

        let expected = NPT::NotApplicable;

        assert_eq!(result, expected)
    }

    #[ignore]
    fn dt_simple() {
        let one_point = Point {
            col: 1,
            row: 4,
            ..Default::default()
        };
        let another_point = Point {
            col: 1,
            row: 6,
            ..Default::default()
        };
        let result = one_point.distance_to(&another_point);
        let expected: usize = 2;
        assert_eq!(result, expected)
    }

    #[ignore]
    fn dt_standard() {
        let one_point = Point {
            col: 0,
            row: 0,
            ..Default::default()
        };
        let another_point = Point {
            col: 8,
            row: 8,
            ..Default::default()
        };
        let result = one_point.distance_to(&another_point);
        let expected: usize = 16;
        assert_eq!(result, expected)
    }
}
