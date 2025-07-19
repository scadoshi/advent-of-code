trait NextNum {
    fn next_num(&self) -> i32;
}
impl NextNum for Vec<i32> {
    fn next_num(&self) -> i32 {
        let mut numss: Vec<Vec<i32>> = Vec::from([self.clone()]);
        while numss[numss.len() - 1].iter().sum::<i32>() != 0 {
            // println!("{:#?}", numss);
            numss.push(
                numss[numss.len() - 1]
                    .windows(2)
                    .map(|w| w[1] - w[0])
                    .collect(),
            );
        }
        let mut incr: i32 = 0;
        for nums in numss.iter().rev() {
            incr += nums[nums.len() - 1];
        }
        return incr;
    }
}

#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();

    let numss: Vec<Vec<i32>> = include_str!("../inputs/day09.txt")
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i32>().expect("failed to parse i32"))
                .collect::<Vec<i32>>()
        })
        .collect();

    let result: i32 = numss.into_iter().map(|x| x.next_num()).sum();

    println!("part_one={:#?}", result);
    println!("runtime={:.2?}", start.elapsed());
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();
    println!("part_two={:#?}", 0);
    println!("runtime={:.2?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn next_num_test_1() {
        let input: Vec<i32> = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(input.next_num(), 18)
    }

    #[test]
    fn next_num_test_2() {
        let input: Vec<i32> = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(input.next_num(), 28)
    }

    #[test]
    fn next_num_test_3() {
        let input: Vec<i32> = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(input.next_num(), 68)
    }
}
