use std::{collections::HashMap, path::PathBuf};

enum _Content {
    File {
        path: PathBuf,
        memory: usize,
    },
    Dir {
        path: PathBuf,
        children: Vec<_Content>,
    },
}

fn flat_input() -> HashMap<PathBuf, Vec<String>> {
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

                println!("===\nfunction={:?}\nchildren={:?}", function, children);

                (dirs, path)
            },
        )
        .0
}

#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();
    let dirs = flat_input();
    println!("part_one={:#?}", dirs);
    println!("runtime={:?}", start.elapsed());
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();
    println!("part_two={:?}", 0);
    println!("runtime={:?}", start.elapsed());
}
