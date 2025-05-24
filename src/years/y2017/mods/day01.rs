fn nums() -> Vec<u32> {
    include_str!("../inputs/day01.txt")
        .chars()
        .map(|num_str| {
            num_str
                .to_digit(10)
                .expect(format!("error converting {} to u32", num_str).as_str())
        })
        .collect()
}

pub fn part_one() {
    let nums: Vec<u32> = nums();

    let first = nums[0];
    let last = nums[nums.len() - 1];

    let total = nums
        .windows(2)
        .chain(std::iter::once([last, first].as_slice()))
        .fold(0, |mut total, window| {
            let num1 = window[0];
            let num2 = window[1];

            if num1 == num2 {
                total += num1
            }
            total
        });

    println!("{}", total);
}
pub fn part_two() {
    let mut total = 0;
    let nums = nums();

    let nums_len = nums.len();
    let nums_len_half = nums.len() / 2;

    for (i, num) in nums.iter().enumerate() {
        let num_to_compare = &nums[(i + nums_len_half) % nums_len];
        if num == num_to_compare {
            total += num;
        }
    }
    println!("{}", total);
}
