use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
struct Content {
    content_type: ContentType,
    name: String,
    memory: Option<usize>,
}

#[derive(Debug)]
enum ContentType {
    File,
    Dir,
}

#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();

    let dirs: HashMap<PathBuf, Vec<Content>> = include_str!("../inputs/day07.txt")
        .split("$")
        .skip(1)
        .fold(
            (HashMap::new(), PathBuf::new()),
            |(mut dirs, mut path), x| {
                let mut content = x.trim().split("\r\n").collect::<Vec<&str>>();
                let function = content.remove(0);
                println!("===\nfunction={:?}\ncontent={:?}", function, content);

                match function {
                    "cd .." => path = path.parent().expect("failed to go back").to_path_buf(),
                    x if function.contains("cd") => {
                        let to_dir = x.split_once(" ").unwrap().1;
                        path.push(to_dir);
                    }
                    "ls" => {
                        let content: Vec<Content> = content
                            .into_iter()
                            .map(|x| {
                                let content_type = if x.contains("dir ") {
                                    ContentType::Dir
                                } else {
                                    ContentType::File
                                };
                                // figure out how to get the full path here
                                let name = path
                                    + x.split_whitespace()
                                        .nth(1)
                                        .expect("dir not found")
                                        .to_string();

                                let memory = match content_type {
                                    ContentType::Dir => None,
                                    ContentType::File => Some(
                                        x.split_whitespace()
                                            .next()
                                            .expect("next not found")
                                            .parse::<usize>()
                                            .expect("failed to parse to usize"),
                                    ),
                                };

                                Content {
                                    content_type,
                                    name,
                                    memory,
                                }
                            })
                            .collect();
                        dirs.insert(path.clone(), content);
                    }
                    _ => panic!("oops"),
                }
                println!("{:?}", path);
                (dirs, path)
            },
        )
        .0;

    println!("part_one={:#?}", dirs);
    println!("runtime={:?}", start.elapsed());
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();
    println!("part_two={:?}", 0);
    println!("runtime={:?}", start.elapsed());
}

// # notes
//
// ## required functions
// ### current dir
// - indicated by commands like `cd dir_name`
//
// ### dir history
// - keep track of dir path for commands like `cd ..`
//
// ### dir contents
// - indicated by commands like `ls` which are following by dir contents
// - files are preceeded by a number representing memory
// - then something like `file_name.file_extension`
// - dirs are preceeded by the `dir` keyword then something like `dir_name`
//
// ## how do we want to store all of this?
// - recursive structure makes sense
// - but will have to store flatly as a first step no matter what
