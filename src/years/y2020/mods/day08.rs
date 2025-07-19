use std::collections::HashSet;

enum Instr {
    Acc(i32),
    Jmp(i32),
    Nop,
}

impl From<&str> for Instr {
    fn from(value: &str) -> Self {
        let (op_str, val_str) = value.split_once(" ").expect("failed to split once");
        let val = val_str.parse::<i32>().expect("failed to parse i32");
        match op_str {
            "acc" => Self::Acc(val),
            "jmp" => Self::Jmp(val),
            "nop" => Self::Nop,
            _ => panic!("oops"),
        }
    }
}

impl Instr {
    fn input() -> Vec<Self> {
        include_str!("../inputs/day08.txt")
            .lines()
            .map(|x| Instr::from(x))
            .collect()
    }
}

#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();

    let instrs = Instr::input();
    let mut p: usize = 0;
    let mut acc = 0;
    let mut seen: HashSet<usize> = HashSet::new();

    while !seen.contains(&p) {
        seen.insert(p);

        match instrs[p] {
            Instr::Acc(val) => {
                acc += val;
                p += 1;
            }
            Instr::Jmp(val) => p = p.checked_add_signed(val as isize).expect("jmp ovrflw"),
            Instr::Nop => p += 1,
        }
    }

    println!("part_one()={:#?}", acc);
    println!("runtime={:#?}", start.elapsed());
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();
    println!("part_two()={:#?}", 0);
    println!("runtime={:#?}", start.elapsed());
}
