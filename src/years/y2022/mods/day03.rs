#[allow(dead_code)]
fn input() -> Vec<String> {
    include_str!("../inputs/day03.txt")
        .lines()
        .map(|x| x.to_string())
        .collect()
}

#[allow(dead_code)]
fn priority(char: char) -> u8 {
    let mut priority = 0;
    if char.is_uppercase() {
        priority += 26;
    }
    priority + (char.to_lowercase().next().unwrap() as u8 - 'a' as u8 + 1)
}

#[allow(dead_code)]
pub fn part_one() {
    println!(
        "{}",
        input().iter().fold(0, |total: u32, line| {
            let (left, right) = (&line[..line.len() / 2], &line[line.len() / 2..]);
            let common = left
                .chars()
                .find(|char| right.contains(*char))
                .expect("common char not found");
            total + priority(common) as u32
        })
    )
}

#[allow(dead_code)]
pub fn part_two() {
    println!(
        "{}",
        input().chunks(3).fold(0, |total, group| {
            let common = group[0]
                .chars()
                .find(|char| group[1].contains(*char) && group[2].contains(*char))
                .expect("common char not found");
            total + priority(common) as u32
        })
    )
}
