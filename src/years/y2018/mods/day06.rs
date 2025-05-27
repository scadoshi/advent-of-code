use std::cmp::Ordering::{Equal, Greater, Less};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Point {
    col: usize,
    row: usize,
}

#[derive(Debug, Default, PartialEq)]
struct Bounds {
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
}

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

    fn find_nearest_position<'a>(&self, positions: &'a Vec<Point>) -> Option<&'a Point> {
        let mut nearest = &positions[0];

        for position in positions.iter().skip(1) {
            let current_distance = position.distance_to(self);
            let prev_distance = nearest.distance_to(self);
            match current_distance.cmp(&prev_distance) {
                Less => nearest = position,
                Equal => return None,
                Greater => (),
            }
        }

        Some(nearest)
    }

    fn is_boundless(&self, positions: &Vec<Point>) -> bool {
        let bounds = positions.bounds();

        if self.is_edging(&bounds) {
            return true;
        }

        // make this look at every edge piece and see if its nearest is self
        for col in bounds.left..=bounds.right {
            for row in [bounds.top, bounds.bottom] {
                let point = Point { col, row };
                if point.find_nearest_position(positions) == Some(self) {
                    return true;
                }
            }
        }

        for row in bounds.top..=bounds.bottom {
            for col in [bounds.left, bounds.right] {
                let point = Point { col, row };
                if point.find_nearest_position(positions) == Some(self) {
                    return true;
                }
            }
        }

        false
    }
}

trait Points {
    fn from_input() -> Self;
    fn bounds(&self) -> Bounds;
}

impl Points for Vec<Point> {
    fn from_input() -> Self {
        let positions = include_str!("../inputs/day06.txt")
            .lines()
            .filter(|l| !l.starts_with("//"))
            .fold(Vec::new(), |mut positions, line| {
                let parts = line.split_once(",").expect("failed to split once on comma");
                let (col, row) = (
                    parts
                        .0
                        .trim()
                        .parse::<usize>()
                        .expect("failed to parse col into usize"),
                    parts
                        .1
                        .trim()
                        .parse::<usize>()
                        .expect("failed to parse row into usize"),
                );

                positions.push(Point { col, row });

                positions
            });

        positions
    }

    fn bounds(&self) -> Bounds {
        self.iter().fold(
            Bounds {
                left: self[0].col,
                right: self[0].col,
                top: self[0].row,
                bottom: self[0].row,
            },
            |bounds, point| Bounds {
                left: bounds.left.min(point.col),
                right: bounds.right.max(point.col),
                top: bounds.top.min(point.row),
                bottom: bounds.bottom.max(point.row),
            },
        )
    }
}

#[allow(dead_code)]
pub fn part_one() {
    let positions: Vec<Point> = Points::from_input();

    let most = positions
        .iter()
        .filter(|pos| !pos.is_boundless(&positions))
        .fold(0, |most, pos| {
            let current = positions
                .iter()
                .filter(|empty_space_point| {
                    if let Some(other_pos) = empty_space_point.find_nearest_position(&positions) {
                        pos == other_pos
                    } else {
                        false
                    }
                })
                .count();

            (current).max(most)
        });

    println!("{}", most);
}

#[allow(dead_code)]
pub fn part_two() {}

#[allow(dead_code)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn near_one() {
        let positions: Vec<Point> = vec![
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
        ];

        let point = Point {
            col: 1,
            row: 4,
            ..Default::default()
        };

        let result = point.find_nearest_position(&positions);

        let expected = Some(&Point {
            col: 3,
            row: 4,
            ..Default::default()
        });

        assert_eq!(result, expected)
    }

    #[test]
    fn near_two() {
        let positions: Vec<Point> = vec![
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
            Point {
                col: 5,
                row: 5,
                ..Default::default()
            },
        ];

        let point = Point {
            col: 1,
            row: 4,
            ..Default::default()
        };

        let result = point.find_nearest_position(&positions);

        let expected = None;

        assert_eq!(result, expected)
    }

    #[test]
    fn dist_simple() {
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

    #[test]
    fn dist_standard() {
        let one_point = Point {
            col: 0,
            row: 0,
            ..Default::default()
        };
        let another_point = Point {
            col: 4,
            row: 8,
            ..Default::default()
        };
        let result = one_point.distance_to(&another_point);
        let expected: usize = 12;
        assert_eq!(result, expected)
    }

    #[test]
    fn bdless_true() {
        let positions = vec![
            Point { col: 1, row: 1 },
            Point { col: 1, row: 6 },
            Point { col: 8, row: 3 },
            Point { col: 3, row: 4 },
            Point { col: 5, row: 5 },
            Point { col: 8, row: 9 },
        ];

        assert_eq!(positions[0].is_boundless(&positions), true)
    }

    #[test]
    fn bdless_false() {
        let positions = vec![
            Point { col: 1, row: 1 },
            Point { col: 1, row: 6 },
            Point { col: 8, row: 3 },
            Point { col: 3, row: 4 },
            Point { col: 5, row: 5 },
            Point { col: 8, row: 9 },
        ];

        assert_eq!(positions[3].is_boundless(&positions), false)
    }

    #[test]
    fn bds_input() {
        let positions = vec![
            Point { col: 1, row: 1 },
            Point { col: 1, row: 6 },
            Point { col: 8, row: 3 },
            Point { col: 3, row: 4 },
            Point { col: 5, row: 5 },
            Point { col: 8, row: 9 },
        ];

        let result = positions.bounds();
        let expected = Bounds {
            left: 1,
            right: 8,
            top: 1,
            bottom: 9,
        };
        assert_eq!(result, expected)
    }
}
