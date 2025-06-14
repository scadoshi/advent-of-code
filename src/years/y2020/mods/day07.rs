use std::collections::{HashMap, HashSet, VecDeque};

fn rules() -> HashMap<String, HashMap<String, usize>> {
    include_str!("../inputs/day07.txt")
        .lines()
        .filter(|x| !x.starts_with("//"))
        .map(|line| {
            let parts = line.split_once("bags contain").unwrap();
            let bag = parts.0.trim().to_string();
            let mut contents: HashMap<String, usize> = HashMap::new();
            if !parts.1.contains("no other bags") {
                contents = parts
                    .1
                    .split(',')
                    .map(|x| {
                        let mut parts = x.trim().split_whitespace();
                        let qty = parts
                            .next()
                            .unwrap()
                            .parse::<usize>()
                            .expect(format!("parts\n{:?}\n", parts).as_str());
                        let name = parts
                            .filter(|p| !p.contains("bag"))
                            .collect::<Vec<&str>>()
                            .join(" ");
                        (name, qty)
                    })
                    .collect();
            }
            (bag, contents)
        })
        .collect()
}

#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();
    let rules = rules();
    let mut queue: VecDeque<String> = VecDeque::from(["shiny gold".to_string()]);
    let mut result: HashSet<String> = HashSet::new();

    while let Some(check) = queue.pop_front() {
        let found: Vec<String> = rules
            .iter()
            .filter(|(_, contents)| contents.contains_key(&check))
            .map(|(id, _)| id.clone())
            .collect();

        result.extend(found.clone());
        queue.extend(found);
    }

    println!("part_one={:?}", result.len());
    println!("runtime={:?}", start.elapsed());
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();
    let rules = rules();

    let mut queue: VecDeque<(String, usize)> = VecDeque::from([("shiny gold".to_string(), 1)]);
    let mut result: usize = 0;

    while let Some((id, count)) = queue.pop_front() {
        if let Some(contents) = rules.get(&id) {
            result += count * contents.iter().map(|c| c.1).sum::<usize>();
            queue.extend(
                contents
                    .into_iter()
                    .map(|(cid, ccount)| (cid.clone(), ccount * count))
                    .collect::<Vec<(String, usize)>>(),
            );
        }
    }

    println!("part_two={:?}", result);
    println!("runtime={:?}", start.elapsed());
}
