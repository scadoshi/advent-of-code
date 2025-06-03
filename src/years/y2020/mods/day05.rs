#[allow(dead_code)]
struct BoardingPass {
    id: usize,
    row: usize,
    col: usize,
}

impl BoardingPass {
    #[allow(dead_code)]
    fn new(id: usize, row: usize, col: usize) -> Self {
        Self { id, row, col }
    }

    #[allow(dead_code)]
    fn input() -> Vec<BoardingPass> {
        let mut boarding_passes = include_str!("../inputs/day05.txt").lines().fold(
            Vec::new(),
            |mut boarding_passes: Vec<BoardingPass>, line| {
                let row_instr: Vec<char> = line.chars().take(7).collect();
                let col_instr: Vec<char> = line.chars().skip(7).take(3).collect();

                let mut row = 0;
                let mut row_max = 127;
                for char in row_instr {
                    let diff = row_max - row + 1;
                    if char == 'F' {
                        row_max -= diff / 2;
                    } else {
                        row += diff / 2;
                    }
                }

                let mut col = 0;
                let mut col_max = 7;
                for char in col_instr {
                    let diff = col_max - col + 1;
                    if char == 'L' {
                        col_max -= diff / 2;
                    } else {
                        col += diff / 2;
                    }
                }
                let id = (row_max * 8) + col_max;

                boarding_passes.push(BoardingPass::new(id, row, col));
                boarding_passes
            },
        );
        boarding_passes.sort_by(|a, b| a.id.cmp(&b.id));
        boarding_passes
    }
}

#[allow(dead_code)]
pub fn part_one() {
    println!(
        "{}",
        BoardingPass::input()
            .iter()
            .max_by(|a, b| a.id.cmp(&b.id))
            .unwrap()
            .id
    );
}

#[allow(dead_code)]
pub fn part_two() {
    let boarding_passes = BoardingPass::input();
    for window in boarding_passes.windows(2) {
        if window[1].id as isize - window[0].id as isize > 1 {
            println!("{}", window[0].id + 1);
        }
    }
}
