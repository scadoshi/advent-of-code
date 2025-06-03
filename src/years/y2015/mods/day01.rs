#[allow(dead_code)]
/// both parts
pub fn part_one() {
    let mut first_neg_i: Option<usize> = None;
    let floor_num = include_str!("../inputs/day01.txt")
        .chars()
        .enumerate()
        .fold(0, |mut floor_num, (i, dir)| {
            if dir == '(' {
                floor_num += 1
            } else {
                floor_num -= 1
            }
            if first_neg_i.is_none() && floor_num == -1 {
                first_neg_i = Some(i + 1);
            }
            floor_num
        });
    println!(
        "final floor_num is {:?}\nfirst negative index is {:?}",
        floor_num,
        first_neg_i.unwrap()
    );
}

pub fn part_two() {}
