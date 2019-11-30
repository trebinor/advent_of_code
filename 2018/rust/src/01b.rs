use std::collections::HashSet;
use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let mut freq_sum = 0;
    let mut freqs = HashSet::new();
    let lines: String = fs::read_to_string("../../inputs/01").expect("Failed to read line");
    let lines: Vec<String> = lines
        .split_whitespace()
        .map(|s: &str| s.to_string())
        .collect();
    'outer: loop {
        for line in &lines {
            let freq: i32 = line.trim().parse().expect("Failed to parse number");
            freq_sum += freq;
            if freqs.contains(&freq_sum) {
                break 'outer;
            } else {
                freqs.insert(freq_sum);
            }
        }
    }

    println!("{}", freq_sum);

    Ok(())
}
