use crate::icc::IntCodeComputer;

#[aoc(day09, part1, original)]
pub fn original_09a(input: &str) -> i64 {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    let mut icc = IntCodeComputer::new(v, false);
    icc.program.resize(1024 * 1024, 0);
    icc.inputq.push_back(1);
    icc.execute();
    icc.consume_output().parse().unwrap()
}

#[aoc(day09, part2, original)]
pub fn original_09b(input: &str) -> i64 {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    let mut icc = IntCodeComputer::new(v, true);
    icc.program.resize(1024 * 1024, 0);
    icc.inputq.push_back(2);
    icc.execute();
    icc.consume_output().parse().unwrap()
}
