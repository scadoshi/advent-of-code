#[allow(dead_code)]
fn even_digited(number: usize) -> bool {
    number.to_string().len() % 2 == 0
}

use std::collections::HashMap;

#[allow(dead_code)]
struct StoneSystem {
    stones: Vec<usize>,
    cache: HashMap<(usize, usize), usize>,
}

impl StoneSystem {
    #[allow(dead_code)]
    fn new() -> Self {
        StoneSystem {
            stones: std::fs::read_to_string("src/years/y2024/inputs/day11.txt")
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
            cache: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    fn blink_stones(&mut self, blink_count: usize) -> usize {
        self.blink_alot_r(self.stones.clone(), blink_count)
    }

    #[allow(dead_code)]
    fn blink_alot_r(&mut self, nums: Vec<usize>, blink_count: usize) -> usize {
        if blink_count == 0 {
            return nums.len();
        }

        let mut count = 0;
        for number in nums {
            count += self.process_number(number, blink_count);
        }
        count
    }

    #[allow(dead_code)]
    fn process_number(&mut self, number: usize, blink_count: usize) -> usize {
        if self.cache.contains_key(&(number, blink_count)) {
            return self.cache.get(&(number, blink_count)).unwrap().clone();
        }

        let result = self.blink_alot_r(blink(number), blink_count - 1);
        self.cache.insert((number, blink_count), result);
        result
    }
}

#[allow(dead_code)]
fn blink(number: usize) -> Vec<usize> {
    if number == 0 {
        return vec![1];
    } else if even_digited(number) {
        let num_str = number.to_string();
        let (left, right) = num_str.split_at(num_str.len() / 2);
        vec![
            left.parse::<usize>().unwrap(),
            right.parse::<usize>().unwrap(),
        ]
    } else {
        vec![number * 2024]
    }
}

#[allow(dead_code)]
pub fn part_one() {
    let mut system = StoneSystem::new();
    println!("{}", system.blink_stones(25));
}

#[allow(dead_code)]
pub fn part_two() {
    let mut system = StoneSystem::new();
    println!("{}", system.blink_stones(75));
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    #[allow(dead_code)]
    fn test_blink_1() {
        let expected = vec![2024];
        let result = blink(1);
        assert_eq!(expected, result)
    }

    #[ignore]
    #[allow(dead_code)]
    fn test_blink_0() {
        let expected = vec![1];
        let result = blink(0);
        assert_eq!(expected, result)
    }

    #[ignore]
    #[allow(dead_code)]
    fn test_blink_10() {
        let expected = vec![1, 0];
        let result = blink(10);
        assert_eq!(expected, result)
    }
}
