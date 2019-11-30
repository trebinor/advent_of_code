use std::fs;
use std::io;

const INPUT: &str = "../../inputs/02";

fn main() -> Result<(), io::Error> {
    let lines: Vec<String> = fs::read_to_string(INPUT)
        .expect("Failed to read line")
        .split_whitespace()
        .map(|s: &str| s.to_string())
        .collect();

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
                println!("{}{}", &a[..i], &b[i + 1..]);
                break 'a;
            }
        }
    }

    Ok(())
}
