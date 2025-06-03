use md5;

fn append_and_hash(input: &str, appended: i32) -> String {
    format!(
        "{:x}",
        md5::compute((input.to_string().clone() + appended.to_string().as_ref()).as_bytes())
    )
}

#[allow(dead_code)]
pub fn part_one() {
    let input: &str = include_str!("../inputs/day05.txt");
    let mut appended = 0;
    let mut password = String::new();

    while password.len() < 8 {
        let hashed = append_and_hash(input, appended);
        if hashed.starts_with("00000") {
            password += hashed.chars().nth(5).unwrap().to_string().as_str();
        }
        appended += 1;
    }
    println!("{}", password);
}

#[allow(dead_code)]
pub fn part_two() {
    let input: &str = include_str!("../inputs/day05.txt");
    let mut appended = 0;
    let mut password: Vec<char> = vec!['_'; 8];

    while password.iter().any(|x| *x == '_') {
        let hashed = append_and_hash(input, appended);
        if hashed.starts_with("00000") {
            if let Some(i) = hashed.chars().nth(5).unwrap().to_digit(10) {
                if i < password.len() as u32 && password[i as usize] == '_' {
                    password[i as usize] = hashed.chars().nth(6).unwrap();
                }
            }
        }
        appended += 1;
    }
    println!("{}", password.iter().collect::<String>());
}
