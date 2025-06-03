#[allow(dead_code)]
fn fuel_for(mass: isize, consider_added_mass: bool) -> isize {
    if !consider_added_mass {
        (mass / 3) - 2
    } else {
        recur_fuel_for(mass, 0)
    }
}

#[allow(dead_code)]
fn recur_fuel_for(mass: isize, total: isize) -> isize {
    let result = fuel_for(mass, false);
    if result <= 0 {
        return total;
    }
    return recur_fuel_for(result, total + result);
}

#[allow(dead_code)]
pub fn part_one() {
    println!(
        "{}",
        include_str!("../inputs/day01.txt")
            .lines()
            .map(|x| fuel_for(x.parse::<isize>().unwrap(), false))
            .sum::<isize>()
    );
}

#[allow(dead_code)]
pub fn part_two() {
    println!(
        "{}",
        include_str!("../inputs/day01.txt")
            .lines()
            .map(|x| fuel_for(x.parse::<isize>().unwrap(), true))
            .sum::<isize>()
    );
}
