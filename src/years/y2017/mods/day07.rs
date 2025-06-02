use std::collections::{HashMap, VecDeque};

#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();
    let programs: Vec<Program> = include_str!("../inputs/day07.txt")
        .lines()
        .map(|l| Program::from(l))
        .collect();

    let root = programs
        .iter()
        .find(|p| {
            let children: Vec<&String> = programs.iter().flat_map(|p| &p.children).collect();
            !children.contains(&&p.name)
        })
        .expect("root not found");

    println!("part_one={:?} ... runtime={:?}", root, start.elapsed());
}

#[derive(Debug, Clone)]
struct Program {
    name: String,
    weight: usize,
    children: Vec<String>,
}

impl Program {
    fn from(str: &str) -> Self {
        let name = str[..str.find("(").expect("opening parenthesis not found")]
            .trim()
            .to_string();
        let weight = str[str.find("(").expect("opening parenthesis not found") + 1
            ..str.find(")").expect(") not found")]
            .parse::<usize>()
            .expect("failed to parse usize");
        let mut children = Vec::new();
        if let Some((_, children_str)) = str.split_once("->") {
            children = children_str
                .split(",")
                .map(|x| x.trim().to_string())
                .collect();
        }
        Program {
            name,
            weight,
            children,
        }
    }
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();
    let programs: Vec<Program> = include_str!("../inputs/day07.txt")
        .lines()
        .map(|l| Program::from(l))
        .collect();
    let root = programs
        .iter()
        .find(|p| {
            let children: Vec<&String> = programs.iter().flat_map(|p| &p.children).collect();
            !children.contains(&&p.name)
        })
        .expect("root not found");

    println!("part_two={:#?} ... runtime={:?}", "", start.elapsed());
}
