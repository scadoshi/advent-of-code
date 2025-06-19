trait Abba {
    fn is_abba(&self) -> bool;
}
impl Abba for [char] {
    fn is_abba(&self) -> bool {
        if self.len() > 4 {
            return false;
        }
        self[0] != self[1] && self[0] == self[3] && self[1] == self[2]
    }
}

trait Aba {
    fn is_aba(&self) -> bool;
    fn counterpart(&self) -> [char; 3];
}
impl Aba for [char] {
    fn is_aba(&self) -> bool {
        if self.len() > 3 {
            return false;
        }
        self[0] != self[1] && self[0] == self[2]
    }
    fn counterpart(&self) -> [char; 3] {
        [self[1], self[0], self[1]]
    }
}

fn input() -> Vec<String> {
    include_str!("../inputs/day07.txt")
        .lines()
        .map(|x| x.to_string())
        .collect()
}

#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();
    println!(
        "part_one={:?} ... runtime={:?}",
        {
            input()
                .into_iter()
                .map(|ip_address| {
                    let mut in_brackets = false;
                    let mut abba_found = false;
                    for window in ip_address.chars().collect::<Vec<char>>().windows(4) {
                        if window.contains(&'[') {
                            in_brackets = true;
                            continue;
                        }
                        if window.contains(&']') {
                            in_brackets = false;
                            continue;
                        }
                        if window.is_abba() && in_brackets {
                            return false;
                        }
                        if window.is_abba() && !in_brackets {
                            abba_found = true;
                        }
                    }
                    abba_found
                })
                .filter(|x| *x)
                .count()
        },
        start.elapsed()
    );
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();
    println!(
        "part_two={:?} ... runtime={:?}",
        {
            input()
                .into_iter()
                .map(|ip_address| {
                    let mut in_brackets = false;
                    let mut outside_abas: Vec<[char; 3]> = Vec::new();
                    let mut inside_abas: Vec<[char; 3]> = Vec::new();
                    for window in ip_address.chars().collect::<Vec<char>>().windows(3) {
                        if window.contains(&'[') {
                            in_brackets = true;
                            continue;
                        }
                        if window.contains(&']') {
                            in_brackets = false;
                            continue;
                        }

                        if window.is_aba() && in_brackets {
                            if outside_abas.contains(&window.counterpart()) {
                                return true;
                            }
                            // unwrap is safe because windows ensures only three elements
                            inside_abas.push(window.try_into().unwrap());
                        }
                        if window.is_aba() && !in_brackets {
                            if inside_abas.contains(&window.counterpart()) {
                                return true;
                            }
                            // unwrap is safe because windows ensures only three elements
                            outside_abas.push(window.try_into().unwrap());
                        }
                    }
                    false
                })
                .filter(|x| *x)
                .count()
        },
        start.elapsed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[allow(dead_code)]
    fn counterpart_simple() {
        let input = ['a', 'b', 'a'];
        let result = input.counterpart();
        let expected = ['b', 'a', 'b'];
        assert_eq!(result, expected)
    }
}
