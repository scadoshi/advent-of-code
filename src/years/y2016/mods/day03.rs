fn is_triangle(a: i32, b: i32, c: i32) -> bool {
    a + b > c && b + c > a && a + c > b
}

pub fn part_one() {
    let valid_triangles =
        include_str!("../inputs/day03.txt")
            .lines()
            .fold(0, |mut valid_triangles, line| {
                let [a, b, c] = line
                    .split_whitespace()
                    .map(|num_str| num_str.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
                    .try_into()
                    .unwrap();
                if is_triangle(a, b, c) {
                    valid_triangles += 1;
                }
                valid_triangles
            });

    println!("{}", valid_triangles);
}

pub fn part_two() {
    let valid_triangles = include_str!("../inputs/day03.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i32>>>()
        .chunks(3)
        .fold(0, |mut valid_triangles, chunk| {
            for i in 0..3 {
                let [a, b, c] = [chunk[0][i], chunk[1][i], chunk[2][i]];
                if is_triangle(a, b, c) {
                    valid_triangles += 1;
                }
            }
            valid_triangles
        });

    println!("{}", valid_triangles);
}
