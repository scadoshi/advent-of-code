fn input() -> Box<[i32]> {
    include_str!("../inputs/day07.txt")
        .split(',')
        .map(|x| x.parse::<i32>().expect("failed to parse to i32"))
        .collect::<Box<[i32]>>()
}

#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();
    let input = input();
    println!(
        "part_one={}",
        (*input.iter().min().expect("min not found")..=*input.iter().max().expect("max not found"))
            .fold(i32::MAX, |amount, point| {
                amount.min(input.iter().map(|x| (x - point).abs()).sum())
            })
    );
    println!("runtime={:?}", start.elapsed());
}

#[allow(dead_code)]
pub fn part_two() {}
