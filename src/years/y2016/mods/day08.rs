use rand;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();

    let instructions = instructions();
    let screen_dimensions = ScreenDimensions {
        width: 50,
        height: 6,
    };
    let mut pixels: HashSet<(usize, usize)> = HashSet::new();
    for instruction in instructions {
        pixels.process_instruction(instruction, &screen_dimensions);
    }

    println!("part_one={:#?}", pixels.len());
    println!("runtime={:?}", start.elapsed());
}

fn instructions() -> Vec<InstructionEntry> {
    include_str!("../inputs/day08.txt")
        .lines()
        .filter(|x| !x.starts_with("//"))
        .map(|x| InstructionEntry::from(x))
        .collect()
}

#[derive(Debug, PartialEq, Clone)]
enum RowCol {
    Row,
    Col,
}

#[derive(Debug, Clone)]
enum InstructionEntry {
    NewRectangle {
        width: usize,
        height: usize,
    },
    Rotate {
        which: RowCol,
        number: usize,
        delta: usize,
    },
}

impl From<&str> for InstructionEntry {
    fn from(value: &str) -> Self {
        let (function_str, remainder) = value.split_once(" ").expect("failed to split once");

        match function_str {
            "rect" => {
                let (width, height) = remainder.split_once("x").expect("failed to split once");
                let (width, height) = (
                    width.parse::<usize>().expect("failed to parse to usize"),
                    height.parse::<usize>().expect("failed to parse to usize"),
                );
                InstructionEntry::NewRectangle { width, height }
            }
            "rotate" => {
                let (which_str, remainder) =
                    remainder.split_once(" ").expect("failed to split once");
                let which = match which_str {
                    "row" => RowCol::Row,
                    "column" => RowCol::Col,
                    _ => panic!("oops"),
                };
                let number = remainder
                    .split_whitespace()
                    .find(|x| x.contains(&"="))
                    .expect("equal sign not found")
                    .replace("y=", "")
                    .replace("x=", "")
                    .parse::<usize>()
                    .expect("failed to parse to usize");

                let delta = remainder
                    .split_whitespace()
                    .rev()
                    .next()
                    .expect("last not found")
                    .parse::<usize>()
                    .expect("failed to parse to usize");

                InstructionEntry::Rotate {
                    which,
                    number,
                    delta,
                }
            }
            _ => panic!("oops"),
        }
    }
}

type Pixel = (usize, usize);
type Pixels = HashSet<Pixel>;

trait PrintOnScreenSize {
    fn print_on(&self, screen_dimensions: &ScreenDimensions);
}

impl PrintOnScreenSize for Pixels {
    fn print_on(&self, screen_dimensions: &ScreenDimensions) {
        std::thread::sleep(std::time::Duration::from_millis(50));
        print!("\x1B[2J\x1B[1;1H");
        for row in 0..screen_dimensions.height {
            for col in 0..screen_dimensions.width {
                if self.contains(&(col, row)) {
                    print!("{} ", if rand::random::<bool>() { "1" } else { "0" });
                } else {
                    print!("  ");
                }
            }
            print!("\n");
        }
    }
}

trait ProcessInstruction {
    fn process_instruction(
        &mut self,
        instruction: InstructionEntry,
        screen_dimensions: &ScreenDimensions,
    );
}
impl ProcessInstruction for Pixels {
    fn process_instruction(
        &mut self,
        instruction: InstructionEntry,
        screen_dimensions: &ScreenDimensions,
    ) {
        match instruction {
            InstructionEntry::NewRectangle { width, height } => {
                // input=2x3
                // loop should translate to this
                // x 0 1 2 3
                // 0 # # . .
                // 1 # # . .
                // 2 # # . .
                // 3 . . . .
                for row in 0..height {
                    for col in 0..width {
                        self.insert((col, row));
                    }
                }
            }
            InstructionEntry::Rotate {
                which,
                number,
                delta,
            } => {
                // input=2x3
                // row 1 move by 2
                // match should translate to this
                // x 0 1 2 3
                // 0 # # . .
                // 1 . . # #
                // 2 # # . .
                // 3 . . . .
                let mut pxs_to_remove: Vec<(usize, usize)> = Vec::new();
                let mut pxs_to_add: Vec<(usize, usize)> = Vec::new();
                for pixel in self.iter() {
                    if which == RowCol::Row && pixel.1 == number {
                        pxs_to_remove.push(pixel.clone());
                        pxs_to_add.push(((pixel.0 + delta) % screen_dimensions.width, pixel.1));
                    } else if which == RowCol::Col && pixel.0 == number {
                        pxs_to_remove.push(pixel.clone());
                        pxs_to_add.push((pixel.0, (pixel.1 + delta) % screen_dimensions.height));
                    }
                }
                for px in pxs_to_remove {
                    self.remove(&px);
                }
                for px in pxs_to_add {
                    self.insert(px);
                }
            }
        }
        self.print_on(&screen_dimensions);
    }
}

struct ScreenDimensions {
    width: usize,
    height: usize,
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();
    println!("part_two={:?}", 0);
    println!("runtime={:?}", start.elapsed());
}
