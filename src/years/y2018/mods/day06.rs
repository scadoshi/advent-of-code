use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    col: u32,
    row: u32,
}

impl Point {
    fn new(col: u32, row: u32) -> Self {
        Self { col, row }
    }

    fn manhattan_distance(&self, other_point: &Point) -> u32 {
        ((self.col as i16 - other_point.col as i16).abs()
            + (self.row as i16 - other_point.row as i16).abs()) as u32
    }

    fn is_boundless(&self, empty_points: &HashSet<Point>, bounds: &Bounds) -> bool {
        // point itself is on boundary
        if [bounds.left, bounds.right].contains(&self.col)
                || [bounds.top, bounds.bottom].contains(&self.row)
            {
                return true;
            }
        // one of its empty points is on boundary
        for empty_point in empty_points {
            if [bounds.left, bounds.right].contains(&empty_point.col)
                || [bounds.top, bounds.bottom].contains(&empty_point.row)
            {
                return true;
            }
        }
        false
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.col, self.row)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Bounds {
    left: u32,
    right: u32,
    bottom: u32,
    top: u32,
}

impl Bounds {
    fn empty_points(&self, points: &HashSet<Point>) -> HashSet<Point> {
        let mut empty_points: HashSet<Point> = HashSet::new();
        for col in self.left..=self.right {
            for row in self.top..=self.bottom {
                let eval_point = Point::new(col, row);
                if !points.contains(&eval_point) {
                    empty_points.insert(eval_point);
                }
            }
        }
        empty_points
    }
}

#[derive(Debug)]
enum InputGetterError {
    NotEnoughComma,
    TooMuchComma,
    FailedToParseu32,
}

#[derive(Default)]
struct InputGetter {}
impl InputGetter {
    fn get_input(&self) -> Result<HashSet<Point>, InputGetterError> {
        include_str!("../inputs/day06.txt")
            .lines()
            .filter(|x| !x.starts_with("//"))
            .map(|x| {
                if let Some(parts) = x.split_once(",") {
                    if parts.1.contains(",") {
                        return Err(InputGetterError::TooMuchComma);
                    }

                    let col = parts
                        .0
                        .parse::<u32>()
                        .map_err(|_| InputGetterError::FailedToParseu32)?;
                    let row = parts
                        .1
                        .trim()
                        .parse::<u32>()
                        .map_err(|_| InputGetterError::FailedToParseu32)?;

                    Ok(Point::new(col, row))
                } else {
                    return Err(InputGetterError::NotEnoughComma);
                }
            })
            .collect::<Result<HashSet<Point>, InputGetterError>>()
    }
}

trait PointHandling {
    fn get_bounds(&self) -> Bounds;
    fn distance_map(&self, points: &HashSet<Point>) -> HashMap<Point, HashSet<Point>>;
}

impl PointHandling for HashSet<Point> {
    fn get_bounds(&self) -> Bounds {
        let mut left: Option<u32> = None;
        let mut right: Option<u32> = None;
        let mut top: Option<u32> = None;
        let mut bottom: Option<u32> = None;

        for point in self {
            if left.is_none() || point.col < left.unwrap() {
                left = Some(point.col);
            }
            if right.is_none() || point.col > right.unwrap() {
                right = Some(point.col);
            }
            if top.is_none() || point.row < top.unwrap() {
                top = Some(point.row);
            }
            if bottom.is_none() || point.row > bottom.unwrap() {
                bottom = Some(point.row);
            }
        }

        Bounds {
            left: left.unwrap(),
            right: right.unwrap(),
            top: top.unwrap(),
            bottom: bottom.unwrap(),
        }
    }

    fn distance_map(&self, points: &HashSet<Point>) -> HashMap<Point, HashSet<Point>> {
        // first process
        // filtering & finding
        let result: HashMap<Point, Point> = self
            .iter()
            .filter_map(|empty_point| {
                let mut lowest_distance: Option<u32> = None;
                // contains empty_point and the nearest_point
                let mut result: Option<(Point, Point)> = None;

                for current_point in points.iter() {
                    let current_distance = empty_point.manhattan_distance(current_point);

                    // if empty_point has multiple points that tie for nearest
                    // exclude from filter by returning None
                    if lowest_distance.is_some() && current_distance == lowest_distance.unwrap() {
                        return None;
                    }
                    // regular criteria for if the current_distance is less than lowest_distance
                    if lowest_distance.is_none() || current_distance < lowest_distance.unwrap() {
                        lowest_distance = Some(current_distance);
                        result = Some((empty_point.clone(), current_point.clone()));
                    }
                }
                result
            })
            .collect();

        // second process
        // grouping
        let result: HashMap<Point, HashSet<Point>> = result.into_iter().fold(
            HashMap::new(),
            |mut map: HashMap<Point, HashSet<Point>>, (empty_point, nearest_point)| {
                map.entry(nearest_point).or_default().insert(empty_point);
                map
            },
        );
        result
    }
}

#[allow(dead_code)]
pub fn part_one() {
    let ig = InputGetter::default();
    let points_res = ig.get_input();
    // println!("points result: {:?}", points_res);

    if let Some(points) = points_res.as_ref().ok() {
        let bounds = points.get_bounds();
        let empty_points = bounds.empty_points(&points);
        let regular_map = empty_points.distance_map(points);
        let filtered_map: HashMap<Point, HashSet<Point>> = regular_map
            .into_iter()
            .filter(|(point, empty_points)| !point.is_boundless(&empty_points, &bounds))
            .collect();
        let total_map: HashMap<Point, usize> = filtered_map
            .into_iter()
            .map(|(x, y)| (x, y.len() + 1))
            .collect();

        println!("{:#?}", total_map);
    } else {
        println!("points had an error: {:?}", points_res);
    }
}

pub fn part_two() {}
