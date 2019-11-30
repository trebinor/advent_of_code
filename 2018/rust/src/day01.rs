use std::collections::HashSet;

#[aoc(day01, part1)]
pub fn solve_1a(input: &str) -> i32 {
    input
        .lines()
        .map(|l| l.trim().parse::<i32>().unwrap())
        .sum()
}

#[aoc(day01, part2)]
pub fn solve_1b(input: &str) -> i32 {
    let mut freq_sum = 0;
    let mut freqs = HashSet::new();
    'outer: loop {
        for freq in input.lines().map(|l| l.trim().parse::<i32>().unwrap()) {
            freq_sum += freq;
            if freqs.contains(&freq_sum) {
                break 'outer;
            } else {
                freqs.insert(freq_sum);
            }
        }
    }

    freq_sum
}
