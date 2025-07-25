use std::collections::HashSet;

#[derive(Debug)]
enum Instr {
    L(i32),
    R(i32),
    U(i32),
    D(i32),
}

impl From<&str> for Instr {
    fn from(value: &str) -> Self {
        let (dir, val_str) = value.split_once(" ").expect("failed to split once");
        let val: i32 = val_str.parse().expect("failed to parse i32");
        match dir {
            "L" => Instr::L(val),
            "R" => Instr::R(val),
            "U" => Instr::U(val),
            "D" => Instr::D(val),
            _ => panic!("input dir is not valid direction"),
        }
    }
}

type Instrs = Vec<Instr>;
trait Input {
    fn input() -> Self;
}
impl Input for Instrs {
    fn input() -> Self {
        include_str!("../inputs/day09.txt")
            .lines()
            .map(|x| Instr::from(x))
            .collect()
    }
}

type Point = (i32, i32);
trait PointOps {
    fn traverse(&self, instr: Instr) -> Point;
    fn point_diff(&self, other: &Point) -> Point;
    fn update_tail(&self, t: &Point) -> Point;
}
impl PointOps for Point {
    fn traverse(&self, instr: Instr) -> Self {
        let (row, col) = *self;
        match instr {
            Instr::L(val) => (row, col - val),
            Instr::R(val) => (row, col + val),
            Instr::U(val) => (row + val, col),
            Instr::D(val) => (row - val, col),
        }
    }
    fn point_diff(&self, other: &Point) -> Point {
        (self.0 - other.0, self.1 - other.1)
    }
    fn update_tail(&self, t: &Point) -> Point {
        // . . . .
        // . . . .
        // . t . .
        // . . . .
        // . . . .
        //
        // move_tail = [
        //     (0, 2), (0, -2), (2, 0), (-2, 0)
        // ]
        // can_ignore = [
        //     any where h.point_diff(t).0 and .1 are .abs() < 2
        // ]

        let (row_diff, col_diff) = self.point_diff(t);

        (0, 0)
    }
}

#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();

    let instrs: Instrs = Instrs::input();
    println!("{:?}", instrs);

    println!("part_one={:#?}", 0);
    println!("runtime={:?}", start.elapsed());
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();
    println!("part_two={:#?}", 0);
    println!("runtime={:?}", start.elapsed());
}
