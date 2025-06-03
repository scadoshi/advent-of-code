#[allow(dead_code)]
pub fn part_one() {
    println!(
        "{}",
        include_str!("../inputs/day02.txt")
            .lines()
            .map(|x| x.replace("X", "Rock"))
            .map(|x| x.replace("Y", "Paper"))
            .map(|x| x.replace("Z", "Scissors"))
            .map(|x| x.replace("A", "Rock"))
            .map(|x| x.replace("B", "Paper"))
            .map(|x| x.replace("C", "Scissors"))
            .fold(0, |mut score, line| {
                // println!("{}", line);

                let opp_move: &str = line.split_whitespace().next().unwrap();
                let my_move: &str = line.split_whitespace().nth(1).unwrap();

                match my_move {
                    "Rock" => {
                        score += 1;
                        match opp_move {
                            "Rock" => score += 3,
                            "Paper" => (),
                            "Scissors" => score += 6,
                            _ => {
                                println!("r f");
                            }
                        }
                    }
                    "Paper" => {
                        score += 2;
                        match opp_move {
                            "Rock" => score += 6,
                            "Paper" => score += 3,
                            "Scissors" => (),
                            _ => {
                                println!("p f");
                            }
                        }
                    }
                    "Scissors" => {
                        score += 3;
                        match opp_move {
                            "Rock" => (),
                            "Paper" => score += 6,
                            "Scissors" => score += 3,
                            _ => {
                                println!("s f");
                            }
                        }
                    }
                    _ => {
                        println!("{my_move}");
                    }
                }
                score
            })
    )
}

#[allow(dead_code)]
pub fn part_two() {
    println!(
        "{}",
        include_str!("../inputs/day02.txt")
            .lines()
            .map(|x| x.replace("X", "Lose"))
            .map(|x| x.replace("Y", "Draw"))
            .map(|x| x.replace("Z", "Win"))
            .map(|x| x.replace("A", "Rock"))
            .map(|x| x.replace("B", "Paper"))
            .map(|x| x.replace("C", "Scissors"))
            .fold(0, |mut score, line| {
                // println!("{}", line);

                let opp_move: &str = line.split_whitespace().next().unwrap();
                let my_move: &str = line.split_whitespace().nth(1).unwrap();

                match my_move {
                    "Lose" => match opp_move {
                        "Rock" => score += 3,
                        "Paper" => score += 1,
                        "Scissors" => score += 2,
                        _ => {
                            println!("r f");
                        }
                    },
                    "Draw" => {
                        score += 3;
                        match opp_move {
                            "Rock" => score += 1,
                            "Paper" => score += 2,
                            "Scissors" => score += 3,
                            _ => {
                                println!("p f");
                            }
                        }
                    }
                    "Win" => {
                        score += 6;
                        match opp_move {
                            "Rock" => score += 2,
                            "Paper" => score += 3,
                            "Scissors" => score += 1,
                            _ => {
                                println!("s f");
                            }
                        }
                    }
                    _ => {
                        println!("{my_move}");
                    }
                }
                score
            })
    )
}
