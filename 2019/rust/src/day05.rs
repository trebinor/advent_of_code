use icc::IntCodeComputer;

#[aoc(day05, part1, original)]
pub fn original_5a(input: &str) -> String {
    let mut v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    compute_5a(&mut v)
}

fn compute_5a(v: &mut Vec<i64>) -> String {
    let mut pc: usize = 0;
    const INPUT: i64 = 1;
    let mut output: String = "".to_string();
    loop {
        let s = format!("{:0>5}", v[pc].to_string());
        let mut c = s.chars();
        let _p2 = c.next();
        let p1 = match c.next().unwrap() {
            '0' => ParameterMode::Position,
            '1' => ParameterMode::Immediate,
            _ => unreachable!(),
        };
        let p0 = match c.next().unwrap() {
            '0' => ParameterMode::Position,
            '1' => ParameterMode::Immediate,
            _ => unreachable!(),
        };
        let operation = c.take(2).collect::<String>().parse::<i64>().unwrap();
        match operation {
            1 => add(v, &mut pc, p0, p1),
            2 => mul(v, &mut pc, p0, p1),
            3 => store(v, INPUT, &mut pc, p0),
            4 => output.push_str(&show(v, &mut pc, p0)),
            99 => break,
            _ => panic!(),
        }
    }
    output
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

fn add(v: &mut Vec<i64>, pc: &mut usize, p0: ParameterMode, p1: ParameterMode) {
    let ia0 = v[*pc + 1];
    let ia1 = v[*pc + 2];
    let oa = v[*pc + 3];
    let a0 = match p0 {
        ParameterMode::Position => v[ia0 as usize],
        ParameterMode::Immediate => ia0,
    };
    let a1 = match p1 {
        ParameterMode::Position => v[ia1 as usize],
        ParameterMode::Immediate => ia1,
    };
    v[oa as usize] = a0 + a1;
    *pc += 4;
}

fn mul(v: &mut Vec<i64>, pc: &mut usize, p0: ParameterMode, p1: ParameterMode) {
    let ia0 = v[*pc + 1];
    let ia1 = v[*pc + 2];
    let oa = v[*pc + 3];
    let a0 = match p0 {
        ParameterMode::Position => v[ia0 as usize],
        ParameterMode::Immediate => ia0,
    };
    let a1 = match p1 {
        ParameterMode::Position => v[ia1 as usize],
        ParameterMode::Immediate => ia1,
    };
    v[oa as usize] = a0 * a1;
    *pc += 4;
}

fn store(v: &mut Vec<i64>, input: i64, pc: &mut usize, p0: ParameterMode) {
    let is0 = v[*pc + 1];
    let _s0 = match p0 {
        ParameterMode::Position => v[is0 as usize],
        ParameterMode::Immediate => is0,
    };
    v[is0 as usize] = input;
    *pc += 2;
}

fn show(v: &mut Vec<i64>, pc: &mut usize, p0: ParameterMode) -> String {
    let is0 = v[*pc + 1];
    let s0 = match p0 {
        ParameterMode::Position => v[is0 as usize],
        ParameterMode::Immediate => is0,
    };
    *pc += 2;
    s0.to_string()
}

fn operations_5_to_8(
    v: &mut Vec<i64>,
    pc: &mut usize,
    p0: ParameterMode,
    p1: ParameterMode,
    o: Operation,
) {
    let ip0 = v[*pc + 1];
    let ip1 = v[*pc + 2];
    let oa = v[*pc + 3];
    let o0 = match p0 {
        ParameterMode::Position => v[ip0 as usize],
        ParameterMode::Immediate => ip0,
    };
    let o1 = match p1 {
        ParameterMode::Position => v[ip1 as usize],
        ParameterMode::Immediate => ip1,
    };
    if o == Operation::JumpIfTrue {
        if o0 != 0 {
            *pc = o1 as usize;
        } else {
            *pc += 3;
        }
    } else if o == Operation::JumpIfFalse {
        if o0 == 0 {
            *pc = o1 as usize;
        } else {
            *pc += 3;
        }
    } else if o == Operation::LessThan {
        v[oa as usize] = if o0 < o1 { 1 } else { 0 };
        *pc += 4;
    } else if o == Operation::Equals {
        v[oa as usize] = if o0 == o1 { 1 } else { 0 };
        *pc += 4;
    }
}

#[aoc(day05, part2, original)]
pub fn original_5b(input: &str) -> String {
    let mut v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    compute_5b(&mut v)
}

#[derive(PartialEq, PartialOrd)]
enum Operation {
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
}

fn compute_5b(v: &mut Vec<i64>) -> String {
    let mut pc: usize = 0;
    const INPUT: i64 = 5;
    let mut output: String = "".to_string();
    loop {
        let s = format!("{:0>5}", v[pc].to_string());
        let mut c = s.chars();
        let _p2 = c.next();
        let p1 = match c.next().unwrap() {
            '0' => ParameterMode::Position,
            '1' => ParameterMode::Immediate,
            _ => unreachable!(),
        };
        let p0 = match c.next().unwrap() {
            '0' => ParameterMode::Position,
            '1' => ParameterMode::Immediate,
            _ => unreachable!(),
        };
        let operation = c.take(2).collect::<String>().parse::<i64>().unwrap();
        match operation {
            1 => add(v, &mut pc, p0, p1),
            2 => mul(v, &mut pc, p0, p1),
            3 => store(v, INPUT, &mut pc, p0),
            4 => output.push_str(&show(v, &mut pc, p0)),
            5 => operations_5_to_8(v, &mut pc, p0, p1, Operation::JumpIfTrue),
            6 => operations_5_to_8(v, &mut pc, p0, p1, Operation::JumpIfFalse),
            7 => operations_5_to_8(v, &mut pc, p0, p1, Operation::LessThan),
            8 => operations_5_to_8(v, &mut pc, p0, p1, Operation::Equals),
            99 => break,
            _ => panic!(),
        }
    }
    output
}

#[aoc(day05, part1, icc)]
pub fn icc_5a(input: &str) -> String {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    let mut icc = IntCodeComputer::new(v, true);
    icc.inputq.push_back(1);
    loop {
        icc.execute();
        let output = icc.consume_output();
        icc.execute_one();
        if icc.terminated {
            break output;
        } else {
            assert_eq!(output, "0");
        }
    }
}

#[aoc(day05, part2, icc)]
pub fn icc_5b(input: &str) -> String {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    let mut icc = IntCodeComputer::new(v, false);
    icc.inputq.push_back(5);
    icc.execute();
    icc.consume_output()
}

#[cfg(test)]
mod tests {
    use day05::icc_5a;
    use day05::icc_5b;
    use day05::original_5a;
    use day05::original_5b;
    use std::fs;
    const ANSWER_5A: &str = "16489636";
    const ANSWER_5B: &str = "9386583";

    #[test]
    fn t05a() {
        assert!(
            original_5a(&fs::read_to_string("input/2019/day5.txt").unwrap().trim())
                .ends_with(ANSWER_5A)
        );
    }
    #[test]
    fn t05b() {
        assert!(
            original_5b(&fs::read_to_string("input/2019/day5.txt").unwrap().trim())
                .ends_with(ANSWER_5B)
        );
    }

    #[test]
    fn t05a_icc() {
        assert_eq!(
            ANSWER_5A,
            icc_5a(&fs::read_to_string("input/2019/day5.txt").unwrap().trim())
        );
    }
    #[test]
    fn t05b_icc() {
        assert!(
            icc_5b(&fs::read_to_string("input/2019/day5.txt").unwrap().trim()).ends_with(ANSWER_5B)
        );
    }
}
