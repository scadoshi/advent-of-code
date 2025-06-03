use std::collections::VecDeque;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;

fn seeds() -> Vec<usize> {
    include_str!("../inputs/day05.txt")
        .lines()
        .next()
        .expect("Cannot find first line")
        .split_once(":")
        .expect("Failed to split_once by a colon (':')")
        .1
        .split_whitespace()
        .map(|num_str| num_str.parse().expect("Error converting to usize"))
        .collect()
}

fn seed_ranges() -> Vec<Range> {
    include_str!("../inputs/day05.txt")
        .lines()
        .next()
        .expect("Cannot find first line")
        .split_once(":")
        .expect("Failed to split_once by a colon (':')")
        .1
        .split_whitespace()
        .map(|x| x.parse::<usize>().expect("Failed to parse to usize"))
        .collect::<Vec<usize>>()
        .chunks(2)
        .map(|chunk| Range::from_parts(chunk[0], chunk[0] + chunk[1] - 1))
        .collect()
}

fn all_range_map_lists() -> Vec<Vec<RangeMap>> {
    let maps: Vec<Vec<RangeMap>> = include_str!("..//inputs//day05.txt")
        .split("\r\n\r\n")
        .skip(1)
        .map(|x| {
            x.split_once(":")
                .expect("Failed to split_once by a colon (':')")
                .1
                .split("\r\n")
                .skip(1)
                .map(|rm_str| {
                    let parts = rm_str
                        .split_whitespace()
                        .map(|num_str| {
                            num_str
                                .parse::<usize>()
                                .expect("Failed to convert to usize")
                        })
                        .collect::<Vec<usize>>();
                    RangeMap::from_parts(
                        Range::from_parts(parts[1], parts[1] + parts[2] - 1),
                        Range::from_parts(parts[0], parts[0] + parts[2] - 1),
                    )
                })
                .collect()
        })
        .collect();
    maps
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Range {
    first: usize,
    last: usize,
}

struct RangeMap {
    source: Range,
    destination: Range,
}

impl Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.first, self.last)
    }
}

impl Debug for RangeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?} -> {:?})", self.source, self.destination)
    }
}

impl Range {
    fn from_parts(first: usize, last: usize) -> Self {
        Self { first, last }
    }

    fn contains(&self, value: &usize) -> bool {
        *value >= self.first && *value <= self.last
    }

    fn split_map(&self, rmap: &RangeMap) -> Vec<Range> {
        let mut result: Vec<Range> = Vec::new();

        // no overlap
        if self.last < rmap.source.first || self.first > rmap.source.last {
            return vec![self.clone()];
        }

        // left
        if self.first < rmap.source.first {
            result.push(Range::from_parts(self.first, rmap.source.first - 1));
        }

        // middle
        result.push(Range::from_parts(
            self.first.max(rmap.source.first).range_map(&rmap),
            self.last.min(rmap.source.last).range_map(&rmap),
        ));

        // right
        if self.last > rmap.source.last {
            result.push(Range::from_parts(rmap.source.last + 1, self.last));
        }

        result
    }

    fn to_location_ranges(&self, arml: &Vec<Vec<RangeMap>>, log: &mut File) -> Vec<Range> {
        let mut result: Vec<Range> = Vec::new();
        let mut queue: VecDeque<(Range, usize)> = VecDeque::from(vec![(self.clone(), 0)]);

        while let Some((range_to_check, group)) = queue.pop_front() {
            if group == arml.len() {
                result.push(range_to_check);
                continue;
            }

            if let Some(matching_rmap) = arml[group].iter().find(|rmap| {
                !(range_to_check.last < rmap.source.first
                    || range_to_check.first > rmap.source.last)
            }) {
                for range in range_to_check.split_map(matching_rmap) {
                    if range.first == 0 {
                        writeln!(log, "{:?}", range).expect("Failed to write to log");
                    }
                    queue.push_back((range, group + 1));
                }
            } else {
                queue.push_back((range_to_check, group + 1));
            }
        }

        result
    }
}

impl RangeMap {
    fn from_parts(source: Range, destination: Range) -> Self {
        Self {
            source,
            destination,
        }
    }
}

trait SeedMapping {
    fn to_location(&self, arml: &Vec<Vec<RangeMap>>) -> Option<usize>;
    fn range_map(&self, rmap: &RangeMap) -> usize;
}

impl SeedMapping for usize {
    fn to_location(&self, arml: &Vec<Vec<RangeMap>>) -> Option<usize> {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::from(vec![(self.clone(), 0)]);

        while let Some((value, group)) = queue.pop_front() {
            if group == arml.len() {
                return Some(value);
            }

            if let Some(matching_rmap) =
                arml[group].iter().find(|rmap| rmap.source.contains(&value))
            {
                let mapped = value.range_map(matching_rmap);
                queue.push_front((mapped, group + 1));
            } else {
                queue.push_front((value, group + 1));
            }
        }
        None
    }
    fn range_map(&self, rmap: &RangeMap) -> usize {
        if rmap.source.contains(self) {
            let dif = rmap.destination.first as isize - rmap.source.first as isize;
            return (*self as isize + dif) as usize;
        }
        *self
    }
}

#[allow(dead_code)]
pub fn part_one() {
    let mut lowest_location: Option<usize> = None;
    let arml = all_range_map_lists();
    for seed in seeds() {
        if let Some(new_location) = seed.to_location(&arml) {
            if lowest_location.is_none() || new_location < lowest_location.unwrap() {
                lowest_location = Some(new_location);
            }
        }
    }
    println!("{}", lowest_location.expect("No locations found"));
}

#[allow(dead_code)]
pub fn part_two() {
    let mut log =
        File::create("src\\years\\y2023\\mods\\day05_log.txt").expect("Failed to create log");
    let arml = all_range_map_lists();
    let mut lowest_location: Option<usize> = None;
    let seed_ranges = seed_ranges();

    for seed_range in seed_ranges {
        let location_ranges = seed_range.to_location_ranges(&arml, &mut log);

        for range in location_ranges {
            if lowest_location.is_none()
                || (range.first < lowest_location.unwrap() && range.first != 0)
            {
                lowest_location = Some(range.first);
            }
        }
    }
    println!("{}", lowest_location.expect("No locations found"));
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    #[ignore]
    fn split_map_test() {
        let range = Range::from_parts(10, 20);
        let range_map = RangeMap::from_parts(Range::from_parts(5, 15), Range::from_parts(105, 115));

        let result: Vec<Range> = range.split_map(&range_map);
        let expected: Vec<Range> = vec![Range::from_parts(110, 115), Range::from_parts(16, 20)];
        assert_eq!(result, expected);
    }
}
