fn react(chars: &Vec<char>) -> Vec<char> {
    let mut new_chars: Vec<char> = chars.clone();
    let mut removal: Vec<usize> = Vec::new();
    loop {
        for i in 0..new_chars.len() {
            if removal.contains(&i) {
                continue;
            }
            if i + 1 < new_chars.len()
                && new_chars[i].to_lowercase().next().unwrap()
                    == new_chars[i + 1].to_lowercase().next().unwrap()
                && new_chars[i] != new_chars[i + 1]
            {
                removal.extend([i, i + 1].into_iter());
            }
        }
        if removal.is_empty() {
            break;
        }
        for i in removal.iter().rev() {
            new_chars.remove(*i);
        }
        removal.clear();
    }
    new_chars
}

fn chars() -> Vec<char> {
    include_str!("../inputs/day05.txt").chars().collect()
}

#[allow(dead_code)]
pub fn part_one() {
    println!("{}", react(&chars()).len());
}

#[allow(dead_code)]
pub fn part_two() {
    let mut least: i32 = std::i32::MAX;
    for a in 'a' as usize..'z' as usize {
        let char = a as u8 as char;
        let chars: Vec<char> = chars()
            .into_iter()
            .filter(|x| x.to_lowercase().next().unwrap() != char)
            .collect();
        let reaction_len = react(&chars).len() as i32;
        if reaction_len < least {
            least = reaction_len;
        }
    }
    println!("{}", least);
}
