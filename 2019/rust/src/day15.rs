use crate::icc::IntCodeComputer;
use rand::prelude::*;

const GRID_X: usize = 100;
const GRID_Y: usize = 100;

#[aoc(day15, part1)]
pub fn original_15a(input: &str) -> u32 {
    let v: Vec<i64> = input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect();

    let mut min_steps: u32 = std::u32::MAX;
    // TODO: Implement proper, non-random, faster solution.
    for i in 1..=10000 {
        print!("Sim {} ", i);
        let steps = random_walk(v.clone());
        min_steps = std::cmp::min(steps, min_steps);
        println!(" min steps: {}", min_steps);
    }
    //print_grid(&grid);
    min_steps
}

fn random_walk(program: Vec<i64>) -> u32 {
    let mut grid = [[' '; GRID_X]; GRID_Y];
    let mut x = GRID_X / 2;
    let mut y = GRID_Y / 2;
    grid[x][y] = 'X';
    let mut rng = rand::thread_rng();
    let mut steps: u32 = 0;
    let mut icc = IntCodeComputer::new(program, true);
    icc.program.resize(1024 * 1024, 0);
    loop {
        let movement = rng.gen_range(1, 5); //NSWE
        icc.inputq.push_back(movement);
        icc.execute();
        assert!(!icc.output.is_empty());
        match icc.consume_output().parse().unwrap() {
            0 => {
                match movement {
                    1 => grid[x][y + 1] = '#',
                    2 => grid[x][y - 1] = '#',
                    3 => grid[x - 1][y] = '#',
                    4 => grid[x + 1][y] = '#',
                    _ => unreachable!(),
                };
                //println!("Robot found wall. Still at ({},{})", x, y);
            }
            1 => {
                grid[x][y] = if grid[x][y] == 'X' { 'X' } else { '.' };
                match movement {
                    1 => y += 1,
                    2 => y -= 1,
                    3 => x -= 1,
                    4 => x += 1,
                    _ => unreachable!(),
                }
                //println!("Robot moved to ({},{})", x, y);
                grid[x][y] = if grid[x][y] == 'X' { 'X' } else { '.' };
                steps += 1;
            }
            2 => {
                grid[x][y] = if grid[x][y] == 'X' { 'X' } else { '.' };
                match movement {
                    1 => y += 1,
                    2 => y -= 1,
                    3 => x -= 1,
                    4 => x += 1,
                    _ => unreachable!(),
                }
                //println!("Robot moved to ({},{}) and found oxygen system there", x, y);
                grid[x][y] = 'S';
                steps += 1;
                break steps;
            }
            _ => unreachable!(),
        }
    }
}

#[allow(dead_code)]
fn print_grid(grid: &[[char; GRID_X]; GRID_Y]) {
    for a in 0..GRID_X {
        for b in 0..GRID_Y {
            print!("{}", grid[b][a]);
        }
        println!();
    }
}
fn print_grid_small(grid: &[[char; 41]; 41]) {
    for a in 0..41 {
        for b in 0..41 {
            print!("{}", grid[b][a]);
        }
        println!();
    }
}

