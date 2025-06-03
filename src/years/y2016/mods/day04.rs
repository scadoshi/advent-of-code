use std::collections::HashMap;

fn rooms() -> Vec<(String, i32, String)> {
    include_str!("../inputs/day04.txt")
        .lines()
        .fold(Vec::new(), |mut rooms, line| {
            let encrypted_name = line[..line.len() - 11].to_string();

            let checksum = line[line.chars().position(|x| x == '[').unwrap() + 1
                ..line.chars().position(|x| x == ']').unwrap()]
                .to_string();

            let sector_id: i32 = line[line.chars().position(|x| x == '[').unwrap() - 3
                ..line.chars().position(|x| x == '[').unwrap()]
                .parse()
                .unwrap();

            rooms.push((encrypted_name, sector_id, checksum));
            rooms
        })
}

pub fn part_one() {
    let mut total = 0;

    let rooms: Vec<(String, i32, String)> = rooms();

    for room in rooms.iter() {
        let mut letter_counts: Vec<(char, i32)> = room
            .0
            .chars()
            .fold(
                HashMap::new(),
                |mut letter_counts: HashMap<char, i32>, letter| {
                    if letter != '-' {
                        *letter_counts.entry(letter).or_insert(0) += 1;
                    }
                    letter_counts
                },
            )
            .into_iter()
            .collect::<Vec<(char, i32)>>();

        letter_counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

        let calculated_checksum: String =
            letter_counts[0..5].iter().map(|x| x.0).collect::<String>();

        if calculated_checksum == room.2 {
            total += room.1;
        }
    }

    println!("{}", total);
}
pub fn part_two() {
    let rooms: Vec<(String, i32, String)> = rooms();

    let decoded_rooms = rooms
        .iter()
        .fold(Vec::new(), |mut decoded_rooms: Vec<String>, room| {
            decoded_rooms.push(
                room.1.to_string()
                    + " - "
                    + &room
                        .0
                        .chars()
                        .map(|letter| {
                            if letter == '-' {
                                return ' ';
                            }
                            let a = 'a' as i32;
                            char::from(
                                u8::try_from(((letter as i32 - a + room.1) % 26) + a).unwrap(),
                            )
                        })
                        .collect::<String>(),
            );
            decoded_rooms
        });

    println!("{:#?}", decoded_rooms);
    println!(
        "{:#?}",
        decoded_rooms.iter().find(|room_string| {
            room_string.contains("north")
                || room_string.contains("pole")
                || room_string.contains("object")
        })
    )
}
