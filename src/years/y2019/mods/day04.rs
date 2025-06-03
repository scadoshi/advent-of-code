#[allow(dead_code)]
pub fn part_one() {
    let range: Vec<i32> = include_str!("../inputs/day04.txt")
        .split('-')
        .map(|x| x.parse().unwrap())
        .collect();

    let min = range[0];
    let max = range[1];

    let mut total = 0;
    'main: for password in min..=max {
        let mut double_digits: bool = false;
        for chars in password
            .to_string()
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
        {
            if chars[0] == chars[1] {
                double_digits = true;
            }
            if chars[0] > chars[1] {
                continue 'main;
            }
        }
        if !double_digits {
            continue 'main;
        }
        total += 1;
    }

    println!("{}", total);
}

#[allow(dead_code)]
trait StrExt {
    fn groups(&self) -> impl Iterator<Item = Vec<char>>;
}

impl StrExt for String {
    #[allow(dead_code)]
    fn groups(&self) -> impl Iterator<Item = Vec<char>> {
        let mut groups: Vec<Vec<char>> = Vec::new();
        let mut group: Vec<char> = Vec::new();
        for char in self.chars() {
            if !group.is_empty() && char != group[0] {
                groups.push(group.clone());
                group.clear();
            }
            group.push(char);
        }
        groups.push(group.clone());
        group.clear();
        groups.into_iter()
    }
}

#[allow(dead_code)]
pub fn part_two() {
    let range: Vec<i32> = include_str!("../inputs/day04.txt")
        .split('-')
        .map(|x| x.parse().unwrap())
        .collect();

    let min = range[0];
    let max = range[1];
    let mut total = 0;
    'main: for password in min..=max {
        for chars in password
            .to_string()
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
        {
            if chars[0] > chars[1] {
                continue 'main;
            }
        }
        if !password.to_string().groups().any(|x| x.len() == 2) {
            continue 'main;
        }
        total += 1;
    }
    println!("{}", total);
}
