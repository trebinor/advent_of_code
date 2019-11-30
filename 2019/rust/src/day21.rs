use icc::IntCodeComputer;
use itertools::Itertools;

fn deploy(springscript: &str, icc_program: Vec<i64>) -> String {
    let mut icc = IntCodeComputer::new(icc_program, false);
    icc.program.resize(1024 * 1024, 0);
    for c in springscript.chars() {
        icc.inputq.push_back(c as i64);
    }
    icc.execute();
    icc.consume_output()
}

fn parse_successful_output(output: &str) -> u32 {
    output
        .split("1010")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap()
}

#[allow(dead_code)]
fn find_working_springscript(icc_program: Vec<i64>, commands: Vec<&str>, movement: &str) -> u32 {
    let mut p = 1;
    let mut output: String;
    'outer: loop {
        for command_set in commands.iter().permutations(p) {
            let mut program: String = "".to_string();
            for command in command_set {
                program.push_str(command);
                program.push('\n');
            }
            program.push_str(movement);
            program.push('\n');
            output = deploy(&program, icc_program.clone());
            let output_bytes = output.as_bytes();
            let mut i = 1;
            let mut dot_encountered = false; // skip garbage output at the beginning
            while i < output_bytes.len() {
                if i + 1 >= output_bytes.len() {
                    println!("Based on a parsing error, maybe the answer is {}", output);
                    break 'outer;
                }
                let c = std::str::from_utf8(&output_bytes[i..i + 2])
                    .unwrap()
                    .parse::<u8>()
                    .unwrap() as char;
                if dot_encountered || c == '.' {
                    dot_encountered = true;
                }
                i += 2;
            }
            if !dot_encountered {
                println!("Based on no printout, maybe the answer is {}", output);
                break 'outer;
            }
        }
        p += 1;
    }
    parse_successful_output(&output)
}

#[aoc(day21, part1)]
pub fn solution_21a(input: &str) -> u32 {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    // I'm not playing this game when I have a computer to do it for me.
    let _commands = vec![
        "AND A J", "AND B J", "AND C J", "AND D J", "AND A T", "AND B T", "AND C T", "AND D T",
        "AND J T", "AND T J", "AND J J", "AND T T", "OR A J", "OR B J", "OR C J", "OR D J",
        "OR A T", "OR B T", "OR C T", "OR D T", "OR J T", "OR T J", "OR J J", "OR T T", "NOT A J",
        "NOT B J", "NOT C J", "NOT D J", "NOT A T", "NOT B T", "NOT C T", "NOT D T", "NOT J T",
        "NOT T J", "NOT J J", "NOT T T",
    ];
    //find_working_springscript(v.clone(), _commands, "WALK");
    // found by iterating over the permutations with find_working_springscript()
    let program = r"OR A T
                    AND C T
                    NOT T J
                    AND D J
                    WALK
                   ";
    parse_successful_output(&deploy(&program, v.clone()))
}

#[aoc(day21, part2)]
pub fn solution_21b(input: &str) -> u32 {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    let _commands = vec![
        "AND A J", "AND B J", "AND C J", "AND D J", "AND E J", "AND F J", "AND G J", "AND H J",
        "AND I J", "AND A T", "AND B T", "AND C T", "AND D T", "AND E T", "AND F T", "AND G T",
        "AND H T", "AND I T", "AND J T", "AND T J", "AND J J", "AND T T", "OR A J", "OR B J",
        "OR C J", "OR D J", "OR E J", "OR F J", "OR G J", "OR H J", "OR I J", "OR A T", "OR B T",
        "OR C T", "OR D T", "OR E T", "OR F T", "OR G T", "OR H T", "OR I T", "OR J T", "OR T J",
        "OR J J", "OR T T", "NOT A J", "NOT B J", "NOT C J", "NOT D J", "NOT E J", "NOT F J",
        "NOT G J", "NOT H J", "NOT I J", "NOT A T", "NOT B T", "NOT C T", "NOT D T", "NOT E T",
        "NOT F T", "NOT G T", "NOT H T", "NOT I T", "NOT J T", "NOT T J", "NOT J J", "NOT T T",
    ];
    //find_working_springscript(v.clone(), _commands, "RUN");
    // Found one!
    let program = r"NOT A T
                AND D T
                NOT B J
                AND H J
                AND D J
                OR T J
                NOT C T
                AND E T
                AND D T
                OR T J
                NOT C T
                AND H T
                AND D T
                OR T J
                RUN
               ";
    parse_successful_output(&deploy(&program, v.clone()))
}

#[cfg(test)]
mod tests {
    use day21::solution_21a;
    use day21::solution_21b;
    use std::fs;
    const ANSWER_21A: u32 = 19_349_530;
    const ANSWER_21B: u32 = 1_142_805_439;

    #[test]
    fn t21a() {
        assert_eq!(
            ANSWER_21A,
            solution_21a(&fs::read_to_string("input/2019/day21.txt").unwrap().trim())
        );
    }
    #[test]
    fn t21b() {
        assert_eq!(
            ANSWER_21B,
            solution_21b(&fs::read_to_string("input/2019/day21.txt").unwrap().trim())
        );
    }
}
