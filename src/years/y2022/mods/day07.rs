use std::{collections::HashMap, path::PathBuf};

type Dirs = HashMap<PathBuf, Vec<String>>;
fn dirs() -> Dirs {
    include_str!("../inputs/day07.txt")
        .split("$")
        .skip(1)
        .fold(
            (HashMap::<PathBuf, Vec<String>>::new(), PathBuf::new()),
            |(mut dirs, mut path), line| {
                let mut content: Vec<String> = line.trim().lines().map(|x| x.to_string()).collect();
                let function = content.remove(0);
                let children = content;

                match function.as_str() {
                    "cd .." => {
                        path = path.parent().unwrap().to_path_buf();
                    }
                    x if function.contains("cd") => {
                        let to = x.split_once(" ").expect("failed to split once").1;
                        path.push(to)
                    }
                    "ls" => {
                        dirs.insert(path.clone(), children.clone());
                    }
                    _ => {
                        panic!("oops")
                    }
                }
                // println!("===\nfunction={:?}\nchildren={:?}", function, children);
                (dirs, path)
            },
        )
        .0
}

fn size(path: &PathBuf, full_dir: &Dirs) -> usize {
    let children = full_dir
        .iter()
        .find(|x| x.0 == path)
        .expect(format!("path {:#?} not found", path).as_str())
        .1;

    let memory: usize = children
        .iter()
        .map(|x| {
            let (left, right) = x.split_once(" ").expect("failed to split once");

            match left {
                "dir" => {
                    let mut new_path = path.clone();
                    new_path.push(right);
                    size(&new_path, full_dir)
                }
                memory_str => memory_str
                    .parse::<usize>()
                    .expect("failed to parse to usize"),
            }
        })
        .sum();

    memory
}

#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();
    let result: usize = {
        let dirs = dirs();
        dirs.keys()
            .map(|x| {
                let size = size(x, &dirs);
                if size > 100000 {
                    0
                } else {
                    size
                }
            })
            .sum()
    };
    println!("part_one={:#?}", result);
    println!("runtime={:?}", start.elapsed());
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();
    let dirs = dirs();
    let max = 70000000;
    let used = size(&PathBuf::from("/"), &dirs);
    let free = max - used;
    let needed = 30000000;
    let target = needed - free;

    let result = dirs
        .iter()
        .filter_map(|x| {
            let size = size(x.0, &dirs);
            if size >= target {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .expect("min not found");

    println!("part_two={:?}", result);
    println!("runtime={:?}", start.elapsed());
}
