use std::collections::HashSet;

#[derive(Clone)]
enum Instr {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl From<&str> for Instr {
    fn from(value: &str) -> Self {
        let (op_str, val_str) = value.split_once(" ").expect("failed to split once");
        let val = val_str.parse::<i32>().expect("failed to parse i32");
        match op_str {
            "acc" => Self::Acc(val),
            "jmp" => Self::Jmp(val),
            "nop" => Self::Nop(val),
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
    println!("part_one()={:?}", instrs.run());
    println!("runtime={:#?}", start.elapsed());
}

#[derive(Debug)]
enum InstrsExitCode {
    #[allow(dead_code)]
    InfLoop(i32),
    #[allow(dead_code)]
    Succ(i32),
}

type Instrs = Vec<Instr>;
trait Run {
    fn run(&self) -> InstrsExitCode;
}
impl Run for Instrs {
    fn run(&self) -> InstrsExitCode {
        let mut p: usize = 0;
        let mut acc = 0;
        let mut seen: HashSet<usize> = HashSet::new();
        loop {
            seen.insert(p);

            match self[p] {
                Instr::Acc(val) => {
                    acc += val;
                    p += 1;
                }
                Instr::Jmp(val) => p = p.checked_add_signed(val as isize).expect("jmp ovrflw"),
                Instr::Nop(_) => p += 1,
            }

            if p >= self.len() {
                return InstrsExitCode::Succ(acc);
            }

            if seen.contains(&p) {
                return InstrsExitCode::InfLoop(acc);
            }
        }
    }
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();
    let instrs = Instr::input();
    let mut result: Option<InstrsExitCode> = None;
    for (i, instr) in instrs.iter().enumerate() {
        match instr {
            Instr::Acc(_) => continue,
            Instr::Nop(val) if *val == 0 => continue,
            instr => {
                let mut adj_instrs = instrs.clone();
                adj_instrs[i] = match instr {
                    Instr::Acc(_) => panic!("oops"), // should never happen
                    Instr::Jmp(val) => Instr::Nop(*val),
                    Instr::Nop(val) => Instr::Jmp(*val),
                };
                result = Some(adj_instrs.run());
                match result {
                    Some(InstrsExitCode::InfLoop(_)) => continue,
                    Some(InstrsExitCode::Succ(_)) => break,
                    None => panic!("oops"),
                }
            }
        }
    }
    println!("part_two()={:?}", result.expect("result not found"));
    println!("runtime={:#?}", start.elapsed());
}
