trait ToBool {
    fn to_bool(&self) -> bool;
}

impl ToBool for char {
    fn to_bool(&self) -> bool {
        match self {
            '1' => true,
            _ => false,
        }
    }
}

#[allow(dead_code)]
fn input() -> Vec<i32> {
    include_str!("../inputs/day05.txt")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn intcode_computer(nums: &Vec<i32>, input: i32) -> Option<i32> {
    let mut output: Option<i32> = None;

    let mut mut_nums = nums.clone();
    let mut ip = 0;

    while ip < nums.len() {
        let instruction = mut_nums[ip];
        // println!("reading instruction {} at index {}", instruction, ip);

        // parse opcode and modes from instruction
        let (opcode, param_mode1, param_mode2) = if instruction < 99 {
            (instruction, false, false)
        } else {
            let oc = instruction
                .to_string()
                .chars()
                .rev()
                .take(2)
                .collect::<Vec<char>>()
                .iter()
                .rev()
                .collect::<String>()
                .parse()
                .unwrap();

            // get everything but last two
            let modes = instruction
                .to_string()
                .chars()
                .take(instruction.to_string().len() - 2)
                .collect::<Vec<char>>();

            // normalize those without leading zeroes
            let modes = match modes.len() {
                0 => vec!['0'; 3],
                1 => vec!['0'; 2].into_iter().chain(modes).collect(),
                2 => vec!['0'; 1].into_iter().chain(modes).collect(),
                _ => modes,
            };

            // map values
            let (pm1, pm2) = (modes[2].to_bool(), modes[1].to_bool());

            // return values
            (oc, pm1, pm2)
        };

        if ([1, 2, 7, 8].contains(&opcode) && ip >= nums.len() - 4)
            || ([3, 4].contains(&opcode) && ip >= nums.len() - 2)
            || ([5, 6].contains(&opcode) && ip >= nums.len() - 3)
        {
            break;
        }

        // collect parameters
        let param1 = 
            // immediate
            if param_mode1 || [3, 4].contains(&opcode) || ([1, 2, 5, 6, 7, 8].contains(&opcode) && param_mode1) {
                mut_nums[ip+1]
            // position
            } else if [1, 2, 5, 6, 7, 8].contains(&opcode) && !param_mode1 {
                mut_nums[mut_nums[ip+1] as usize]
            // not needed
            } else {
                0
            };

        let param2 = 
            // immediate
            if [1, 2, 5, 6, 7, 8].contains(&opcode) && param_mode2 {
                mut_nums[ip + 2]
            // position
            } else if [1, 2, 5, 6, 7, 8].contains(&opcode) && !param_mode2 {
                mut_nums[mut_nums[ip+2] as usize]
            // not needed
            } else {
                0
            };

        let param3 = 
            // always immediate
            if [1, 2, 5, 6, 7, 8].contains(&opcode) {
                mut_nums[ip+3]
            // not needed
            } else {
                0
            };

        match opcode {
            1 => {
                mut_nums[param3 as usize] = param1 + param2;
                ip += 4;
                // println!(
                //     "adding {} and {} ({}) and placing into index {}",
                //     param1,
                //     param2,
                //     param1 + param2,
                //     param3
                // );
            }
            2 => {
                mut_nums[param3 as usize] = param1 * param2;
                ip += 4;
                // println!(
                //     "multiplying {} and {} ({}) and placing into index {}",
                //     param1,
                //     param2,
                //     param1 * param2,
                //     param3
                // );
            }
            3 => {
                mut_nums[param1 as usize] = input;
                ip += 2;
                // println!("intaking {} and placing into index {}", input, param1);
            }
            4 => {
                output = Some(mut_nums[param1 as usize]);
                ip += 2;
                // println!(
                //     "placing {} from index {} into output",
                //     mut_nums[param1 as usize], param1
                // );
            }
            99 => {
                // println!("ending program");
                break;
            }
            _ => (),
        }
        if input != 1 {
            match opcode {
                5 => {
                    if param1 != 0 {
                        ip = param2 as usize;
                    } else {
                        ip += 3;
                    }
                    // println!(
                    //     "{} is non-zero: {} - jumping to {}",
                    //     param1,
                    //     param1 != 0,
                    //     ip
                    // );
                }
                6 => {
                    if param1 == 0 {
                        ip = param2 as usize;
                    } else {
                        ip += 3;
                    }
                    // println!("{} is zero: {} - jumping to {}", param1, param1 == 0, ip);
                }
                7 => {
                    if param1 < param2 {
                        mut_nums[param3 as usize] = 1;
                    } else {
                        mut_nums[param3 as usize] = 0;
                    }
                    ip += 4;
                    // println!(
                    //     "{} is less than {}: {} - placing {} at index {}",
                    //     param1,
                    //     param2,
                    //     param1 < param2,
                    //     if param1 < param2 { 1 } else { 0 },
                    //     param3
                    // );
                }
                8 => {
                    if param1 == param2 {
                        mut_nums[param3 as usize] = 1;
                    } else {
                        mut_nums[param3 as usize] = 0;
                    }
                    ip += 4;
                    // println!(
                    //     "{} is equal to {}: {} - placing {} at index {}",
                    //     param1,
                    //     param2,
                    //     param1 == param2,
                    //     if param1 == param2 { 1 } else { 0 },
                    //     param3
                    // );
                }
                _ => (),
            }
        }
        if ip >= nums.len() {
            break;
        }
    }
    output
}

#[allow(dead_code)]
pub fn part_one() {
    println!("{:?}", intcode_computer(&input(), 1));
}

#[allow(dead_code)]
pub fn part_two() {
    println!("{:?}", intcode_computer(&input(), 5));
}
