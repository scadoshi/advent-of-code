#[allow(dead_code)]
pub fn part_one() {
    println!(
        "{:?}",
        include_str!("../inputs/day02.txt")
            .lines()
            .fold([0, 0], |[mut horizontal, mut depth], line| {
                let distance = line
                    .split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                match line.split_whitespace().next().unwrap() {
                    "forward" => horizontal += distance,
                    "down" => depth += distance,
                    "up" => depth -= distance,
                    _ => (),
                }

                [horizontal, depth]
            })
            .iter()
            .map(|x| x.abs())
            .product::<i32>()
    );
}

#[allow(dead_code)]
pub fn part_two() {
    println!(
        "{:?}",
        include_str!("../inputs/day02.txt").lines().fold(
            [0, 0, 0],
            |[mut horizontal, mut depth, mut aim], line| {
                let distance = line
                    .split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                match line.split_whitespace().next().unwrap() {
                    "forward" => {
                        horizontal += distance;
                        if aim > 0 {
                            depth -= distance * aim
                        } else {
                            depth += distance * aim
                        }
                    }
                    "down" => aim -= distance,
                    "up" => aim += distance,
                    _ => (),
                }

                [horizontal, depth, aim]
            }
        )[..2]
            .iter()
            .map(|x| x.abs())
            .product::<i32>()
    );
}
