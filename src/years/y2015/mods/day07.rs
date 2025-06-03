use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Instruction {
    source1: Option<String>,
    source2: Option<String>,
    operation: Option<String>,
    circuit: String,
}

impl Instruction {
    fn new(instruction: &str) -> Self {
        let mut keys: Vec<String> = instruction
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();
        let operation = if keys
            .iter()
            .any(|key| ["AND", "OR", "NOT", "LSHIFT", "RSHIFT"].contains(&key.as_str()))
        {
            Some(
                keys.remove(
                    keys.iter()
                        .position(|x| {
                            ["AND", "OR", "NOT", "LSHIFT", "RSHIFT"].contains(&x.as_str())
                        })
                        .expect(
                            format!("failed to find operator in instruction: {}", instruction)
                                .as_str(),
                        ),
                ),
            )
        } else {
            None
        };

        let source1 = Some(keys.remove(0));
        let source2 = {
            match keys[0].as_ref() {
                "->" => None,
                _ => Some(keys.remove(0)),
            }
        };
        let circuit = keys.remove(keys.len() - 1);
        Self {
            source1,
            source2,
            operation,
            circuit,
        }
    }
}

#[derive(Debug)]
struct Instructions {
    list: Vec<Instruction>,
}

impl Instructions {
    fn new(instructions_str: &str) -> Self {
        let list = instructions_str
            .lines()
            .map(|line| Instruction::new(line))
            .collect::<Vec<Instruction>>();

        Self { list }
    }
}

#[allow(dead_code)]
fn get_signal(
    cache: &mut HashMap<String, i32>,
    all_instructions: &Instructions,
    instruction_to_run: &Instruction,
) -> Option<i32> {
    // check cache
    if let Some(value) = cache.get(&instruction_to_run.circuit) {
        return Some(value.clone());
    }

    // check source1
    let value1 = if let Some(source1) = &instruction_to_run.source1 {
        if let Ok(number) = source1.parse::<i32>() {
            Some(number)
        } else {
            get_signal(
                cache,
                all_instructions,
                all_instructions
                    .list
                    .iter()
                    .find(|x| x.circuit == *source1)?,
            )
        }
    } else {
        None
    };

    // check source2
    let value2 = if let Some(source2) = &instruction_to_run.source2 {
        if let Ok(num) = source2.parse::<i32>() {
            Some(num)
        } else {
            get_signal(
                cache,
                all_instructions,
                all_instructions
                    .list
                    .iter()
                    .find(|x| x.circuit == *source2)?,
            )
        }
    } else {
        None
    };

    // operation
    let result = match &instruction_to_run.operation {
        Some(operation) => match operation.as_str() {
            "AND" => Some(value1.unwrap() & value2.unwrap()),
            "OR" => Some(value1.unwrap() | value2.unwrap()),
            "NOT" => Some(!value1.unwrap()),
            "LSHIFT" => Some(value1.unwrap() << value2.unwrap()),
            "RSHIFT" => Some(value1.unwrap() >> value2.unwrap()),
            _ => None,
        },
        None => value1,
    };

    cache.insert(instruction_to_run.circuit.clone(), result.clone().unwrap());

    result
}

#[allow(dead_code)]
pub fn part_one() {
    let instructions = Instructions::new(include_str!("../inputs/day07.txt"));

    let mut cache: HashMap<String, i32> = HashMap::new();

    let result = get_signal(
        &mut cache,
        &instructions,
        instructions.list.iter().find(|x| x.circuit == "a").unwrap(),
    )
    .unwrap();

    println!("{}", result);
}

#[allow(dead_code)]
pub fn part_two() {
    let mut instructions = Instructions::new(include_str!("..//inputs//day07.txt"));

    let mut cache: HashMap<String, i32> = HashMap::new();

    let first_result = get_signal(
        &mut cache,
        &instructions,
        instructions.list.iter().find(|x| x.circuit == "a").unwrap(),
    )
    .unwrap();

    if let Some(instruction) = instructions.list.iter_mut().find(|x| x.circuit == "b") {
        instruction.source1 = Some(first_result.to_string());
    }

    cache.clear();

    let second_result = get_signal(
        &mut cache,
        &instructions,
        instructions.list.iter().find(|x| x.circuit == "a").unwrap(),
    )
    .unwrap();

    println!("{}", second_result);
}
