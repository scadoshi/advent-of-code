#[allow(dead_code)]
fn input() -> Vec<i32> {
    include_str!("../inputs/day01.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn part_one() {
    let nums = input();
    let target_sum = 2020;
    for (i, num1) in nums.iter().enumerate() {
        if nums
            .iter()
            .enumerate()
            .any(|(j, num2)| target_sum - num1 == *num2 && i != j)
        {
            println!("{}", num1 * (target_sum - num1));
            break;
        }
    }
}

#[allow(dead_code)]
pub fn part_two() {
    let nums = input();

    let target = 2020;
    'main: for num1 in nums.iter() {
        let sub_target = target - num1;
        for num2 in nums.iter() {
            let sub_sub_target = sub_target - num2;
            if nums.contains(&sub_sub_target) {
                println!("{}", sub_sub_target * num1 * num2);
                break 'main;
            }
        }
    }
}
