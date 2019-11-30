use icc::IntCodeComputer;

#[aoc(day09, part1, original)]
pub fn original_9a(input: &str) -> i64 {
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
pub fn original_9b(input: &str) -> i64 {
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

#[cfg(test)]
mod tests {
    use day09::original_9a;
    use day09::original_9b;
    use std::fs;
    const ANSWER_9A: i64 = 2_932_210_790;
    const ANSWER_9B: i64 = 73144;

    #[test]
    fn t09a() {
        assert_eq!(
            ANSWER_9A,
            original_9a(&fs::read_to_string("input/2019/day9.txt").unwrap().trim())
        );
    }
    #[test]
    fn t09b() {
        assert_eq!(
            ANSWER_9B,
            original_9b(&fs::read_to_string("input/2019/day9.txt").unwrap().trim())
        );
    }
}
