#[derive(Debug, Default)]
struct Race {
    time: usize,
    distance: usize,
}

impl From<(usize, usize)> for Race {
    fn from((time, distance): (usize, usize)) -> Self {
        Race { time, distance }
    }
}

fn races() -> Vec<Race> {
    let parts = include_str!("../inputs/day06.txt")
        .split_once("\n")
        .expect("failed to split once");
    let times: Vec<usize> = parts
        .0
        .split_once(":")
        .expect("failed to split once")
        .1
        .split_whitespace()
        .map(|x| x.parse::<usize>().expect("failed to parse to usize"))
        .collect();
    let distances: Vec<usize> = parts
        .1
        .split_once(":")
        .expect("failed to split once")
        .1
        .split_whitespace()
        .map(|x| x.parse::<usize>().expect("failed to parse to usize"))
        .collect();
    times.into_iter().zip(distances).map(Into::into).collect()
}

fn race() -> Race {
    let parts = include_str!("../inputs/day06.txt")
        .split_once("\n")
        .expect("failed to split once");
    let time = parts
        .0
        .split_once(":")
        .expect("failed to split once")
        .1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<usize>()
        .expect("failed to parse to usize");

    let distance = parts
        .1
        .split_once(":")
        .expect("failed to split once")
        .1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<usize>()
        .expect("failed to parse to usize");
    (time, distance).into()
}

impl Race {
    fn wins(&self) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();

        for push_time in 1..=(self.time - 1) {
            if push_time * (self.time - push_time) > self.distance {
                result.push(push_time);
            }
        }

        result
    }
}

#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();
    println!(
        "part_one={:?} ... runtime={:?}",
        {
            races()
                .into_iter()
                .map(|r| r.wins().len())
                .product::<usize>()
        },
        start.elapsed()
    );
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();
    println!(
        "part_two={:?} ... runtime={:?}",
        race().wins().len(),
        start.elapsed()
    );
}
