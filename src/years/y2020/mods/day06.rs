use std::collections::HashSet;

#[allow(dead_code)]
pub fn part_one() {
    println!(
        "{:#?}",
        include_str!("../inputs/day06.txt")
            .split("\n\n")
            .fold(Vec::new(), |mut groups: Vec<usize>, group_str| {
                let groups_answers: HashSet<char> =
                    group_str.lines().flat_map(|x| x.chars()).collect();
                groups.push(groups_answers.len());
                groups
            })
            .into_iter()
            .sum::<usize>()
    );
}

#[allow(dead_code)]
pub fn part_two() {
    println!(
        "{:?}",
        include_str!("../inputs/day06.txt")
            .split("\n\n")
            .fold(Vec::new(), |mut groups: Vec<usize>, group_str| {
                let mut total = 0;

                let distinct_qs: HashSet<char> =
                    group_str.lines().flat_map(|x| x.chars()).collect();

                for q in distinct_qs {
                    if group_str.lines().all(|answ_str| answ_str.contains(q)) {
                        total += 1;
                    }
                }
                groups.push(total);
                groups
            })
            .into_iter()
            .sum::<usize>()
    );
}
