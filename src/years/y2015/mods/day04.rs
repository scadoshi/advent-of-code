use md5;

#[allow(dead_code)]
pub fn part_one() {
    let input: &str = include_str!("../inputs/day04.txt");
    let mut appended = 0;

    while format!(
        "{:x}",
        md5::compute((input.to_string().clone() + appended.to_string().as_ref()).as_bytes())
    )
    .chars()
    .enumerate()
    .filter(|(i, char)| *i < 5 as usize && *char == '0')
    .count()
        < 5
    {
        // println!("{:?}", md5::compute((input.clone() + appended.to_string().as_ref()).as_bytes()));
        appended += 1;
    }

    println!("{appended}");
    println!(
        "{:?}",
        md5::compute((input.to_string().clone() + appended.to_string().as_ref()).as_bytes())
    )
}

#[allow(dead_code)]
pub fn part_two() {
    let input: String =
        std::fs::read_to_string("src/day_modules/input/four.txt").expect("error reading file");
    let mut appended = 0;

    while format!(
        "{:x}",
        md5::compute((input.clone() + appended.to_string().as_ref()).as_bytes())
    )
    .chars()
    .enumerate()
    .filter(|(i, char)| *i < 6 as usize && *char == '0')
    .count()
        < 6
    {
        // println!("{:?}", md5::compute((input.clone() + appended.to_string().as_ref()).as_bytes()));
        appended += 1;
    }

    println!("{appended}");
    println!(
        "{:?}",
        md5::compute((input + appended.to_string().as_ref()).as_bytes())
    )
}
