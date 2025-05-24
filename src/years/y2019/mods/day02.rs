#[allow(dead_code)]
fn input() -> Vec<i32> {
    include_str!("../inputs/day02.txt")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn weird(nums: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut new_nums = nums.clone();
    new_nums[1] = noun;
    new_nums[2] = verb;
    for i in (0..new_nums.len()).step_by(4) {
        let (num1, num2) = (
            new_nums[new_nums[i + 1] as usize],
            new_nums[new_nums[i + 2] as usize],
        );
        let pos = new_nums[i + 3] as usize;

        match new_nums[i] {
            1 => new_nums[pos] = num1 + num2,
            2 => new_nums[pos] = num1 * num2,
            3 => (),
            4 => (),
            99 => break,
            _ => panic!("Invalid Opcode: {}", new_nums[i]),
        }
    }
    new_nums[0]
}

#[allow(dead_code)]
pub fn part_one() {
    let nums = input();
    println!("{}", weird(&nums, 12, 2));
}

#[allow(dead_code)]
pub fn part_two() {
    let nums = input();
    for i in 0..=99 {
        for j in 0..=99 {
            if weird(&nums, i, j) == 19690720 {
                println!("{}", 100 * i + j);
            }
        }
    }
}
