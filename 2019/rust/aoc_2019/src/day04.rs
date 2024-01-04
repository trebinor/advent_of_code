#[aoc(day04, part1, original)]
pub fn original_04a(_input: &str) -> u32 {
    let min = 278_384;
    let max = 824_795;
    let mut count = 0;
    for i in min..=max {
        if is_a_double_digit(i) && is_monotonically_increasing(i) {
            count += 1;
        }
    }
    count
}

#[aoc(day04, part1, parsed)]
pub fn parsed_04a(input: &str) -> u32 {
    let v: Vec<i32> = input
        .trim()
        .splitn(2, '-')
        .map(|m| m.parse::<i32>().unwrap())
        .collect();
    let mut count = 0;
    (v[0]..=v[1]).for_each(|i: i32| {
        if is_a_double_digit(i) && is_monotonically_increasing(i) {
            count += 1
        }
    });
    count
}

#[aoc(day04, part2, original)]
pub fn original_04b(_input: &str) -> u32 {
    let min = 278_384;
    let max = 824_795;
    let mut count = 0;
    for i in min..=max {
        if is_a_double_digit_strict(i) && is_monotonically_increasing(i) {
            count += 1;
        }
    }
    count
}

#[aoc(day04, part2, parsed)]
pub fn parsed_04b(input: &str) -> u32 {
    let v: Vec<i32> = input
        .trim()
        .splitn(2, '-')
        .map(|m| m.parse::<i32>().unwrap())
        .collect();
    let mut count = 0;
    (v[0]..=v[1]).for_each(|i: i32| {
        if is_a_double_digit_strict(i) && is_monotonically_increasing(i) {
            count += 1
        }
    });
    count
}

fn is_a_double_digit(i: i32) -> bool {
    let s: Vec<u32> = i
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    for d in 0..5 {
        if s[d] == s[d + 1] {
            return true;
        }
    }
    false
}

fn is_monotonically_increasing(i: i32) -> bool {
    let s: Vec<u32> = i
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    for d in 0..5 {
        if s[d] <= s[d + 1] {
            continue;
        } else {
            return false;
        }
    }
    true
}

fn is_a_double_digit_strict(i: i32) -> bool {
    let s: Vec<u32> = i
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    for d in 0..5 {
        if s[d] == s[d + 1] {
            if d == 0 && s[d] != s[d + 2] {
                return true;
            } else if d == 4 && s[d - 1] != s[d] {
                return true;
            } else if (d == 1 || d == 2 || d == 3) && (s[d - 1] != s[d] && s[d + 2] != s[d]) {
                return true;
            }
        }
    }
    false
}
