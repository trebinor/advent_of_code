use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), io::Error> {
    let mut freq_sum = 0;
    let f = File::open("../../inputs/01")?;
    let f = BufReader::new(f);
    for line in f.lines() {
        let freq: i32 = line
            .expect("Failed to read line")
            .trim()
            .parse()
            .expect("Failed to parse number");
        freq_sum += freq;
    }

    println!("{}", freq_sum);

    Ok(())
}
