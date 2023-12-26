#[aoc(day16, part1)]
pub fn solution_16a(input: &str) -> String {
    let s: Vec<i32> = input
        .trim()
        .chars()
        .map(|n| n.to_string().parse::<i32>().unwrap())
        .collect();
    let base_repeat: Vec<i32> = vec![0, 1, 0, -1];
    let mut previous_signal: Vec<i32> = s.clone();
    let mut signal: Vec<i32> = s.clone();
    for _phases in 1..=100 {
        for i in 0..s.len() {
            let mut v: Vec<i32> = Vec::new();
            let mut j = 0;
            let mut first_value_skipped: bool = false;
            'repeat: loop {
                for _k in 0..=i {
                    if !first_value_skipped {
                        first_value_skipped = true;
                        continue;
                    }
                    v.push(base_repeat[j % 4]);
                    if v.len() == s.len() {
                        break 'repeat;
                    }
                }
                j += 1;
            }
            let mut sum: i32 = 0;
            for x in 0..signal.len() {
                sum += previous_signal[x] * v[x]
            }
            signal[i] = sum.abs() % 10;
        }
        previous_signal = signal.clone();
    }
    signal[0..8].iter().map(|n| n.to_string()).collect()
}

#[aoc(day16, part2)]
pub fn solution_16b(input: &str) -> String {
    let mut s: Vec<u32> = input
        .trim()
        .chars()
        .map(|n| n.to_string().parse::<u32>().unwrap())
        .collect();
    let clone = s.clone();
    for _repeat in 1..10_000 {
        s.extend(&clone);
    }
    for _phases in 1..=100 {
        // Cumulative sum approach starting at the end and summing in reverse. I did not discover this on my own.
        let mut i = s.len() - 2;
        while i != 0 {
            s[i] = (s[i] + s[i + 1]) % 10;
            i -= 1;
        }
    }
    let skip_digits = input[..7].parse::<usize>().unwrap();
    s[(skip_digits as usize)..(skip_digits as usize + 8)]
        .iter()
        .map(|n| n.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day16::solution_16a;
    use crate::day16::solution_16b;
    use std::fs;
    const ANSWER_16A: &str = "27831665";
    const ANSWER_16B: &str = "36265589";

    #[test]
    fn t16a() {
        assert_eq!(
            ANSWER_16A,
            solution_16a(&fs::read_to_string("input/2019/day16.txt").unwrap().trim())
        );
    }
    #[test]
    fn t16b() {
        assert_eq!(
            ANSWER_16B,
            solution_16b(&fs::read_to_string("input/2019/day16.txt").unwrap().trim())
        );
    }
}
