use icc::IntCodeComputer;

#[aoc(day19, part1)]
pub fn solution_19a(input: &str) -> usize {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    let mut ones: usize = 0;
    for i in 0..50 {
        for j in 0..50 {
            let mut icc = IntCodeComputer::new(v.clone(), true);
            icc.program.resize(1024, 0);
            icc.inputq.push_back(i);
            icc.inputq.push_back(j);
            icc.execute_one();
            while icc.previous_operation != 3 {
                icc.execute_one();
            }
            icc.execute_one();
            while icc.previous_operation != 3 {
                icc.execute_one();
            }
            icc.execute();
            let output = icc.consume_output();
            if output.parse::<i64>().unwrap() == 1 {
                ones += 1;
            }
        }
    }
    ones
}

fn math_on_closest_x_y_in_square(x: usize, y: usize) -> usize {
    x * 10000 + y
}

#[aoc(day19, part2)]
pub fn solution_19b(input: &str) -> usize {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut grid: Vec<Vec<u8>> = vec![vec![0; 10000]; 10000];
    // TODO: This is very slow, so rework to do something like a binary search for 100 length rows and columns
    'outer: for j in 0..10000 {
        let mut beam = false;
        for i in 0..10000 {
            let mut icc = IntCodeComputer::new(v.clone(), true);
            icc.program.resize(1024, 0);
            icc.inputq.push_back(i as i64);
            icc.inputq.push_back(j as i64);
            icc.execute_one();
            while icc.previous_operation != 3 {
                icc.execute_one();
            }
            icc.execute_one();
            while icc.previous_operation != 3 {
                icc.execute_one();
            }
            icc.execute();
            let output = icc.consume_output().parse::<i64>().unwrap();
            if output == 1 {
                grid[i][j] = 1;
                if !beam {
                    // check the diagonal for the full 100x100
                    let mut beam_square = true;
                    for k in 0..100 {
                        let r = grid.get(i + k);
                        if r.is_some() {
                            if j < k {
                                beam_square = false;
                                break;
                            } else {
                                let c = r.unwrap().get(j - k);
                                if c.is_some() {
                                    if *c.unwrap() != 1 {
                                        beam_square = false;
                                        break;
                                    }
                                } else {
                                    beam_square = false;
                                    break;
                                }
                            }
                        } else {
                            beam_square = false;
                            break;
                        }
                    }
                    if beam_square {
                        x = i;
                        y = j;
                        break 'outer;
                    }
                }
                beam = true;
            } else {
                if beam {
                    break;
                }
                grid[i][j] = 0;
            }
        }
    }
    /*
    for i in x..x + 100 {
        for j in y - 100..y {
            grid[i][j] = 8;
        }
    }
    for i in 0..1200 {
        for j in 0..1200 {
            print!("{}", grid[i][j]);
        }
        println!();
    }
    */
    math_on_closest_x_y_in_square(x, y - 100 + 1) // TODO: Why do we need to adjust by 1?
}

#[cfg(test)]
mod tests {
    use day19::solution_19a;
    use day19::solution_19b;
    use std::fs;
    const ANSWER_19A: usize = 226;
    const ANSWER_19B: usize = 7900946;

    #[test]
    fn t19a() {
        assert_eq!(
            ANSWER_19A,
            solution_19a(&fs::read_to_string("input/2019/day19.txt").unwrap().trim())
        );
    }
    #[test]
    fn t19b() {
        assert_eq!(
            ANSWER_19B,
            solution_19b(&fs::read_to_string("input/2019/day19.txt").unwrap().trim())
        );
    }
}
