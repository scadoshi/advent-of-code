use std::collections::HashMap;

#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();

    let mut regs: HashMap<String, i32> = HashMap::new();
    let mut high_opt: Option<i32> = None;

    for line in include_str!("../inputs/day08.txt").lines() {
        let mut parts = line.split_whitespace();
        let reg = parts.next().expect("reg not found").to_string();
        let dir = parts.next().expect("dir not found");
        let d: i32 = parts
            .next()
            .expect("d not found")
            .parse()
            .expect("failed to parse to i32");
        let _ = parts.next().expect("if not found"); // discarding this
        let creg = parts.next().expect("creg not found").to_string();
        let cop = parts.next().expect("cop not found");
        let cnum: i32 = parts
            .next()
            .expect("cd not found")
            .parse()
            .expect("failed to parse to i32");

        if !regs.contains_key(&creg) {
            regs.insert(creg.clone(), 0);
        }
        let creg_val = regs[&creg];
        let reg_mut_val = regs.entry(reg).or_default();

        let condition_met = match cop {
            "<" => creg_val < cnum,
            ">" => creg_val > cnum,
            "<=" => creg_val <= cnum,
            ">=" => creg_val >= cnum,
            "==" => creg_val == cnum,
            "!=" => creg_val != cnum,
            _ => panic!("oops"),
        };

        if condition_met {
            match dir {
                "inc" => *reg_mut_val += d,
                "dec" => *reg_mut_val -= d,
                _ => panic!("oops"),
            }
        }

        if high_opt.is_none() || *reg_mut_val > high_opt.unwrap() {
            high_opt = Some(reg_mut_val.clone());
        }
    }

    println!(
        "part_one={:#?}",
        regs.into_iter()
            .max_by(|(_, a), (_, b)| a.cmp(&b))
            .expect("max not found")
            .1
    );
    println!("part_two={:#?}", high_opt.expect("highest not found"));
    println!("runtime={:#?}", start.elapsed());
}

#[allow(dead_code)]
pub fn part_two() {
    // let start = std::time::Instant::now();
    // println!("part_one={:#?}\nruntime={:#?}", 0, start.elapsed());
}
