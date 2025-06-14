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
pub fn part_two() {
    let start = std::time::Instant::now();
    let input = input();
    println!(
        "part_two={}",
        (*input.iter().min().expect("min not found")..=*input.iter().max().expect("max not found"))
            .fold(i32::MAX, |amount, point| {
                amount.min(
                    input
                        .iter()
                        // this gets the diff of the testing point
                        // then calcs the triangular number of the limit
                        // i.e. if limit is 4, then do 1 + 2 + 3 + 4 = 10
                        // then sums every crabs result (fuel usage)
                        .map(|x| (0..=(x - point).abs()).sum::<i32>())
                        .sum(),
                )
            })
    );
    println!("runtime={:?}", start.elapsed());
}
