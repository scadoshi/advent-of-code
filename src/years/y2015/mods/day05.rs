#[allow(dead_code)]
pub fn part_one() {
    let input: Vec<String> = include_str!("../inputs/day05.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut nice_string_count = 0;

    'main: for string in input {
        // println!("===\nevaluating: {string}");

        // three vowels
        if string
            .chars()
            .filter(|char| "aeiou".contains(*char))
            .count()
            < 3
        {
            // println!("oops - less than three vowels");
            continue;
        }

        // double letters
        let mut last = string.chars().next().unwrap();
        let mut double_found = false;
        for current in string.chars().skip(1) {
            if current == last {
                double_found = true;
                break;
            }
            last = current;
        }

        if !double_found {
            // println!("oops - no doubles");
            continue;
        }

        // bad pairs
        let mut last = string.chars().next().unwrap();
        for current in string.chars().skip(1) {
            if ["ab", "cd", "pq", "xy"].contains(&format!("{}{}", last, current).as_str()) {
                // println!("oops - bad pair");
                continue 'main;
            }
            last = current;
        }

        // println!("nice!");
        nice_string_count += 1;
    }
    println!("{}", nice_string_count);
}

#[allow(dead_code)]
pub fn part_two() {
    let input: Vec<String> = std::fs::read_to_string("src/day_modules/input/five.txt")
        .expect("error reading file")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut nice_string_count = 0;

    for string in input {
        // double non-overlapping pair e.g. "aaaa" (aa), "eddie_is_unhinged" (ed) or "momo" (mo)
        if !string
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .any(|pair| {
                let pair_string: String = pair.iter().collect::<String>();
                string.matches(pair_string.as_str()).count() > 1
            })
        {
            continue;
        }

        // double with middle e.g. "pitiless_plunderer" (ere or iti) or "pooop" (ooo)
        if !string
            .chars()
            .collect::<Vec<char>>()
            .windows(3)
            .any(|triplet| triplet[0] == triplet[2])
        {
            continue;
        }

        nice_string_count += 1;
    }

    println!("{}", nice_string_count);
}