#[aoc(day15, part2)]
pub fn original_15b(_input: &str) -> u32 {
    // Solved grid from part 1 using a random walk and 1000 iterations. Does not always fully solve the maze, of course.
    let mut grid: [[char; 41]; 41] = [
        [
            ' ', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', ' ', '#', '#', '#', '#',
            '#', '#', '#', '#', '#', '#', '#', ' ', '#', '#', '#', ' ', '#', '#', '#', '#', '#',
            '#', '#', '#', '#', '#', '#', ' ',
        ],
        [
            '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', '#',
        ],
        [
            '#', ' ', '#', '#', '#', '#', '#', ' ', '#', '#', '#', '#', '#', ' ', '#', '#', '#',
            '#', '#', '#', '#', ' ', '#', '#', '#', ' ', '#', ' ', '#', ' ', '#', '#', '#', '#',
            '#', '#', '#', '#', '#', '#', ' ',
        ],
        [
            '#', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#', ' ', '#', ' ', ' ',
            ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#', ' ', '#', ' ', '#', ' ', ' ', ' ',
            '#', ' ', ' ', ' ', ' ', ' ', '#',
        ],
        [
            '#', ' ', '#', '#', '#', '#', '#', '#', '#', '#', '#', ' ', '#', ' ', '#', '#', '#',
            '#', '#', ' ', '#', '#', '#', '#', '#', '#', '#', ' ', '#', ' ', '#', ' ', '#', ' ',
            '#', ' ', '#', '#', '#', ' ', '#',
        ],
        [
            '#', ' ', '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ',
            ' ', '#', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', '#', ' ', ' ', ' ', '#', ' ',
            ' ', ' ', ' ', ' ', '#', ' ', '#',
        ],
        [
            '#', ' ', '#', ' ', '#', '#', '#', '#', '#', '#', '#', ' ', '#', '#', '#', '#', '#',
            ' ', '#', ' ', '#', '#', '#', ' ', '#', ' ', '#', '#', '#', '#', '#', '#', '#', '#',
            '#', '#', '#', '#', '#', ' ', '#',
        ],
        [
            '#', ' ', '#', ' ', '#', 'O', ' ', ' ', ' ', ' ', '#', ' ', '#', ' ', ' ', ' ', '#',
            ' ', ' ', ' ', '#', ' ', ' ', ' ', '#', ' ', '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            '#', ' ', ' ', ' ', ' ', ' ', '#',
        ],
        [
            '#', ' ', '#', ' ', '#', '#', '#', '#', '#', ' ', '#', ' ', '#', '#', '#', ' ', '#',
            '#', '#', ' ', '#', '#', '#', '#', '#', ' ', '#', ' ', '#', '#', '#', '#', '#', ' ',
            '#', ' ', '#', '#', '#', ' ', '#',
        ],
        [
            '#', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', '#', ' ', '#', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ',
            '#', ' ', '#', ' ', ' ', ' ', '#',
        ],
        [
            '#', ' ', '#', '#', '#', '#', '#', ' ', '#', '#', '#', '#', '#', '#', '#', '#', '#',
            '#', '#', ' ', '#', ' ', '#', ' ', '#', '#', '#', '#', '#', '#', '#', ' ', '#', '#',
            '#', ' ', '#', '#', '#', '#', ' ',
        ],
        [
            '#', ' ', ' ', ' ', '#', ' ', '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', '#', ' ', '#', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', '#',
        ],
        [
            ' ', '#', '#', ' ', '#', ' ', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
            '#', '#', '#', '#', ' ', '#', '#', '#', ' ', '#', ' ', '#', '#', '#', '#', '#', ' ',
            '#', '#', '#', '#', '#', ' ', '#',
        ],
        [
            '#', ' ', '#', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', ' ', '#', ' ', '#', ' ', '#', ' ', '#', ' ', ' ', ' ', '#', ' ',
            '#', ' ', ' ', ' ', '#', ' ', '#',
        ],
        [
            '#', ' ', '#', ' ', '#', ' ', '#', '#', '#', '#', '#', ' ', '#', '#', '#', '#', '#',
            '#', '#', '#', '#', '#', '#', ' ', '#', ' ', '#', ' ', '#', ' ', '#', ' ', '#', '#',
            '#', ' ', '#', ' ', '#', ' ', '#',
        ],
        [
            '#', ' ', '#', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ',
            ' ', '#', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', '#', ' ', '#', ' ', '#', ' ',
            ' ', ' ', '#', ' ', ' ', ' ', '#',
        ],
        [
            '#', ' ', '#', ' ', '#', '#', '#', ' ', '#', '#', '#', ' ', '#', ' ', '#', '#', '#',
            ' ', '#', '#', '#', '#', '#', ' ', '#', '#', '#', '#', '#', ' ', '#', ' ', '#', ' ',
            '#', '#', '#', '#', '#', ' ', '#',
        ],
        [
            '#', ' ', '#', ' ', '#', ' ', ' ', ' ', '#', ' ', ' ', ' ', '#', ' ', '#', ' ', '#',
            ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ',
            '#', ' ', ' ', ' ', '#', ' ', '#',
        ],
        [
            '#', ' ', '#', ' ', '#', ' ', '#', '#', '#', '#', '#', '#', '#', ' ', '#', ' ', '#',
            '#', '#', '#', '#', ' ', '#', ' ', '#', '#', '#', ' ', '#', '#', '#', ' ', '#', '#',
            '#', '#', '#', ' ', '#', ' ', '#',
        ],
        [
            '#', ' ', ' ', ' ', '#', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#', ' ', '#', ' ', ' ',
            ' ', ' ', ' ', '#', ' ', '#', ' ', ' ', ' ', '#', ' ', '#', ' ', ' ', ' ', '#', ' ',
            ' ', ' ', '#', ' ', ' ', ' ', '#',
        ],
        [
            '#', ' ', '#', '#', '#', ' ', '#', ' ', '#', '#', '#', ' ', '#', ' ', '#', '#', '#',
            ' ', '#', ' ', '#', '#', '#', '#', '#', '#', '#', ' ', '#', '#', '#', '#', '#', ' ',
            '#', ' ', '#', ' ', '#', '#', ' ',
        ],
        [
            '#', ' ', ' ', ' ', '#', ' ', ' ', ' ', '#', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ',
            ' ', '#', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#', ' ', '#', ' ', ' ', ' ', ' ', ' ',
            '#', ' ', '#', ' ', '#', ' ', '#',
        ],
        [
            ' ', '#', '#', ' ', '#', '#', '#', '#', '#', ' ', '#', '#', '#', '#', '#', '#', '#',
            '#', '#', ' ', '#', ' ', '#', ' ', '#', ' ', '#', ' ', '#', ' ', '#', '#', '#', '#',
            '#', ' ', '#', ' ', '#', ' ', '#',
        ],
        [
            '#', ' ', '#', ' ', ' ', ' ', '#', ' ', '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', '#', ' ', '#', ' ', '#', ' ', ' ', ' ', '#', ' ', ' ', ' ', '#', ' ',
            ' ', ' ', '#', ' ', ' ', ' ', '#',
        ],
        [
            '#', ' ', '#', '#', '#', ' ', '#', ' ', '#', '#', '#', '#', '#', '#', '#', '#', '#',
            '#', '#', '#', '#', ' ', '#', ' ', '#', '#', '#', '#', '#', '#', '#', ' ', '#', ' ',
            '#', '#', '#', '#', '#', ' ', '#',
        ],
        [
            '#', ' ', '#', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', '#', ' ', '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', ' ',
            '#', ' ', ' ', ' ', '#', ' ', '#',
        ],
        [
            '#', ' ', '#', ' ', '#', '#', '#', '#', '#', '#', '#', ' ', '#', '#', '#', '#', '#',
            '#', '#', ' ', '#', ' ', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', ' ',
            '#', ' ', '#', ' ', '#', '#', ' ',
        ],
        [
            '#', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#',
            ' ', '#', ' ', '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', '#', ' ',
            ' ', ' ', '#', ' ', ' ', ' ', '#',
        ],
        [
            '#', ' ', '#', '#', '#', ' ', '#', '#', '#', ' ', '#', '#', '#', '#', '#', ' ', '#',
            ' ', '#', ' ', '#', '#', '#', ' ', '#', '#', '#', '#', '#', ' ', '#', ' ', '#', '#',
            '#', '#', ' ', '#', '#', ' ', '#',
        ],
        [
            '#', ' ', '#', ' ', ' ', ' ', '#', ' ', '#', ' ', '#', ' ', ' ', ' ', '#', ' ', '#',
            ' ', '#', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ',
            ' ', ' ', '#', ' ', ' ', ' ', '#',
        ],
        [
            '#', ' ', '#', ' ', '#', '#', '#', ' ', '#', ' ', '#', ' ', '#', '#', '#', ' ', '#',
            ' ', '#', '#', '#', '#', '#', '#', '#', ' ', '#', '#', '#', '#', '#', '#', '#', '#',
            '#', ' ', '#', ' ', '#', ' ', '#',
        ],
        [
            '#', ' ', '#', ' ', ' ', ' ', '#', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#',
            ' ', ' ', ' ', ' ', ' ', '#', ' ', '#', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', ' ', ' ', '#', ' ', '#',
        ],
        [
            '#', ' ', '#', '#', '#', ' ', '#', '#', '#', ' ', '#', '#', '#', ' ', '#', '#', '#',
            '#', '#', ' ', '#', ' ', '#', ' ', '#', '#', '#', ' ', '#', '#', '#', '#', '#', '#',
            '#', '#', '#', '#', '#', ' ', '#',
        ],
        [
            '#', ' ', ' ', ' ', '#', ' ', ' ', ' ', '#', ' ', ' ', ' ', '#', ' ', '#', ' ', ' ',
            ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ' ', ' ', '#', ' ', '#', ' ', '#',
        ],
        [
            ' ', '#', '#', ' ', '#', ' ', '#', ' ', '#', '#', '#', ' ', '#', ' ', '#', ' ', '#',
            '#', '#', '#', '#', ' ', '#', '#', '#', '#', '#', ' ', '#', '#', '#', '#', '#', '#',
            '#', ' ', '#', ' ', '#', ' ', '#',
        ],
        [
            '#', ' ', ' ', ' ', '#', ' ', '#', ' ', '#', ' ', '#', ' ', '#', ' ', ' ', ' ', '#',
            ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            '#', ' ', '#', ' ', '#', ' ', '#',
        ],
        [
            '#', ' ', '#', '#', ' ', '#', '#', ' ', '#', ' ', '#', ' ', '#', '#', '#', '#', '#',
            '#', '#', ' ', '#', '#', '#', ' ', '#', ' ', '#', '#', '#', '#', '#', '#', '#', '#',
            '#', ' ', '#', ' ', '#', ' ', '#',
        ],
        [
            '#', ' ', ' ', ' ', '#', ' ', ' ', ' ', '#', ' ', '#', ' ', ' ', ' ', '#', ' ', ' ',
            ' ', '#', ' ', '#', ' ', ' ', ' ', '#', ' ', '#', ' ', ' ', ' ', '#', ' ', ' ', ' ',
            ' ', ' ', '#', ' ', '#', ' ', '#',
        ],
        [
            ' ', '#', '#', ' ', '#', ' ', '#', '#', '#', ' ', '#', '#', '#', ' ', '#', ' ', '#',
            ' ', '#', '#', '#', ' ', '#', '#', '#', ' ', '#', ' ', '#', ' ', '#', ' ', '#', '#',
            '#', '#', '#', ' ', '#', ' ', '#',
        ],
        [
            '#', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#',
            ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', '#', ' ',
            ' ', ' ', ' ', ' ', ' ', ' ', '#',
        ],
        [
            ' ', '#', '#', '#', '#', '#', ' ', '#', '#', '#', '#', '#', '#', '#', '#', '#', ' ',
            '#', '#', '#', '#', '#', ' ', '#', '#', '#', '#', '#', ' ', '#', '#', '#', ' ', '#',
            '#', '#', '#', '#', '#', '#', ' ',
        ],
    ];

    let mut minutes: u32 = 0;
    loop {
        // grow O's with "little" o's
        for a in 0..41 {
            for b in 0..41 {
                if grid[b][a] == 'O' {
                    if b as i32 > 0 {
                        grid[b - 1][a] = if grid[b - 1][a] == 'O' {
                            'O'
                        } else if grid[b - 1][a] == 'o' || grid[b - 1][a] == ' ' {
                            'o'
                        } else {
                            '#'
                        };
                    }
                    if b as i32 + 1 < 41 {
                        grid[b + 1][a] = if grid[b + 1][a] == 'O' {
                            'O'
                        } else if grid[b + 1][a] == 'o' || grid[b + 1][a] == ' ' {
                            'o'
                        } else {
                            '#'
                        };
                    }
                    if a as i32 > 0 {
                        grid[b][a - 1] = if grid[b][a - 1] == 'O' {
                            'O'
                        } else if grid[b][a - 1] == 'o' || grid[b][a - 1] == ' ' {
                            'o'
                        } else {
                            '#'
                        };
                    }
                    if a as i32 + 1 < 41 {
                        grid[b][a + 1] = if grid[b][a + 1] == 'O' {
                            'O'
                        } else if grid[b][a + 1] == 'o' || grid[b][a + 1] == ' ' {
                            'o'
                        } else {
                            '#'
                        };
                    }
                }
            }
        }

        // promote little o's
        let mut little_os_promoted = 0;
        for a in 0..41 {
            for b in 0..41 {
                if grid[b][a] == 'o' {
                    grid[b][a] = 'O';
                    little_os_promoted += 1;
                }
            }
        }

        if little_os_promoted == 0 {
            break;
        } else {
            minutes += 1;
        }
    }
    print_grid_small(&grid);
    minutes
}

#[cfg(test)]
mod tests {
    use crate::day15::original_15a;
    use crate::day15::original_15b;
    use std::fs;
    const ANSWER_15A: u32 = 204;
    const ANSWER_15B: u32 = 340;

    #[test]
    fn t15a() {
        assert_eq!(
            ANSWER_15A,
            original_15a(&fs::read_to_string("input/2019/day15.txt").unwrap().trim())
        );
    }
    #[test]
    fn t15b() {
        assert_eq!(
            ANSWER_15B,
            original_15b(&fs::read_to_string("input/2019/day15.txt").unwrap().trim())
        );
    }
}
