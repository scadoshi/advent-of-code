#[allow(dead_code)]
pub fn part_one() {
    println!(
        "{}",
        include_str!("../inputs/day04.txt")
            .lines()
            .fold(0, |mut total, line| {
                let ranges = line
                    .split(',')
                    .map(|x| {
                        x.split('-')
                            .map(|x| x.parse::<u32>().unwrap())
                            .collect::<Vec<u32>>()
                    })
                    .collect::<Vec<Vec<u32>>>();

                let left_min = ranges[0][0];
                let left_max = ranges[0][1];
                let right_min = ranges[1][0];
                let right_max = ranges[1][1];

                if (right_min <= left_min && left_max <= right_max)
                    || (left_min <= right_min && right_max <= left_max)
                {
                    total += 1;
                }
                total
            })
    )
}

#[allow(dead_code)]
pub fn part_two() {
    println!(
        "{}",
        include_str!("../inputs/day04.txt")
            .lines()
            .fold(0, |mut total, line| {
                let ranges = line
                    .split(',')
                    .map(|x| {
                        x.split('-')
                            .map(|x| x.parse::<u32>().unwrap())
                            .collect::<Vec<u32>>()
                    })
                    .collect::<Vec<Vec<u32>>>();

                let left_min = ranges[0][0];
                let left_max = ranges[0][1];
                let right_min = ranges[1][0];
                let right_max = ranges[1][1];

                if (right_min <= left_min && left_min <= right_max)
                    || (right_min <= left_max && left_max <= right_max)
                    || (left_min <= right_min && right_min <= left_max)
                    || (left_min <= right_max && right_max <= left_max)
                {
                    total += 1;
                }
                total
            })
    )
}
