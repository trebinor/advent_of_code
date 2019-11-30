use std::collections::HashMap;

#[aoc(day02, part1)]
pub fn solve_2a(input: &str) -> u32 {
    let lines: Vec<String> = input.lines().map(|l: &str| l.trim().to_string()).collect();
    let mut twice = 0;
    let mut thrice = 0;
    for line in lines {
        let mut letters = HashMap::new();
        for l in line.trim().chars() {
            *letters.entry(l).or_insert(0) += 1;
        }
        let mut twice_this = false;
        let mut thrice_this = false;
        for (_key, value) in letters {
            twice = if value == 2 && !twice_this {
                twice_this = true;
                twice + 1
            } else {
                twice
            };
            thrice = if value == 3 && !thrice_this {
                thrice_this = true;
                thrice + 1
            } else {
                thrice
            };
        }
    }

    twice * thrice
}

#[aoc(day02, part2)]
pub fn solve_2b(input: &str) -> String {
    let lines: Vec<String> = input.lines().map(|l: &str| l.trim().to_string()).collect();
    let mut solution = "".to_string();
    'a: for a in lines.iter() {
        'b: for b in lines.iter() {
            if a == b {
                continue;
            }
            let mut diffs = 0;
            let mut i = 0;
            for x in 0..b.len() {
                if a.as_bytes()[x] != b.as_bytes()[x] {
                    diffs += 1;
                    i = x;
                    if diffs > 1 {
                        continue 'b;
                    }
                }
            }
            if diffs <= 1 {
                solution = format!("{}{}", &a[..i], &b[i + 1..]).to_string();
            }
        }
    }

    solution
}
