use std::collections::HashMap;

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
struct Instruction {
    coordinate1: (i32, i32),
    coordinate2: (i32, i32),
    toggle: Option<bool>,
}

impl Instruction {
    #[allow(dead_code)]
    fn new(instruction_string: &str) -> Self {
        // turn on 887,9 through 959,629
        // ['turn', 'on', '887,9', 'through', '959,629']
        // ['887,9', '959,629']
        // in flat map
        // [[887, 9], [959, 629]]
        // [887, 9, 959, 629]
        let nums: Vec<i32> = instruction_string
            .split_whitespace()
            .filter(|x| x.contains(','))
            .flat_map(|y| {
                y.split(',')
                    .map(|z| z.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect();

        let coordinate1 = (nums[0], nums[1]);

        let coordinate2 = (nums[2], nums[3]);

        let toggle: Option<bool> = if instruction_string.contains("turn on") {
            Some(true)
        } else if instruction_string.contains("turn off") {
            Some(false)
        } else {
            None
        };

        Self {
            coordinate1,
            coordinate2,
            toggle,
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
struct Instructions {
    list: Vec<Instruction>,
}

impl Instructions {
    #[allow(dead_code)]
    fn new(instructions_str: &str) -> Self {
        let list: Vec<Instruction> = instructions_str
            .lines()
            .map(|instruction_str| Instruction::new(instruction_str))
            .collect();
        Self { list }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
struct Lights {
    coordinate_list: HashMap<(i32, i32), bool>,
}

impl Lights {
    #[allow(dead_code)]
    fn new() -> Self {
        let coordinate_list = (0..1000)
            .flat_map(|x| (0..1000).map(move |y| ((x, y), false)))
            .collect();
        Self { coordinate_list }
    }

    #[allow(dead_code)]
    fn toggle(self: &mut Self, instruction: Instruction) {
        for col in instruction.coordinate1.0..=instruction.coordinate2.0 {
            for row in instruction.coordinate1.1..=instruction.coordinate2.1 {
                self.coordinate_list.insert(
                    (row, col),
                    instruction
                        .toggle
                        .unwrap_or(!*self.coordinate_list.get(&(row, col)).unwrap()),
                );
            }
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
struct Lights2 {
    coordinate_list: HashMap<(i32, i32), i32>,
}

impl Lights2 {
    #[allow(dead_code)]
    fn new() -> Self {
        let coordinate_list = (0..1000)
            .flat_map(|x| (0..1000).map(move |y| ((x, y), 0)))
            .collect();
        Self { coordinate_list }
    }

    #[allow(dead_code)]
    fn toggle(self: &mut Self, instruction: Instruction) {
        for col in instruction.coordinate1.0..=instruction.coordinate2.0 {
            for row in instruction.coordinate1.1..=instruction.coordinate2.1 {
                self.coordinate_list.entry((row, col)).and_modify(|v| {
                    *v += match instruction.toggle {
                        Some(true) => 1,
                        Some(false) => {
                            if *v > 0 {
                                -1
                            } else {
                                0
                            }
                        }
                        None => 2,
                    }
                });
            }
        }
    }
}

#[allow(dead_code)]
pub fn part_one() {
    let mut lights = Lights::new();

    for instruction in Instructions::new(include_str!("../inputs/day06.txt")).list {
        lights.toggle(instruction);
    }

    let lights_on_count = lights
        .coordinate_list
        .values()
        .filter(|value| **value)
        .count();

    println!("{}", lights_on_count);
}

#[allow(dead_code)]
pub fn part_two() {
    let mut lights = Lights2::new();

    for instruction in Instructions::new(include_str!("..//inputs//day06.txt")).list {
        lights.toggle(instruction);
    }

    let total_brightness = lights.coordinate_list.values().sum::<i32>();

    println!("{}", total_brightness);
}

// turn on (2, 4) through (4, 6)
// .............
// .............
// ....XXX......
// ....XXX......
// ....XXX......
// .............
// .............
// col range 2 to 4
// row range 4 to 6
