use icc::IntCodeComputer;

#[derive(PartialEq, Debug)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

fn get_tile(tile_num: i64) -> Tile {
    match tile_num {
        0 => Tile::Empty,
        1 => Tile::Wall,
        2 => Tile::Block,
        3 => Tile::HorizontalPaddle,
        4 => Tile::Ball,
        _ => unreachable!(),
    }
}

#[aoc(day13, part1)]
pub fn original_13a(input: &str) -> i64 {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    let mut icc = IntCodeComputer::new(v, true);
    icc.program.resize(1024 * 1024, 0);
    let mut block_tiles: i64 = 0;
    loop {
        icc.execute();
        if icc.terminated {
            break;
        }
        let _x: i32 = icc.consume_output().parse().unwrap();
        icc.execute();
        let _y: i32 = icc.consume_output().parse().unwrap();
        icc.execute();
        let t: Tile = get_tile(icc.consume_output().parse().unwrap());
        if t == Tile::Block {
            block_tiles += 1;
        }
    }
    block_tiles
}

#[aoc(day13, part2)]
pub fn original_13b(input: &str) -> i64 {
    let mut v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();
    v[0] = 2; // 2 quarters
    let mut icc = IntCodeComputer::new(v, true);
    icc.program.resize(1024 * 1024, 0);
    icc.inputq.push_back(0); // neutral joystick by default
    let mut score: i64 = 0;
    let mut paddle_x: i32 = 0;
    let mut ball_x: i32 = 0;
    loop {
        icc.execute();
        if icc.terminated {
            break;
        } else if icc.previous_operation == 4 {
            let x: i32 = icc.consume_output().parse().unwrap();
            icc.execute();
            let y: i32 = icc.consume_output().parse().unwrap();
            icc.execute();
            let output: i64 = icc.consume_output().parse().unwrap();
            if x == -1 && y == 0 {
                score = output;
            } else {
                let t: Tile = get_tile(output);
                if t == Tile::Block {
                    // draw tile block
                } else if t == Tile::HorizontalPaddle {
                    paddle_x = x;
                } else if t == Tile::Ball {
                    ball_x = x;
                }
            }
        }
        let new_input = if ball_x < paddle_x {
            -1
        } else if ball_x > paddle_x {
            1
        } else {
            0
        };
        if icc.inputq.is_empty() {
            icc.inputq.push_back(new_input)
        } else {
            let back = icc.inputq.back_mut().unwrap();
            *back = new_input;
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use day13::original_13a;
    use day13::original_13b;
    use std::fs;
    const ANSWER_13A: i64 = 427;
    const ANSWER_13B: i64 = 21426;

    #[test]
    fn t13a() {
        assert_eq!(
            ANSWER_13A,
            original_13a(&fs::read_to_string("input/2019/day13.txt").unwrap().trim())
        );
    }
    #[test]
    fn t13b() {
        assert_eq!(
            ANSWER_13B,
            original_13b(&fs::read_to_string("input/2019/day13.txt").unwrap().trim())
        );
    }
}
