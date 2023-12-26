#[aoc(day02, part1, original)]
pub fn original_2a(input: &str) -> u32 {
    let mut v: Vec<u32> = input
        .trim()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    v[1] = 12;
    v[2] = 2;
    compute(&mut v)
}

fn compute(mut v: &mut Vec<u32>) -> u32 {
    let mut i: usize = 0;
    loop {
        match v[i] {
            1 => do_opcode_1(&mut v, i),
            2 => do_opcode_2(&mut v, i),
            99 => break,
            _ => panic!(),
        }
        i += 4;
    }
    v[0]
}

fn do_opcode_1(v: &mut Vec<u32>, i: usize) {
    let ia1: usize = v[i + 1] as usize;
    let ia2: usize = v[i + 2] as usize;
    let oa: usize = v[i + 3] as usize;
    v[oa] = v[ia1] + v[ia2];
}
fn do_opcode_2(v: &mut Vec<u32>, i: usize) {
    let ia1: usize = v[i + 1] as usize;
    let ia2: usize = v[i + 2] as usize;
    let oa: usize = v[i + 3] as usize;
    v[oa] = v[ia1] * v[ia2];
}

const PART2_EXPECTED_OUTPUT: u32 = 19690720;
#[aoc(day02, part2, original)]
pub fn original_2b(input: &str) -> u32 {
    let v: Vec<u32> = input
        .trim()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    for noun in 0..99 {
        for verb in 0..99 {
            let mut v_cloned = v.clone();
            v_cloned[1] = noun;
            v_cloned[2] = verb;
            if compute(&mut v_cloned) == PART2_EXPECTED_OUTPUT {
                return 100 * noun + verb;
            }
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use crate::day02::original_2a;
    use crate::day02::original_2b;
    use std::fs;
    const ANSWER_2A: u32 = 5_866_714;
    const ANSWER_2B: u32 = 5208;

    #[test]
    fn t02a() {
        assert_eq!(
            ANSWER_2A,
            original_2a(&fs::read_to_string("input/2019/day2.txt").unwrap().trim())
        );
    }
    #[test]
    fn t02b() {
        assert_eq!(
            ANSWER_2B,
            original_2b(&fs::read_to_string("input/2019/day2.txt").unwrap().trim())
        );
    }
}
