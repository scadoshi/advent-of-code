#[allow(dead_code)]
pub fn part_one() {
    let mut maze: Vec<isize> = include_str!("../inputs/day05.txt")
        .lines()
        .map(|x| x.parse::<isize>().unwrap())
        .collect();

    let mut position: isize = 0;
    let mut count: isize = 0;

    while (position as usize) < maze.len() {
        let jump = maze[position as usize];
        maze[position as usize] += 1;
        position += jump;
        count += 1;
    }

    println!("{}", count);
}

#[allow(dead_code)]
pub fn part_two() {
    let mut maze: Vec<isize> = include_str!("../inputs/day05.txt")
        .lines()
        .map(|x| x.parse::<isize>().unwrap())
        .collect();

    let mut position: isize = 0;
    let mut count: isize = 0;

    while (position as usize) < maze.len() {
        let jump = maze[position as usize];
        if maze[position as usize] >= 3 {
            maze[position as usize] -= 1;
        } else {
            maze[position as usize] += 1;
        }
        position += jump;
        count += 1;
    }

    println!("{}", count);
}
