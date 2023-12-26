use crate::icc::IntCodeComputer;
const GRID_X: usize = 30000;
const GRID_Y: usize = 30000;

#[derive(Clone, Copy, Debug, std::cmp::PartialEq)]
enum PanelColor {
    Black,
    White,
}

#[derive(Clone, Copy, Debug, std::cmp::PartialEq)]
enum TurnDirection {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, std::cmp::PartialEq)]
enum Orienation {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct PanelPoint {
    painted: bool,
    color: PanelColor,
}

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|o| o.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day11, part1)]
pub fn paint_panels_start_black(v: &[i64]) -> u32 {
    paint_panels(v, false, PanelColor::Black)
}

#[aoc(day11, part2)]
pub fn paint_panels_start_white(v: &[i64]) -> u32 {
    paint_panels(v, true, PanelColor::White)
}

fn paint_panels(v: &[i64], show_paint: bool, initial_color: PanelColor) -> u32 {
    let mut panels_painted = 0;
    let mut robot_orientation = Orienation::Up;
    let mut panel = vec![
        vec![
            PanelPoint {
                painted: false,
                color: PanelColor::Black
            };
            GRID_X
        ];
        GRID_Y
    ];
    let (mut x, mut y) = (GRID_X / 2, GRID_Y / 2);
    let mut icc = IntCodeComputer::new(v.to_vec(), true);
    icc.program.resize(1024 * 1024, 0);
    panel[x][y].color = initial_color;
    loop {
        icc.inputq
            .push_back(if panel[x][y].color == PanelColor::Black {
                0
            } else {
                1
            });
        icc.execute();
        if icc.terminated {
            break;
        }
        let paint_this_panel_color = if icc.consume_output().parse::<i64>().unwrap() == 0 {
            PanelColor::Black
        } else {
            PanelColor::White
        };
        if !panel[x][y].painted {
            panels_painted += 1;
        }
        panel[x][y].painted = true;
        panel[x][y].color = paint_this_panel_color;
        icc.execute();
        if icc.terminated {
            break;
        }
        let turn_direction = if icc.consume_output().parse::<i64>().unwrap() == 0 {
            TurnDirection::Left
        } else {
            TurnDirection::Right
        };
        match robot_orientation {
            Orienation::Up => {
                if turn_direction == TurnDirection::Left {
                    robot_orientation = Orienation::Left;
                    x -= 1;
                } else {
                    robot_orientation = Orienation::Right;
                    x += 1;
                }
            }
            Orienation::Down => {
                if turn_direction == TurnDirection::Left {
                    robot_orientation = Orienation::Right;
                    x += 1;
                } else {
                    robot_orientation = Orienation::Left;
                    x -= 1;
                }
            }
            Orienation::Left => {
                if turn_direction == TurnDirection::Left {
                    robot_orientation = Orienation::Down;
                    y -= 1;
                } else {
                    robot_orientation = Orienation::Up;
                    y += 1;
                }
            }
            Orienation::Right => {
                if turn_direction == TurnDirection::Left {
                    robot_orientation = Orienation::Up;
                    y += 1;
                } else {
                    robot_orientation = Orienation::Down;
                    y -= 1;
                }
            } //_ => unreachable!(),
        }
    }
    if show_paint {
        let margin = 50;
        for x in (GRID_X / 2) - margin..(GRID_X / 2) + margin {
            for y in (GRID_Y / 2) - margin..(GRID_Y / 2 + margin) {
                match panel[x][y].color {
                    PanelColor::Black => print!(" "),
                    PanelColor::White => print!("*"),
                    //_ => unreachable!(),
                }
            }
            println!();
        }
    }
    panels_painted
}

#[cfg(test)]
mod tests {
    use crate::day11::generator;
    use crate::day11::paint_panels_start_black;
    use crate::day11::paint_panels_start_white;
    use std::fs;
    const ANSWER_11A: u32 = 2088;
    const ANSWER_11B: u32 = 249;

    #[test]
    fn t11a() {
        assert_eq!(
            ANSWER_11A,
            paint_panels_start_black(&generator(
                &fs::read_to_string("input/2019/day11.txt").unwrap().trim()
            ))
        );
    }
    #[test]
    fn t11b() {
        assert_eq!(
            ANSWER_11B,
            paint_panels_start_white(&generator(
                &fs::read_to_string("input/2019/day11.txt").unwrap().trim()
            ))
        );
    }
}
