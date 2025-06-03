#[allow(dead_code)]
fn input() -> Vec<(Vec<i64>, i64)> {
    include_str!("../inputs/day07.txt")
        .lines()
        .fold(Vec::new(), |mut equations, line| {
            let mut parts = line.split(':');
            let target = parts.next().unwrap().parse().unwrap();
            let numbers = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect();
            equations.push((numbers, target));
            equations
        })
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

#[allow(dead_code)]
fn numbers_to_operators(nums: &Vec<usize>) -> Vec<Operator> {
    nums.iter()
        .map(|char| match char {
            0 => Operator::Add,
            1 => Operator::Multiply,
            2 => Operator::Concatenate,
            _ => panic!("Error converting {char} to operator"),
        })
        .collect()
}

#[allow(dead_code)]
fn do_operation(num1: i64, num2: i64, operator: Operator) -> i64 {
    match operator {
        Operator::Add => num1 + num2,
        Operator::Multiply => num1 * num2,
        Operator::Concatenate => (num1.to_string() + num2.to_string().as_str())
            .parse()
            .unwrap(),
    }
}

// Kept for reference: Shows binary pattern representation
// Note: numbers_to_operators() has been updated and is now incompatible here
//
// #[allow(dead_code)]
// fn operator_combinations_binary_method(numbers: &Vec<i64>) -> Vec<Vec<Operator>> {
//     let position_count = numbers.len() - 1;
//     let combination_count = 2_i32.pow(position_count as u32);
//     let operator_combinations = (0..combination_count)
//         .map(|i| numbers_to_operators(format!("{i:0position_count$b}")))
//         .collect();
//     operator_combinations
// }

#[allow(dead_code)]
fn all_combinations(
    position_count: usize,
    element_count: usize,
    current: &mut Vec<usize>,
) -> Vec<Vec<usize>> {
    let mut combinations: Vec<Vec<usize>> = Vec::new();

    if position_count == 0 {
        combinations.push(current.clone());
        return combinations;
    }

    for i in 0..element_count {
        current.push(i);
        combinations.extend(all_combinations(position_count - 1, element_count, current));
        current.remove(current.len() - 1);
    }

    combinations
}

#[allow(dead_code)]
fn operator_combinations(numbers: &Vec<i64>, operators: &Vec<Operator>) -> Vec<Vec<Operator>> {
    let position_count = numbers.len() - 1;
    let operator_count = operators.len();

    all_combinations(position_count, operator_count, &mut vec![])
        .iter()
        .map(|combination| numbers_to_operators(combination))
        .collect()
}

#[allow(dead_code)]
fn numbers_reach_target(numbers: &Vec<i64>, target: &i64, operators: &Vec<Operator>) -> bool {
    let operator_combinations = operator_combinations(&numbers, operators);
    let mut result = false;

    for combination in operator_combinations {
        let mut total: i64 = numbers[0];
        // We skip the first number since it's already in total
        // Then we zip the remaining numbers with operators to evaluate them in sequence
        // For example, with numbers [1,2,3,4] and operators [*,+,||]:
        //
        // total = 1
        // total = 1 * 2  (first operation)
        // total = 2 + 3  (second operation)
        // total = 5 || 4 (third operation)
        for (number, operator) in numbers.iter().skip(1).zip(combination) {
            total = do_operation(total, *number, operator)
        }

        if total == *target {
            result = true;
            break;
        }
    }
    result
}

#[allow(dead_code)]
pub fn part_one() {
    let operators = vec![Operator::Add, Operator::Multiply];
    let equations = input();
    let mut total: i64 = 0;
    for (numbers, target) in equations.iter() {
        if numbers_reach_target(numbers, target, &operators) {
            total += target;
        }
    }
    println!("total: {total}");
}
#[allow(dead_code)]
pub fn part_two() {
    let operators = vec![Operator::Add, Operator::Multiply, Operator::Concatenate];
    let equations = input();
    let mut total: i64 = 0;
    for (numbers, target) in equations.iter() {
        if numbers_reach_target(numbers, target, &operators) {
            total += target;
        }
    }
    println!("total: {total}");
}
