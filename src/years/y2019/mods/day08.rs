#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();
    let px_size = 3;
    let screen_width = 25;
    let _screen_height = 6;

    let chars = include_str!("../inputs/day08.txt")
        .chars()
        .collect::<Box<[char]>>();

    let pixels_ungrouped = chars
        .chunks(px_size)
        .map(|x| Box::from(x))
        .collect::<Box<[Box<[char]>]>>();

    let pixels_grouped = pixels_ungrouped
        .chunks(screen_width)
        .map(|x| Box::from(x))
        .collect::<Box<[Box<[Box<[char]>]>]>>();

    let r0west = pixels_grouped
        .clone()
        .iter()
        .enumerate()
        .map(|(i, x)| (i, x.iter().flatten().filter(|c| **c == '0').count()))
        .min_by_key(|(_, x)| *x)
        .expect("min not found")
        .0;

    let r0west1count = pixels_grouped
        .get(r0west)
        .expect("r0west not found")
        .iter()
        .flatten()
        .filter(|c| **c == '1')
        .count();

    let r0west2count = pixels_grouped
        .get(r0west)
        .expect("r0west not found")
        .iter()
        .flatten()
        .filter(|c| **c == '2')
        .count();

    println!("part_one={:#?}", (r0west, r0west1count, r0west2count));
    println!("runtime={:?}", start.elapsed());
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();
    println!("part_one={:?}", 0);
    println!("runtime={:?}", start.elapsed());
}
