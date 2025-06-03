use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
fn wires() -> Vec<Vec<String>> {
    include_str!("../inputs/day03.txt")
        .lines()
        .map(|x| x.split(",").map(|y| y.to_string()).collect())
        .collect()
}

#[allow(dead_code)]
pub fn part_one() {
    let wires = wires();

    let mut wire_map: HashMap<(i32, i32), HashSet<i32>> = HashMap::new();
    let (mut x, mut y) = (0, 0);

    for (i, wire) in wires.iter().enumerate() {
        for instr in wire {
            let dir = instr.chars().next().unwrap();
            let dist = instr
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<i32>()
                .unwrap();

            for _ in 0..dist {
                match dir {
                    'R' => x += 1,
                    'L' => x -= 1,
                    'U' => y += 1,
                    'D' => y -= 1,
                    _ => {
                        println!("f");
                    }
                }
                wire_map.entry((x, y)).or_default().insert(i as i32);
            }
        }
        (x, y) = (0, 0);
    }

    let result = wire_map
        .iter()
        .filter(|(_, wire_nums)| wire_nums.len() == 2)
        .map(|((x, y), _)| x.abs() + y.abs())
        .min()
        .unwrap();

    println!("{:?}", result);
}

#[allow(dead_code)]
pub fn part_two() {
    let wires = wires();
    let mut steps = 0;
    let mut wire_map: HashMap<(i32, i32), HashMap<i32, i32>> = HashMap::new();
    let (mut x, mut y) = (0, 0);

    for (i, wire) in wires.iter().enumerate() {
        for instr in wire {
            let dir = instr.chars().next().unwrap();
            let dist = instr
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<i32>()
                .unwrap();

            for _ in 0..dist {
                steps += 1;
                match dir {
                    'R' => x += 1,
                    'L' => x -= 1,
                    'U' => y += 1,
                    'D' => y -= 1,
                    _ => (),
                }

                if !wire_map
                    .entry((x, y))
                    .or_default()
                    .contains_key(&(i as i32))
                {
                    wire_map.entry((x, y)).or_default().insert(i as i32, steps);
                }
            }
        }
        (x, y) = (0, 0);
        steps = 0;
    }

    let result = wire_map
        .into_iter()
        .filter(|(_, wire_nums)| wire_nums.len() == 2)
        .map(|(_, x)| x.values().sum::<i32>())
        .min()
        .unwrap();

    println!("{:?}", result);
}
