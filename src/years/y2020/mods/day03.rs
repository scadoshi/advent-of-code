#[allow(dead_code)]
fn input() -> Vec<Vec<char>> {
    include_str!("../inputs/day03.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[allow(dead_code)]
fn trees(dl: usize, dc: usize, grid: &Vec<Vec<char>>) -> isize {
    let (lines, cols, mut trees) = (grid.len(), grid[0].len(), 0);
    let (mut l, mut c) = (0, 0);
    while l < lines {
        (l, c) = (l + dl, (c + dc) % cols);
        if l >= lines {
            break;
        }
        if grid[l][c] == '#' {
            trees += 1;
        }
    }
    trees
}

#[allow(dead_code)]
pub fn part_one() {
    println!("{}", trees(1, 3, &input()));
}

#[allow(dead_code)]
pub fn part_two() {
    let grid = input();
    let mut trees_list: Vec<isize> = Vec::new();
    let move_combos: [(usize, usize); 5] = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    move_combos
        .into_iter()
        .for_each(|(dl, dc)| trees_list.push(trees(dl, dc, &grid)));
    println!("{}", trees_list.iter().product::<isize>());
}
