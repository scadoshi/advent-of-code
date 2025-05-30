use tokio::time::Instant;

fn nodes() -> Vec<(String, String)> {
    include_str!("../inputs/day06.txt")
        .lines()
        .map(|line| {
            let (p, c) = line.split_once(")").expect("failed to split once");
            let (p, c) = (p.to_string(), c.to_string());
            (p, c)
        })
        .collect()
}

#[allow(dead_code)]
pub fn part_one() {
    let start = Instant::now();
    // actual code
    {
        let nodes = nodes();
        for end in nodes.iter().map(|node| node.1) {
            
        }

    }
    println!("{:?}", start.elapsed());
}

#[allow(dead_code)]
pub fn part_two() {}
