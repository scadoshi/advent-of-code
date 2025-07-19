use std::collections::{hash_map::Entry, HashMap, VecDeque};

#[derive(Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

fn next_node(curr_node: &String, instr: &Instr, nodes: &Vec<Node>) -> String {
    match instr {
        Instr::Left => nodes
            .iter()
            .find(|x| x.id == *curr_node)
            .expect("left node not found")
            .left
            .to_owned(),
        Instr::Right => nodes
            .iter()
            .find(|x| x.id == *curr_node)
            .expect("right node not found")
            .right
            .to_owned(),
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        // value: "GNK = (LBV, QNP)"
        let mut parts = value.split_whitespace();
        let id = parts.next().expect("id not found").to_string();
        parts.next(); // discard equal sign
        let left = parts
            .next()
            .expect("left not found")
            .replace([',', '('], "")
            .replace(",", "");
        let right = parts.next().expect("right not found").replace(")", "");
        Self { id, left, right }
    }
}

#[derive(Debug)]
enum Instr {
    Left,
    Right,
}

#[derive(Debug)]
enum InstrError {
    InvalidInput,
}

impl TryFrom<char> for Instr {
    type Error = InstrError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            _ if value == 'L' => Ok(Self::Left),
            _ if value == 'R' => Ok(Self::Right),
            _ => Err(InstrError::InvalidInput),
        }
    }
}

#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();

    let mut all_inp = include_str!("../inputs/day08.txt").lines();
    let instrs: Vec<Instr> = all_inp
        .next()
        .expect("instrs not found")
        .chars()
        .filter_map(|x| Instr::try_from(x).ok())
        .collect();
    let nodes: Vec<Node> = all_inp.skip(1).map(|x| Node::from(x)).collect();

    let mut curr_node: String = "AAA".to_string();
    let mut instr_p = 0; // instr pointer
    let mut steps: usize = 0;

    while curr_node != "ZZZ" {
        let _ = 0;
        curr_node = next_node(&curr_node, &instrs[instr_p], &nodes);
        instr_p = (instr_p + 1) % instrs.len();
        steps += 1;
    }

    println!("part_one={:#?}", steps);
    println!("runtime={:.2?}", start.elapsed());
}

fn gcd(num1: &usize, num2: &usize) -> usize {
    if *num1 == 0 && *num2 == 0 {
        return 0;
    }

    let mut a = *num1;
    let mut b = *num2;

    while b != 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }

    return a;
}

fn lcm(num1: usize, num2: usize) -> usize {
    return num1 * num2 / gcd(&num1, &num2);
}

fn lcm_vec(nums: Vec<usize>) -> Option<usize> {
    if nums.is_empty() {
        None
    } else {
        let mut result = nums[0];
        for num in nums.into_iter().skip(1) {
            result = lcm(result, num);
        }
        Some(result)
    }
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();

    let mut all_inp = include_str!("../inputs/day08.txt").lines();
    let instrs: Vec<Instr> = all_inp
        .next()
        .expect("instrs not found")
        .chars()
        .filter_map(|x| Instr::try_from(x).ok())
        .collect();
    let nodes: Vec<Node> = all_inp.skip(1).map(|x| Node::from(x)).collect();

    let mut steps: usize = 0;
    let mut instr_p = 0;

    let mut ghosts: VecDeque<String> = nodes
        .iter()
        .filter(|x| x.id.ends_with("A"))
        .map(|x| x.id.to_owned())
        .collect();

    // should contain a single cycle data point
    // on the second cycle we calculate fequency
    // and then skip thereafter
    let mut ghost_data: HashMap<usize, usize> = HashMap::new();

    // contains calculated frequencies
    let mut ghost_freqs: HashMap<usize, usize> = HashMap::new();

    while ghost_freqs.len() < ghosts.len() {
        for ghost_num in 0..ghosts.len() {
            let curr_node = ghosts.pop_front().expect("node not found");

            // freq calc-ing || adds entry to ghost_data
            if curr_node.ends_with("Z") && !ghost_freqs.contains_key(&ghost_num) {
                match ghost_data.entry(ghost_num) {
                    Entry::Occupied(entry) => {
                        let freq = steps - entry.get();
                        ghost_freqs.insert(ghost_num, freq);
                    }
                    Entry::Vacant(_) => {
                        ghost_data.insert(ghost_num, steps);
                    }
                }
            }

            let next_node = next_node(&curr_node, &instrs[instr_p], &nodes);
            ghosts.push_back(next_node);
        }

        instr_p = (instr_p + 1) % instrs.len();
        steps += 1;

        if steps % 100000 == 0 {
            println!("{:?}", steps);
        }
    }

    println!(
        "part_two={:#?}",
        lcm_vec(ghost_freqs.into_iter().map(|x| x.1).collect::<Vec<usize>>())
            .expect("lcm not found")
    );
    println!("runtime={:.2?}", start.elapsed());
}
