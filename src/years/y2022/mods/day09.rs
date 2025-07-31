use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn left() -> Direction {
    Direction::Left
}

fn right() -> Direction {
    Direction::Right
}

fn up() -> Direction {
    Direction::Up
}

fn down() -> Direction {
    Direction::Down
}

struct Instr {
    dir: Direction,
    dist: i32,
}

impl From<&str> for Instr {
    fn from(value: &str) -> Self {
        let (dir, val_str) = value.split_once(" ").expect("failed to split once");
        let val: i32 = val_str.parse().expect("failed to parse i32");
        match dir {
            "L" => Instr {
                dir: left(),
                dist: val,
            },
            "R" => Instr {
                dir: right(),
                dist: val,
            },
            "U" => Instr {
                dir: up(),
                dist: val,
            },
            "D" => Instr {
                dir: down(),
                dist: val,
            },
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
    fn step(&mut self, dir: &Direction) -> Point;
    fn diff(&self, other: Point) -> Point;
    fn adjust_to(&mut self, h: Point) -> Point;
}
impl PointOps for Point {
    fn step(&mut self, dir: &Direction) -> Point {
        *self = match dir {
            Direction::Left => (self.0, self.1 - 1),
            Direction::Right => (self.0, self.1 + 1),
            Direction::Up => (self.0 - 1, self.1),
            Direction::Down => (self.0 + 1, self.1),
        };
        *self
    }
    fn diff(&self, other: Point) -> Point {
        (self.0 - other.0, self.1 - other.1)
    }
    fn adjust_to(&mut self, other: Point) -> Point {
        //
        // . h h h .
        // h . . . h
        // h . t . h
        // h . . . h
        // . h h h .
        //
        // simple_diffs = (0, 2), (0, -2), (2, 0), (-2, 0)
        //
        // diag_diffs = (-1, -2), (-2, -1), (-2, 1), (-1, 2),
        // (1, -2), (2, -1), (2, 1), (1, 2)
        //
        // . . . . .
        // . h h h .
        // . h t h .
        // . h h h .
        // . . . . .
        //
        // ignore = if diff.0.abs() < 2 && diff.1.abs() < 2
        //
        *self = match other.diff(*self) {
            // simple
            (0, -2) => self.step(&left()),
            (0, 2) => self.step(&right()),
            (-2, 0) => self.step(&up()),
            (2, 0) => self.step(&down()),
            // diagonal
            diff if [(-1, -2), (-2, -1)].contains(&diff) => self.step(&left()).step(&up()),
            diff if [(-2, 1), (-1, 2)].contains(&diff) => self.step(&right()).step(&up()),
            diff if [(1, -2), (2, -1)].contains(&diff) => self.step(&left()).step(&down()),
            diff if [(2, 1), (1, 2)].contains(&diff) => self.step(&right()).step(&down()),
            // ignore
            diff if diff.0.abs() < 2 && diff.1.abs() < 2 => *self,
            any_other_diff => {
                println!("(*3*)<(bad diff!): {:?}", any_other_diff);
                panic!("...");
            }
        };
        *self
    }
}

type Rope = (Point, Point);
trait RopeOps {
    fn print(&self);
}
impl RopeOps for Rope {
    fn print(&self) {
        let left = self.0 .1.min(self.1 .1) - 3;
        let right = self.0 .1.max(self.1 .1) + 3;
        let top = self.0 .0.min(self.1 .0) - 3;
        let bottom = self.0 .0.max(self.1 .0) + 3;

        for row in top..=bottom {
            for col in left..=right {
                if (row, col) == self.0 {
                    print!("h ");
                } else if (row, col) == self.1 {
                    print!("t ");
                } else {
                    print!(". ")
                }
            }
            println!("");
        }
    }
}

#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();

    let result = Instrs::input()
        .into_iter()
        .fold(
            (HashSet::new(), (0, 0), (0, 0)),
            |(mut visited, mut head, mut tail), instr| {
                for _ in 0..instr.dist {
                    // std::thread::sleep(std::time::Duration::from_millis(100));
                    // (head, tail).print();
                    // print!("\x1B[2J\x1B[1;1H");
                    head.step(&instr.dir);
                    tail.adjust_to(head);
                    visited.insert(tail);
                }
                (visited, head, tail)
            },
        )
        .0
        .len();

    println!("part_one={:#?}", result);
    println!("runtime={:?}", start.elapsed());
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();
    println!("part_two={:#?}", 0);
    println!("runtime={:?}", start.elapsed());
}
