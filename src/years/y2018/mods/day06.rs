use std::cmp::Ordering::{Equal, Greater, Less};
use std::rc::Rc;
use NearestPositionType as NPT;

#[derive(Debug)]
#[allow(dead_code)]
struct Grid {
    positions: Box<[Rc<Point>]>,
    empty_space: Box<[Point]>,
    bounds: Bounds,
}

trait FindBounds {
    fn find_bounds(&self) -> Bounds;
}
impl<T> FindBounds for Box<[T]>
where
    T: AsRef<Point>,
{
    fn find_bounds(&self) -> Bounds {
        let mut left: Option<usize> = None;
        let mut right: Option<usize> = None;
        let mut top: Option<usize> = None;
        let mut bottom: Option<usize> = None;

        for point in self.iter() {
            let (col, row) = (point.as_ref().col, point.as_ref().row);

            if left.is_none() || left.unwrap() > col {
                left = Some(col);
            }
            if right.is_none() || right.unwrap() < col {
                right = Some(col);
            }
            if top.is_none() || top.unwrap() > row {
                top = Some(row);
            }
            if bottom.is_none() || bottom.unwrap() < col {
                bottom = Some(row);
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
    Position(Rc<Point>),
    #[default]
    NotApplicable,
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

    fn find_nearest_position(&self, positions: &[Rc<Point>]) -> NPT {
        let mut nearest: NPT = NPT::NotApplicable;
        let mut first = true;

        for position in positions.iter() {
            let current_distance = position.distance_to(self);

            if first {
                nearest = NPT::Position(Rc::clone(position));
                first = false;
            } else {
                if let NPT::Position(prev_position) = &nearest {
                    let prev_distance = prev_position.distance_to(self);

                    match current_distance.cmp(&prev_distance) {
                        Less => {
                            nearest = NPT::Position(Rc::clone(position));
                        }
                        Equal => {
                            nearest = NPT::NotApplicable;
                            break;
                        }
                        Greater => (),
                    }
                }
            }
        }
        nearest
    }
}

impl AsRef<Point> for Point {
    fn as_ref(&self) -> &Point {
        self
    }
}

impl Grid {
    fn from_input() -> Self {
        // get positions from input text
        // only include col and row
        // for bounds finding
        let basic_positions: Box<[Point]> = include_str!("../inputs/day06.txt")
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

                // we don't care to fill out other fields at this point
                // as we just want a list of (col, row) to work with later
                Point {
                    col,
                    row,
                    ..Default::default()
                }
            })
            .collect::<Box<[Point]>>();

        // find bounds
        let bounds = basic_positions.find_bounds();

        // with bounds we can generate the list of
        // positions with all attributes (i.e. is_edging)
        let positions: Box<[Rc<Point>]> = basic_positions
            .iter()
            .map(|position| {
                Rc::new(Point {
                    col: position.col,
                    row: position.row,
                    point_type: PointType::Position,
                    nearest_position: NPT::NotApplicable,
                    is_edging: position.is_edging(&bounds),
                })
            })
            .collect();

        let empty_space: Box<[Point]> = (bounds.left..=bounds.right)
            .flat_map(|col| {
                (bounds.top..=bounds.bottom)
                    .filter_map(|row| {
                        if positions
                            .iter()
                            .any(|position| position.col == col && position.row == row)
                        {
                            return None;
                        }

                        let basic_point = Point {
                            col,
                            row,
                            ..Default::default()
                        };

                        Some(Point {
                            col,
                            row,
                            point_type: PointType::EmptySpace,
                            nearest_position: basic_point.find_nearest_position(&positions),
                            is_edging: basic_point.is_edging(&bounds),
                        })
                    })
                    .collect::<Box<[Point]>>()
            })
            .collect();

        Self {
            positions,
            empty_space,
            bounds,
        }
    }
}

#[allow(dead_code)]
pub fn part_one() {
    let grid = Grid::from_input();

    let most = grid.positions.iter().fold(0, |most: usize, position| {
        let current = grid
            .empty_space
            .iter()
            .filter(|p| p.nearest_position == NPT::Position(position.clone()))
            .count();

        if current > most {
            current
        } else {
            most
        }
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
    fn fnp_one() {
        let positions: Box<[Rc<Point>]> = Box::new([
            Rc::new(Point {
                col: 3,
                row: 4,
                ..Default::default()
            }),
            Rc::new(Point {
                col: 6,
                row: 7,
                ..Default::default()
            }),
        ]);

        let point = Point {
            col: 1,
            row: 4,
            ..Default::default()
        };

        let result = point.find_nearest_position(&positions);

        let expected = NPT::Position(Rc::new(Point {
            col: 3,
            row: 4,
            ..Default::default()
        }));

        assert_eq!(result, expected)
    }

    #[test]
    fn fnp_two() {
        let positions: Box<[Rc<Point>]> = Box::new([
            Rc::new(Point {
                col: 3,
                row: 4,
                ..Default::default()
            }),
            Rc::new(Point {
                col: 1,
                row: 6,
                ..Default::default()
            }),
            Rc::new(Point {
                col: 5,
                row: 5,
                ..Default::default()
            }),
        ]);

        let point = Point {
            col: 1,
            row: 4,
            ..Default::default()
        };

        let result = point.find_nearest_position(&positions);

        let expected = NPT::NotApplicable;

        assert_eq!(result, expected)
    }

    #[test]
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

    #[test]
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
