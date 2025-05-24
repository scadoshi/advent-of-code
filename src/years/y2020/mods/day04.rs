fn input() -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut group: String = String::from("");
    for line in include_str!("../inputs/day04.txt").lines() {
        match line {
            "" => {
                output.push(group.clone());
                group.clear();
            }
            _ => {
                group = (group + " " + line).trim().to_string();
            }
        }
    }
    output.push(group.clone());
    output
}
#[allow(dead_code)]
pub fn part_one() {
    let mut total = 0;
    for passport in input() {
        if passport.contains(&"byr")
            && passport.contains(&"iyr")
            && passport.contains(&"eyr")
            && passport.contains(&"hgt")
            && passport.contains(&"hcl")
            && passport.contains(&"ecl")
            && passport.contains(&"pid")
        {
            total += 1;
        }
    }
    println!("{}", total);
}

#[allow(dead_code)]
pub fn part_two() {
    let mut total = 0;
    'main: for passport in input() {
        for field in passport
            .split_whitespace()
            .map(|field| field.split(':').collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>()
        {
            if !passport.contains(&"byr")
                || !passport.contains(&"iyr")
                || !passport.contains(&"eyr")
                || !passport.contains(&"hgt")
                || !passport.contains(&"hcl")
                || !passport.contains(&"ecl")
                || !passport.contains(&"pid")
            {
                continue 'main;
            }

            let key = field[0];
            let value = field[1];

            if key == "byr" {
                let year = value.parse::<i32>().unwrap();
                if year < 1920 || year > 2002 {
                    continue 'main;
                }
            }

            if key == "iyr" {
                let year = value.parse::<i32>().unwrap();
                if year < 2010 || year > 2020 {
                    continue 'main;
                }
            }

            if key == "eyr" {
                let year = value.parse::<i32>().unwrap();
                if year < 2020 || year > 2030 {
                    continue 'main;
                }
            }

            if key == "hgt" {
                let units = if value.contains("cm") {
                    "cm"
                } else if value.contains("in") {
                    "in"
                } else {
                    continue 'main;
                };

                let number: i32 = value.replace("cm", "").replace("in", "").parse().unwrap();

                if (units == "cm" && (number < 150 || number > 193))
                    || (units == "in" && (number < 59 || number > 76))
                {
                    continue 'main;
                }
            }

            if key == "hcl" {
                if !value.contains("#") || value.replace("#", "").len() != 6 {
                    continue 'main;
                }

                for char in value.replace("#", "").to_lowercase().chars() {
                    if !"0123456789abcdef".contains(char) {
                        continue 'main;
                    }
                }
            }

            if key == "ecl" {
                if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value) {
                    continue 'main;
                }
            }

            if key == "pid" {
                if value.len() != 9 {
                    continue 'main;
                }
            }
        }

        total += 1
    }
    println!("{}", total);
}
