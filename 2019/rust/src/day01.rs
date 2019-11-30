use std::cmp;

#[aoc(day01, part1, original)]
pub fn original_1a(input: &str) -> i32 {
    input
        .lines()
        .map(|l| l.trim().parse::<f32>().unwrap())
        .map(|f| ((f / 3.0) as i32) - 2)
        .sum()
}

#[aoc(day01, part2, original)]
pub fn original_1b(input: &str) -> i32 {
    let modules = input.lines().map(|l| l.trim().parse::<i32>().unwrap());
    let mut fuel: i32 = 0;
    for fuels in modules {
        let mut module_fuel = original_fuel_mass_compound(fuels);
        while module_fuel != 0 {
            fuel += module_fuel;
            module_fuel = original_fuel_mass_compound(module_fuel);
        }
    }
    fuel
}

fn revised_fuel_mass_simple(module: u32) -> u32 {
    (module / 3.0 as u32) - 2
}

fn original_fuel_mass_compound(module: i32) -> i32 {
    cmp::max(0, module / 3.0 as i32 - 2)
}

fn revised_fuel_mass_compound(mass: u32) -> u32 {
    match mass {
        0 => 0,
        _ => mass + revised_fuel_mass_compound(cmp::max(0, (mass as i32 / 3.0 as i32) - 2) as u32),
    }
}

#[aoc(day01, part1, revised)]
pub fn revised_1a(input: &str) -> u32 {
    input
        .lines()
        .map(|l| revised_fuel_mass_simple(l.trim().parse::<u32>().unwrap()))
        .sum()
}

#[aoc(day01, part2, revised)]
pub fn revised_1b(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mass = l.trim().parse::<u32>().unwrap();
            revised_fuel_mass_compound(mass) - mass
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use day01::original_1a;
    use day01::original_1b;
    use std::fs;
    const ANSWER_1A: i32 = 3_369_286;
    const ANSWER_1B: i32 = 5_051_054;

    #[test]
    fn t01a() {
        assert_eq!(
            ANSWER_1A,
            original_1a(&fs::read_to_string("input/2019/day1.txt").unwrap().trim())
        );
    }
    #[test]
    fn t01b() {
        assert_eq!(
            ANSWER_1B,
            original_1b(&fs::read_to_string("input/2019/day1.txt").unwrap().trim())
        );
    }
}
