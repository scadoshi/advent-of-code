use std::collections::HashSet;

#[allow(dead_code)]
pub fn part_one() {
    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let (mut x, mut y): (i32, i32) = (0, 0);

    include_str!("../inputs/day03.txt")
        .chars()
        .for_each(|direction| {
            if direction == '^' {
                (x, y) = (x, y + 1);
            } else if direction == '>' {
                (x, y) = (x + 1, y);
            } else if direction == 'v' {
                (x, y) = (x, y - 1);
            } else if direction == '<' {
                (x, y) = (x - 1, y);
            }
            visited.insert((x, y));
        });
    println!("{}", visited.into_iter().count());
}

#[allow(dead_code)]
pub fn part_two() {
    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let (mut x1, mut y1): (i32, i32) = (0, 0);
    let (mut x2, mut y2): (i32, i32) = (0, 0);

    std::fs::read_to_string("src/day_modules/input/three.txt")
        .expect("error reading file")
        .chars()
        .enumerate()
        .for_each(|(i, direction)| {
            if i % 2 != 0 {
                if direction == '^' {
                    (x1, y1) = (x1, y1 + 1);
                } else if direction == '>' {
                    (x1, y1) = (x1 + 1, y1);
                } else if direction == 'v' {
                    (x1, y1) = (x1, y1 - 1);
                } else if direction == '<' {
                    (x1, y1) = (x1 - 1, y1);
                }
                visited.insert((x1, y1));
            }
            if i % 2 == 0 {
                if direction == '^' {
                    (x2, y2) = (x2, y2 + 1);
                } else if direction == '>' {
                    (x2, y2) = (x2 + 1, y2);
                } else if direction == 'v' {
                    (x2, y2) = (x2, y2 - 1);
                } else if direction == '<' {
                    (x2, y2) = (x2 - 1, y2);
                }
                visited.insert((x2, y2));
            }
        });
    println!("{}", visited.into_iter().count());
}
