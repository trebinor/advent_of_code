use std::collections::HashMap;
use std::fs;
use std::io;

const INPUT: &str = "../../inputs/02";

fn main() -> Result<(), io::Error> {
    let lines: Vec<String> = fs::read_to_string(INPUT)
        .expect("Failed to read line")
        .split_whitespace()
        .map(|s: &str| s.to_string())
        .collect();
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

    println!("{}", twice * thrice);
    Ok(())
}
