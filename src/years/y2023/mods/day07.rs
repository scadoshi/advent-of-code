use std::{cmp::Ordering, collections::HashMap};

#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();

    let mut hands: Vec<Hand> = input();
    hands.sort_by(|a, b| a.compare_hands(&b));
    let result: usize = hands
        .iter()
        .map(|x| x.bet)
        .enumerate()
        .map(|(i, bet)| {
            let ranking = i + 1;
            let winnings = bet * ranking;
            winnings
        })
        .sum();
    println!("part_one={:#?}", result);
    println!("runtime={:?}", start.elapsed());
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();

    let mut hands: Vec<Hand> = joker_input();
    hands.sort_by(|a, b| a.joker_compare_hands(&b));
    let result: usize = hands
        .iter()
        .map(|x| x.bet)
        .enumerate()
        .map(|(i, bet)| {
            let ranking = i + 1;
            let winnings = bet * ranking;
            winnings
        })
        .sum();
    println!("part_two={:?}", result);
    println!("runtime={:?}", start.elapsed());
}

#[derive(Debug)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn strength(&self) -> usize {
        match self {
            Self::FiveKind => 7,
            Self::FourKind => 6,
            Self::FullHouse => 5,
            Self::ThreeKind => 4,
            Self::TwoPair => 3,
            Self::OnePair => 2,
            Self::HighCard => 1,
        }
    }
}

impl From<&str> for HandType {
    fn from(value: &str) -> Self {
        let mut counts: Vec<usize> = value
            .chars()
            .fold(HashMap::<char, usize>::new(), |mut counts, card| {
                *counts.entry(card).or_default() += 1;
                counts
            })
            .into_values()
            .collect();
        counts.sort();
        match counts
            .iter()
            .map(|x| x.to_string())
            .collect::<String>()
            .as_str()
        {
            "11111" => Self::HighCard,
            "1112" => Self::OnePair,
            "122" => Self::TwoPair,
            "113" => Self::ThreeKind,
            "14" => Self::FourKind,
            "5" => Self::FiveKind,
            "23" => Self::FullHouse,
            _ => panic!("oops"),
        }
    }
}

trait JokerFrom<T> {
    fn joker_from(value: T) -> Self;
}

impl JokerFrom<&str> for HandType {
    fn joker_from(value: &str) -> Self {
        let mut counts: HashMap<char, usize> =
            value.chars().fold(HashMap::new(), |mut counts, card| {
                *counts.entry(card).or_default() += 1;
                counts
            });

        if let Some(joker_count) = counts.remove(&'J') {
            if counts.is_empty() {
                // All cards are jokers, treat as five of a kind
                return Self::FiveKind;
            }

            // Find the card with the highest count (excluding jokers)
            let majority_card = counts
                .iter()
                .max_by(|a, b| a.1.cmp(&b.1))
                .expect("max not found")
                .0
                .clone();

            *counts.entry(majority_card).or_default() += joker_count;
        }

        let mut counts: Vec<&usize> = counts.values().collect();
        counts.sort();

        match counts
            .iter()
            .map(|x| x.to_string())
            .collect::<String>()
            .as_str()
        {
            "11111" => Self::HighCard,
            "1112" => Self::OnePair,
            "122" => Self::TwoPair,
            "113" => Self::ThreeKind,
            "14" => Self::FourKind,
            "5" => Self::FiveKind,
            "23" => Self::FullHouse,
            _ => panic!("oops"),
        }
    }
}

impl JokerFrom<&str> for Hand {
    fn joker_from(value: &str) -> Self {
        let (hand_str, bet) = value.split_once(" ").expect("failed to split once");
        let hand_type = HandType::joker_from(hand_str);
        Self {
            hand_str: hand_str.to_string(),
            hand_type,
            bet: bet.parse::<usize>().expect("failed to parse to usize"),
        }
    }
}

#[derive(Debug)]
struct Hand {
    hand_str: String,
    hand_type: HandType,
    bet: usize,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let (hand_str, bet) = value.split_once(" ").expect("failed to split once");
        let hand_type = HandType::from(hand_str);
        Self {
            hand_str: hand_str.to_string(),
            hand_type,
            bet: bet.parse::<usize>().expect("failed to parse to usize"),
        }
    }
}

impl Hand {
    fn compare_hands(&self, other_hand: &Hand) -> Ordering {
        let comparison = self
            .hand_type
            .strength()
            .cmp(&other_hand.hand_type.strength());
        match comparison {
            Ordering::Equal => {
                let self_strengths: Vec<usize> =
                    self.hand_str.chars().map(|x| x.strength()).collect();
                let other_hand_strengths: Vec<usize> =
                    other_hand.hand_str.chars().map(|x| x.strength()).collect();
                self_strengths.cmp(&other_hand_strengths)
            }
            comparison => comparison,
        }
    }

    fn joker_compare_hands(&self, other_hand: &Hand) -> Ordering {
        let comparison = self
            .hand_type
            .strength()
            .cmp(&other_hand.hand_type.strength());
        match comparison {
            Ordering::Equal => {
                let self_strengths: Vec<usize> =
                    self.hand_str.chars().map(|x| x.joker_strength()).collect();
                let other_hand_strengths: Vec<usize> = other_hand
                    .hand_str
                    .chars()
                    .map(|x| x.joker_strength())
                    .collect();
                self_strengths.cmp(&other_hand_strengths)
            }
            comparison => comparison,
        }
    }
}

fn input() -> Vec<Hand> {
    include_str!("../inputs/day07.txt")
        .lines()
        .map(|x| Hand::from(x))
        .collect()
}

fn joker_input() -> Vec<Hand> {
    include_str!("../inputs/day07.txt")
        .lines()
        .map(|x| Hand::joker_from(x))
        .collect()
}

trait Card {
    fn strength(&self) -> usize;
    fn joker_strength(&self) -> usize;
}

impl Card for char {
    fn strength(&self) -> usize {
        match self {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            _ => panic!("oops!"),
        }
    }

    fn joker_strength(&self) -> usize {
        match self {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            'J' => 1,
            _ => panic!("oops!"),
        }
    }
}
