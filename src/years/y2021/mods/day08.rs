#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();
    let result: usize = include_str!("../inputs/day08.txt")
        .lines()
        .map(|x| {
            x.split_once("|")
                .expect("failed to split once")
                .1
                .trim()
                .split_whitespace()
                .filter(|x| [2, 3, 4, 7].contains(&x.len()))
                .count()
        })
        .sum();
    println!("part_one()={:#?}", result);
    println!("runtime={:#?}", start.elapsed());
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();
    println!("part_two()={:#?}", 0);
    println!("runtime={:#?}", start.elapsed());
}
