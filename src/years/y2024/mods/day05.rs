use std::collections::HashMap;

#[allow(dead_code)]
fn rules() -> HashMap<i32, Vec<i32>> {
    include_str!("../inputs/day05.txt")
        .lines()
        .filter(|line| line.contains('|'))
        .fold(HashMap::new(), |mut map: HashMap<i32, Vec<i32>>, line| {
            let mut numbers = line
                .split('|')
                .map(|num| num.trim().parse::<i32>().unwrap());

            let key = numbers.next().unwrap();
            let value = numbers.next().unwrap();

            map.entry(key).or_insert_with(|| Vec::new()).push(value);
            map
        })
}

#[allow(dead_code)]
fn seqs() -> Vec<Vec<i32>> {
    include_str!("../inputs/day05.txt")
        .lines()
        .filter_map(|line| {
            if line.contains(',') {
                Some(
                    line.split(',')
                        .map(|num| num.trim().parse::<i32>().unwrap())
                        .collect(),
                )
            } else {
                None
            }
        })
        .collect()
}

#[allow(dead_code)]
pub fn part_one() {
    let total: i32 = seqs()
        .iter()
        .filter(|seq| {
            let mut seen = Vec::new();
            seq.iter().all(|&num| {
                let valid = rules().get(&num).map_or(true, |rule_nums| {
                    !rule_nums.iter().any(|rule_num| seen.contains(rule_num))
                });
                seen.push(num);
                valid
            })
        })
        .map(|valid_seq| valid_seq[valid_seq.len() / 2])
        .sum();
    println!("{total}");
}

#[allow(dead_code)]
/// Validates a sequence against dependency rules
///
/// # Arguments
/// * `seq` - Slice containing the sequence of numbers to validate
/// * `rules` - HashMap containing dependency rules where key must come after all values
///
/// # Returns
/// Tuple containing:
/// * Boolean indicating if sequence is valid
/// * Option with index of violating number (if invalid)
/// * Option with index of violated rule (if invalid)
fn seq_is_valid(
    seq: &[i32],
    rules: &HashMap<i32, Vec<i32>>,
) -> (bool, Option<usize>, Option<usize>) {
    // Initialize seen numbers with first element which is always valid by definition
    let mut seen: Vec<i32> = vec![seq[0]];

    // Check each subsequent number against dependency rules
    for (i, &num) in seq.iter().enumerate().skip(1) {
        // Check if current number has any dependency rules
        if let Some(rule) = rules.get(&num) {
            // Find first rule violation and its position in one pass
            if let Some(violated_i) = rule
                .iter()
                .find_map(|rnum| seen.iter().position(|&num| num == *rnum))
            {
                return (false, Some(i), Some(violated_i));
            }
        }
        seen.push(num);
    }
    (true, None, None)
}

#[allow(dead_code)]
pub fn part_two() {
    let mut total = 0;
    let seqs = seqs();
    let rules = rules();

    for mut seq in seqs {
        let (mut valid, mut violating_i, mut violated_i) = seq_is_valid(&seq, &rules);
        if !valid {
            // only perform logic on the invalid seqs
            while !valid {
                // continue to loop until seq is valid
                seq.swap(violating_i.unwrap(), violated_i.unwrap());
                (valid, violating_i, violated_i) = seq_is_valid(&seq, &rules)
            }
            total += seq[seq.len() / 2];
        }
    }
    println!("{total}");
}
