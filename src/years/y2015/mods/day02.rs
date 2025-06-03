#[allow(dead_code)]
pub fn part_one() {
    println!(
        "{}",
        include_str!("../inputs/day02.txt")
            .lines()
            .map(|line| {
                line.split('x')
                    .map(|dimension| {
                        dimension
                            .parse::<i32>()
                            .expect(format!("error converting {} to int", { dimension }).as_str())
                    })
                    .collect::<Vec<i32>>()
            })
            .map(|dimensions| {
                let (l, w, h) = (dimensions[0], dimensions[1], dimensions[2]);
                let mut sides = vec![l * w, w * h, l * h];
                sides.sort();
                sides[0] + sides.iter().map(|side| side * 2).sum::<i32>()
            })
            .sum::<i32>()
    );
}

#[allow(dead_code)]
pub fn part_two() {
    println!(
        "{}",
        std::fs::read_to_string("src/day_modules/input/two.txt")
            .expect("error reading file")
            .lines()
            .map(|line| {
                let mut dimensions = line
                    .split('x')
                    .map(|dimension| {
                        dimension
                            .parse::<i32>()
                            .expect(format!("error converting {} to int", { dimension }).as_str())
                    })
                    .collect::<Vec<i32>>();
                dimensions.sort();
                dimensions
            })
            .map(|dimensions| {
                (dimensions[0] * 2)
                    + (dimensions[1] * 2)
                    + (dimensions[0] * dimensions[1] * dimensions[2])
            })
            .sum::<i32>()
    );
}
