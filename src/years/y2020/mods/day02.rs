#[allow(dead_code)]
pub fn part_one() {
    println!(
        "{:?}",
        include_str!("../inputs/day02.txt")
            .lines()
            .filter(|line| {
                let [parameters, string] = line
                    .split(':')
                    .map(|x| x.trim())
                    .collect::<Vec<&str>>()
                    .try_into()
                    .unwrap();
                let [min, max, char] = parameters
                    .split_whitespace()
                    .flat_map(|x| x.split('-').collect::<Vec<&str>>())
                    .collect::<Vec<&str>>()
                    .try_into()
                    .unwrap();

                let result = string
                    .chars()
                    .filter(|x| *x == char.chars().next().unwrap())
                    .count()
                    >= min.parse::<usize>().unwrap()
                    && string
                        .chars()
                        .filter(|x| *x == char.chars().next().unwrap())
                        .count()
                        <= max.parse::<usize>().unwrap();

                // println!("{line} -> {min}, {max}, {char}, {string}, {result}");

                result
            })
            .count()
    )
}

#[allow(dead_code)]
pub fn part_two() {
    println!(
        "{:?}",
        include_str!("../inputs/day02.txt")
            .lines()
            .filter(|line| {
                // get left and right sides
                let [parameters, string] = line
                    .split(':')
                    .map(|x| x.trim())
                    .collect::<Vec<&str>>()
                    .try_into()
                    .unwrap();
                // get left side components
                let [pos1_str, pos2_str, char_str] = parameters
                    .split_whitespace()
                    .flat_map(|x| x.split('-').collect::<Vec<&str>>())
                    .collect::<Vec<&str>>()
                    .try_into()
                    .unwrap();
                // convert positions to usize from &str
                // adjusts to base 0
                let [pos1, pos2] = [pos1_str, pos2_str]
                    .iter()
                    .map(|pos| pos.parse::<usize>().unwrap() - 1)
                    .collect::<Vec<usize>>()
                    .try_into()
                    .unwrap();
                // convert char to char from &str
                let char = char_str.chars().next().unwrap();
                // calculate result
                let result = (string.chars().nth(pos1) == Some(char)
                    || string.chars().nth(pos2) == Some(char))
                    && string.chars().nth(pos1) != string.chars().nth(pos2);
                // debug print
                println!("{line} -> {pos1}, {pos2}, {char}, {string}, {result}");

                result
            })
            .count()
    )
}
