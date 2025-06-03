use std::collections::HashMap;

#[allow(dead_code)]
fn input() -> Vec<String> {
    include_str!("../inputs/day02.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn part_one() {
    let mut num1 = 0;
    let mut num2 = 0;

    for box_id in input() {
        let letter_counts: HashMap<char, i32> =
            box_id
                .chars()
                .fold(HashMap::new(), |mut letter_count, letter| {
                    *letter_count.entry(letter).or_default() += 1;
                    letter_count
                });

        if letter_counts.iter().any(|(_, count)| count == &2_i32) {
            num1 += 1;
        }

        if letter_counts.iter().any(|(_, count)| count == &3_i32) {
            num2 += 1;
        }
    }

    println!("{}", num1 * num2);
}

#[allow(dead_code)]
pub fn part_two() {
    let input = input();
    let std_len = input[0].len();

    for (i, line1) in input.iter().enumerate() {
        for line2 in input.iter().skip(i) {
            let mut diff_is: Vec<usize> = Vec::new();

            for i in 0..std_len {
                if line1[i..i + 1] != line2[i..i + 1] {
                    diff_is.push(i);
                }
            }

            if diff_is.len() == 1 {
                println!(
                    "{}",
                    line1[..diff_is[0]].to_string() + &line1[diff_is[0] + 1..]
                )
            } else {
                continue;
            }
        }
    }
}
