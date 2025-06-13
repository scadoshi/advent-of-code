use std::collections::{HashMap, HashSet, VecDeque};

fn rules() -> HashMap<char, HashSet<char>> {
    include_str!("../inputs/day07.txt")
        .lines()
        .fold(HashMap::new(), |mut map, line| {
            let req = line[5..6].chars().next().unwrap();
            let step = line[36..37].chars().next().unwrap();
            map.entry(step).or_default().insert(req);
            map
        })
}

fn steps() -> Vec<char> {
    let mut steps =
        include_str!("../inputs/day07.txt")
            .lines()
            .fold(Vec::new(), |mut steps, line| {
                let req = line[5..6].chars().next().unwrap();
                steps.push(req);
                let step = line[36..37].chars().next().unwrap();
                steps.push(step);
                steps
            });
    steps.sort();
    steps.dedup();
    steps
}

fn proper_sequence(rules: &HashMap<char, HashSet<char>>, steps: &Vec<char>) -> String {
    let mut steps = steps.clone();
    let mut p = 0;
    let mut completed = String::new();
    while !steps.is_empty() {
        let step = steps[p];

        if !rules.contains_key(&step)
            || rules
                .get(&step)
                .unwrap()
                .iter()
                .all(|x| completed.chars().any(|c| c == *x))
        {
            completed.push(steps.remove(p));
            p = 0;
            continue;
        }
        p += 1;
    }
    completed
}

#[allow(dead_code)]
pub fn part_one() {
    let start = std::time::Instant::now();
    println!("part_one={:?}", proper_sequence(&rules(), &steps()));
    println!("runtime={:?}", start.elapsed());
}

trait WorkLoad {
    fn workload(&self) -> usize;
}
impl WorkLoad for char {
    fn workload(&self) -> usize {
        (self.to_ascii_lowercase() as isize - 'a' as isize) as usize + 61
    }
}

#[allow(dead_code)]
pub fn part_two() {
    let start = std::time::Instant::now();

    let rules = rules();
    let mut steps = steps();
    // println!("steps={:?}", steps.iter().collect::<String>());

    let worker_max = 5;
    // char is character being worked on
    // usize is time to work on it
    let mut workers: HashMap<char, usize> = HashMap::new();
    // queue for steps waiting to be worked
    let mut ready: VecDeque<char> = VecDeque::new();

    let mut completed = String::new();
    let mut time = 0;

    // println!("{}time=0", "=".repeat(100));
    while !steps.is_empty() || !workers.is_empty() {
        // same rules as part one
        ready.extend(
            steps
                .iter()
                .filter(|s| {
                    !rules.contains_key(&s)
                        || rules
                            .get(&s)
                            .unwrap()
                            .iter()
                            .all(|x| completed.chars().any(|c| c == *x))
                })
                .map(|s| s.to_owned()),
        );
        // println!("ready={:?}", ready.iter().collect::<String>());

        steps.retain(|s| !ready.contains(&s));
        // println!("steps={:?}", steps.iter().collect::<String>());

        // so long as we have enough workers
        while workers.len() < worker_max {
            // and something is ready
            if let Some(next) = ready.pop_front() {
                // println!("f");
                // println!("adding {:?}", next);
                workers.insert(next, next.workload());
            } else {
                break;
            }
        }

        // decrement each time left value
        // if they reach 0 then we add their char to completed
        // and remove them from the work queue
        time += 1;
        workers = workers
            .into_iter()
            .filter_map(|(c, t)| {
                if t - 1 == 0 {
                    // println!("completed {}", c);
                    completed.push(c);
                    None
                } else {
                    Some((c, t - 1))
                }
            })
            .collect();

        // println!("{}time={}", "=".repeat(100), time);
        // println!("workers={:?}", workers);
        // println!("completed={}", completed);
    }

    println!("part_two={:?}", time);
    println!("runtime={:?}", start.elapsed());
}
