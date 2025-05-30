use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashMap,
    fmt::Debug,
    fs::File,
    io::Write,
    path::Path,
};

#[derive(Default, PartialEq, Eq, Hash, Clone)]
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

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.col, self.row)
    }
}

impl Point {
    fn distance_to(&self, other_point: &Point) -> usize {
        (self.col as isize - other_point.col as isize).abs() as usize
            + (self.row as isize - other_point.row as isize).abs() as usize
    }

    fn is_edging(&self, bounds: &Bounds) -> bool {
        [bounds.left, bounds.right].contains(&self.col)
            || [bounds.top, bounds.bottom].contains(&self.row)
    }

    fn find_nearest_position<'a>(
        &self,
        positions: &'a Vec<Point>,
        log: &mut File,
    ) -> Option<&'a Point> {
        // <debug>
        writeln!(log, "\n# finding nearest position for {:?}", self).expect("failed to write to log");
        // </debug>

        let mut nearest = &positions[0];

        for position in positions.iter().skip(1) {
            let current_distance = position.distance_to(self);
            let prev_distance = nearest.distance_to(self);

            // <debug>
            writeln!(log, 
                "## checking position {:?}\n - previous distance = {:?}\n - current_distance = {:?}",
                position, prev_distance, current_distance
            ).expect("failed to write to log");
            //</debug>

            match current_distance.cmp(&prev_distance) {
                Less => nearest = position,
                Equal => return None,
                Greater => (),
            }
        }

        writeln!(log, "## found nearest position to be {:?}", nearest).expect("failed to write to log");

        Some(nearest)
    }

    fn is_boundless(&self, positions: &Vec<Point>, log: &mut File) -> bool {
        let bounds = positions.bounds();

        if self.is_edging(&bounds) {
            return true;
        }

        // make this look at every edge piece and see if its nearest is self
        for col in bounds.left..=bounds.right {
            for row in [bounds.top, bounds.bottom] {
                let point = Point { col, row };
                if point.find_nearest_position(positions, log) == Some(self) {
                    return true;
                }
            }
        }

        for row in bounds.top..=bounds.bottom {
            for col in [bounds.left, bounds.right] {
                let point = Point { col, row };
                if point.find_nearest_position(positions, log) == Some(self) {
                    return true;
                }
            }
        }

        false
    }

    fn empty_space(&self, positions: &Vec<Point>, log: &mut File) -> Option<usize> {
        if self.is_boundless(positions, log) {
            return None;
        }

        let bounds = positions.bounds();
        let mut total = 1;

        for col in bounds.left..=bounds.right {
            for row in bounds.top..=bounds.bottom {
                let point = Point { col, row };

                if positions.contains(&point) {
                    continue;
                }

                if point.find_nearest_position(positions, log) == Some(self) {
                    total += 1;
                    // <debug>
                    {
                        let debug_target = Point { col: 3, row: 4 };
                        if *self == debug_target {
                            writeln!(
                                log,
                                "empty space found {:?} for target {:?}",
                                point, debug_target
                            ).expect("failed to write to log");
                        }
                    }
                    // </debug>
                }
            }
        }

        Some(total)
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
    // <debug>
    if !Path::new("../logs").exists() {
        std::fs::create_dir_all("src/years/y2018/logs").expect("failed to create logs file");
    }
    let mut log = File::create("src/years/y2018/logs/day06.log").expect("failed to create log");
    // </debug>
    let positions: Vec<Point> = Points::from_input();

    let points_counts_map: HashMap<Point, usize> = positions
        .clone()
        .into_iter()
        .filter_map(|point| {
            point
                .empty_space(&positions, &mut log)
                .map(|count| (point, count))
        })
        .collect();

    writeln!(
        log,
        "\n***\n# showing all valid points with their empty space point counts\n{:?}",
        points_counts_map
    ).expect("failed to write to log");
}

#[allow(dead_code)]
pub fn part_two() {}

// #[allow(dead_code)]
// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn near_point_one() {
//         let positions: Vec<Point> = vec![Point { col: 3, row: 4 }, Point { col: 6, row: 7 }];
//         let point = Point { col: 1, row: 4 };
//         let result = point.find_nearest_position(&positions);
//         let expected = Some(&Point { col: 3, row: 4 });
//         assert_eq!(result, expected)
//     }

//     #[test]
//     fn near_point_two() {
//         let positions: Vec<Point> = vec![
//             Point { col: 3, row: 4 },
//             Point { col: 1, row: 6 },
//             Point { col: 5, row: 5 },
//         ];
//         let point = Point { col: 1, row: 4 };
//         let result = point.find_nearest_position(&positions);
//         let expected = None;
//         assert_eq!(result, expected)
//     }

//     #[test]
//     fn distance_to_simple() {
//         let one_point = Point { col: 1, row: 4 };
//         let another_point = Point { col: 1, row: 6 };
//         let result = one_point.distance_to(&another_point);
//         let expected: usize = 2;
//         assert_eq!(result, expected)
//     }

//     #[test]
//     fn distance_to_standard() {
//         let one_point = Point { col: 0, row: 0 };
//         let another_point = Point { col: 4, row: 8 };
//         let result = one_point.distance_to(&another_point);
//         let expected: usize = 12;
//         assert_eq!(result, expected)
//     }

//     fn test_input() -> Vec<Point> {
//         vec![
//             Point { col: 1, row: 1 },
//             Point { col: 1, row: 6 },
//             Point { col: 8, row: 3 },
//             Point { col: 3, row: 4 },
//             Point { col: 5, row: 5 },
//             Point { col: 8, row: 9 },
//         ]
//     }

//     #[test]
//     fn is_boundless_true() {
//         let positions = test_input();
//         assert_eq!(positions[0].is_boundless(&positions), true)
//     }

//     #[test]
//     fn is_boundless_false() {
//         let positions = test_input();
//         assert_eq!(positions[3].is_boundless(&positions), false)
//     }

//     #[test]
//     fn bds_input() {
//         let positions = test_input();
//         let result = positions.bounds();
//         let expected = Bounds {
//             left: 1,
//             right: 8,
//             top: 1,
//             bottom: 9,
//         };
//         assert_eq!(result, expected)
//     }

//     #[test]
//     fn edge_true() {
//         let positions = test_input();
//         let result = positions[2].is_edging(&positions.bounds());
//         assert_eq!(result, true)
//     }

//     #[test]
//     fn edge_false() {
//         let positions = test_input();
//         let result = positions[4].is_edging(&positions.bounds());
//         assert_eq!(result, false)
//     }
// }
