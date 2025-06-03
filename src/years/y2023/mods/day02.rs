use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
struct Move {
    color: String,
    count: i32,
}

impl Move {
    #[allow(dead_code)]
    fn new(input: &str) -> Self {
        // e.g. "10 green"
        let [color, count] = input
            .split_whitespace()
            .rev()
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();
        Self {
            color: color.parse().unwrap(),
            count: count.parse().unwrap(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Game {
    id: i32,
    moves: Vec<Move>,
}

impl Game {
    #[allow(dead_code)]
    fn new(input: &str) -> Self {
        let [game_id_str, paramater_str] =
            input.split(":").collect::<Vec<&str>>().try_into().unwrap();

        let id: i32 = game_id_str[input.find("Game ").unwrap() + 5..input.find(":").unwrap()]
            .parse()
            .expect(
                format!(
                    "Error converting {} to i32",
                    input[input.find("Game ").unwrap() + 6..input.find(":").unwrap()].to_string()
                )
                .as_str(),
            );

        let moves: Vec<Move> = paramater_str
            .to_string()
            .replace(";", ",")
            .split(",")
            .map(|move_str| Move::new(move_str))
            .collect();

        Self { id, moves }
    }
}

#[allow(dead_code)]
fn limits() -> HashMap<String, i32> {
    HashMap::from([
        ("blue".to_string(), 14),
        ("green".to_string(), 13),
        ("red".to_string(), 12),
    ])
}

fn input() -> Vec<Game> {
    include_str!("../inputs/day02.txt")
        .lines()
        .map(|line| Game::new(line))
        .collect()
}

#[allow(dead_code)]
pub fn part_one() {
    let limits = limits();
    let input = input();

    let result = input
        .iter()
        .filter(|game| {
            game.moves
                .iter()
                .all(|x| x.count <= *limits.get(&x.color).unwrap())
        })
        .map(|game| game.id)
        .sum::<i32>();

    println!("{}", result);
}

#[allow(dead_code)]
pub fn part_two() {
    let input = input();

    let result = input
        .iter()
        .fold(
            Vec::new(),
            |mut counts_list: Vec<HashMap<String, i32>>, game| {
                let mut counts: HashMap<String, i32> = HashMap::new();
                game.moves.iter().for_each(|x| {
                    if !counts.contains_key(&x.color) || x.count > *counts.get(&x.color).unwrap() {
                        counts.insert(x.color.clone(), x.count);
                    }
                });
                counts_list.push(counts);
                counts_list
            },
        )
        .iter()
        .map(|x| x.values().product::<i32>())
        .sum::<i32>();

    println!("{}", result);
}
