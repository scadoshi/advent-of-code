use std::collections::HashMap;

#[allow(dead_code)]
pub fn part_one() {
    let bags: HashMap<String, HashMap<String, usize>> = include_str!("../inputs/day07.txt")
        .lines()
        .map(|line| {
            let parts = line.split_once("bags contain").unwrap();
            let bag = parts.0.trim().to_string();
            let mut reqs: HashMap<String, usize> = HashMap::new();
            if !parts.1.contains("no other bags") {
                reqs = parts
                    .1
                    .split(',')
                    .map(|x| {
                        let mut parts = x.trim().split_whitespace();
                        let qty = parts
                            .next()
                            .unwrap()
                            .parse::<usize>()
                            .expect(format!("parts\n{:?}\n", parts).as_str());
                        let name = parts.collect::<Vec<&str>>().join(" ");
                        (name, qty)
                    })
                    .collect();
            }
            (bag, reqs)
        })
        .collect();
    println!("{:#?}", bags);
    println!("{:?}", bags.len());
}

#[allow(dead_code)]
pub fn part_two() {}
