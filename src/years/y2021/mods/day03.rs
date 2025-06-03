use std::collections::HashMap;

fn input() -> Vec<String> {
    include_str!("../inputs/day03.txt")
        .lines()
        .map(|x| x.to_string())
        .collect()
}

/// takes Vec<String> (String represents bits i.e. "11001")
/// return a Vec<Tuple> representing counts of zero and one in respective indeces
#[allow(dead_code)]
fn index_bit_counts(input: &Vec<String>) -> Vec<(i32, i32)> {
    // count bits at each index among all lines
    let mut map: HashMap<usize, (i32, i32)> = HashMap::new();
    for i in 0..input[0].len() {
        for line in input.iter() {
            match line.chars().nth(i).unwrap() {
                '0' => map.entry(i).or_default().0 += 1,
                '1' => map.entry(i).or_default().1 += 1,
                _ => (),
            }
        }
    }
    // turn into vec for sorting and sort
    let mut sorted: Vec<(usize, (i32, i32))> = map.into_iter().collect();
    sorted.sort_by(|x, y| x.0.cmp(&y.0));

    // extract tuples only
    let result: Vec<(i32, i32)> = sorted.iter().map(|line| line.1).collect();

    // return result
    result
}

#[allow(dead_code)]
fn max_bit_str(input: &Vec<(i32, i32)>) -> String {
    input
        .iter()
        .map(|(zero, one)| {
            // println!("zero = {}, one = {}", zero, one); // debug
            if zero > one {
                '0'
            } else if one > zero {
                '1'
            } else {
                '1'
            }
        })
        .collect::<String>()
}

#[allow(dead_code)]
fn min_bit_str(input: &Vec<(i32, i32)>) -> String {
    input
        .iter()
        .map(|(zero, one)| {
            if zero > one {
                '1'
            } else if one > zero {
                '0'
            } else {
                '0'
            }
        })
        .collect::<String>()
}

#[allow(dead_code)]
pub fn part_one() {
    let input = input();
    let index_bit_counts = index_bit_counts(&input);
    let g_rate = max_bit_str(&index_bit_counts);
    let e_rate = min_bit_str(&index_bit_counts);
    println!(
        "{}",
        u32::from_str_radix(g_rate.as_str(), 2).unwrap()
            * u32::from_str_radix(e_rate.as_str(), 2).unwrap()
    );
}

#[allow(dead_code)]
fn function(input: &Vec<String>, toggle: bool) -> String {
    // println!("# starting nums = {:?}", input.iter().map(|x| x.to_string() + ", ").collect::<String>()); // debug
    // keeping track of what is invalid
    let mut invalid: Vec<String> = Vec::new();
    // have to evaluate in index order
    let mut i = 0;
    // goes until we only have one left
    while invalid.len() < input.len() - 1 {
        // calculate new input considering removed
        let input_adj: Vec<String> = input
            .clone()
            .into_iter()
            .filter(|x| !invalid.contains(x))
            .collect();
        // calulate new rate
        let rate = if toggle {
            max_bit_str(&index_bit_counts(&input_adj))
        } else {
            min_bit_str(&index_bit_counts(&input_adj))
        };
        // println!("## rate = {}\n### index {} should equal {}", rate, i, rate.chars().nth(i).unwrap()); // debug
        for line in input.iter() {
            if line.chars().nth(i).unwrap() != rate.chars().nth(i).unwrap()
                && !invalid.contains(line)
            {
                invalid.push(line.to_string());
                // println!("- removed {}", line); // debug
            }
        }
        // println!( // debug
        //     "# nums = {:?}", // debug
        //     input.iter().filter(|x| !invalid.contains(x)).map(|x| x.to_string() + ", ").collect::<String>() // debug
        // ); // debug
        i += 1;
    }

    input
        .iter()
        .filter(|line| !invalid.contains(line))
        .collect::<Vec<&String>>()[0]
        .clone()
}

#[allow(dead_code)]
pub fn part_two() {
    let input = input();
    // oxygen generator rating
    let og_rate = function(&input, true);
    // CO2 scrubber rating
    let cs_rate = function(&input, false);

    // println!("{} {}", og_rate, cs_rate); // debug

    println!(
        "{}",
        u32::from_str_radix(og_rate.as_str(), 2).unwrap()
            * u32::from_str_radix(cs_rate.as_str(), 2).unwrap()
    );
}
