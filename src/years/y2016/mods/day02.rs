pub fn part_one() {
    let keypad: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    let (mut row, mut col) = (1 as i32, 1 as i32);

    let mut code: Vec<i32> = Vec::new();

    for line in include_str!("../inputs/day02.txt")
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
    {
        let end_of_the_line = line.len() - 1;
        for (i, dir) in line.iter().enumerate() {
            let last = (row.clone(), col.clone());

            (row, col) = match dir {
                'U' => (row - 1, col),
                'R' => (row, col + 1),
                'D' => (row + 1, col),
                'L' => (row, col - 1),
                _ => (row, col),
            };

            if row > 2 || col > 2 || row < 0 || col < 0 {
                (row, col) = last;
            }

            if i == end_of_the_line {
                code.push(keypad[row as usize][col as usize].clone());
            }
        }
    }

    println!("{}", code.iter().map(|x| x.to_string()).collect::<String>());
}
pub fn part_two() {
    let keypad: Vec<Vec<char>> = vec![
        vec!['_', '_', '1', '_', '_'],
        vec!['_', '2', '3', '4', '_'],
        vec!['5', '6', '7', '8', '9'],
        vec!['_', 'A', 'B', 'C', '_'],
        vec!['_', '_', 'D', '_', '_'],
    ];

    let (mut row, mut col) = (1 as i32, 1 as i32);

    let mut code: Vec<char> = Vec::new();

    for line in include_str!("..//inputs//day02.txt")
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
    {
        let end_of_the_line = line.len() - 1;
        for (i, dir) in line.iter().enumerate() {
            let last = (row.clone(), col.clone());

            (row, col) = match dir {
                'U' => (row - 1, col),
                'R' => (row, col + 1),
                'D' => (row + 1, col),
                'L' => (row, col - 1),
                _ => (row, col),
            };

            if row > 4 || col > 4 || row < 0 || col < 0 || keypad[row as usize][col as usize] == '_'
            {
                (row, col) = last;
            }

            if i == end_of_the_line {
                code.push(keypad[row as usize][col as usize].clone());
            }
        }
    }

    println!("{}", code.iter().map(|x| x.to_string()).collect::<String>());
}
