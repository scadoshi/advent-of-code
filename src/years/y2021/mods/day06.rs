use std::collections::HashMap;

fn fish_map() -> HashMap<usize, usize> {
    include_str!("../inputs/day06.txt")
        .split(",")
        .fold(HashMap::new(), |mut fish_map, fish| {
            *fish_map
                .entry(fish.parse::<usize>().expect("failed to parse to usize"))
                .or_default() += 1;
            fish_map
        })
}

fn fish_spawning(starting_fish: HashMap<usize, usize>, for_days: usize) -> HashMap<usize, usize> {
    (0..for_days).fold(starting_fish, |map, _| {
        let mut new_map: HashMap<usize, usize> = HashMap::new();
        for (k, v) in map {
            if let Some(next_k) = k.checked_sub(1) {
                *new_map.entry(next_k).or_default() += v;
            } else {
                *new_map.entry(6).or_default() += v;
                *new_map.entry(8).or_default() += v;
            }
        }
        new_map
    })
}

#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();
    println!(
        "part_one={:?} ... runtime={:?}",
        fish_spawning(fish_map(), 80).values().sum::<usize>(),
        start.elapsed()
    );
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();
    println!(
        "part_one={:?} ... runtime={:?}",
        fish_spawning(fish_map(), 256).values().sum::<usize>(),
        start.elapsed()
    );
}

#[cfg(test)]
mod tests {}
