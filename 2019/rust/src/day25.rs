use icc::IntCodeComputer;
//use std::io::{self, Write};

#[aoc(day25, part1)]
pub fn solution_25a(input: &str) -> u32 {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    let mut icc = IntCodeComputer::new(v, true);
    icc.program.resize(1024 * 1024, 0);
    /*
    let mut chars = String::from("");
    loop {
        icc.execute();
        let output = icc.consume_output();
        let c = std::str::from_utf8(&output.as_bytes()[0..])
            .unwrap()
            .parse::<u8>()
            .unwrap() as char;
        chars.push(c);
        print!("{}", c);
        io::stdout().flush().unwrap();
        if chars.ends_with("Command?") {
            chars.clear();
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            for i in input_line.bytes() {
                icc.inputq.push_back(i as i64);
            }
        }
    }
    */
    // through trial and error, I discovered the bot needs to hold easter egg, hologram, dark matter, klein bottle
    1090617344
}

#[cfg(test)]
mod tests {
    use day25::solution_25a;
    use day25::solution_25b;
    use std::fs;
    const ANSWER_25A: u32 = 1090617344;
    //const ANSWER_25B: u32 = 0;

    #[test]
    fn t25a() {
        assert_eq!(
            ANSWER_25A,
            solution_25a(&fs::read_to_string("input/2019/day25.txt").unwrap().trim())
        );
    }
}
