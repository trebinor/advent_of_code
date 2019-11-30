use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Life {
    Empty,
    Bug,
}

const GRID_X: usize = 5;
const GRID_Y: usize = 5;
type GridLevel = [[Life; GRID_X]; GRID_Y];

#[aoc_generator(day24)]
pub fn generator(input: &str) -> [[Life; GRID_X]; GRID_Y] {
    let v = input.lines().map(|l| l.trim()).collect::<Vec<&str>>();
    let mut eris = [[Life::Empty; GRID_X]; GRID_Y];
    for (i, l) in v.iter().enumerate() {
        for (j, x) in l.chars().enumerate() {
            eris[i][j] = match x {
                '.' => Life::Empty,
                '#' => Life::Bug,
                _ => unreachable!(),
            }
        }
    }
    eris
}

fn biodiversity(grid: GridLevel) -> usize {
    let mut sum: usize = 0;
    for i in 0..GRID_X {
        for j in 0..GRID_Y {
            if grid[i][j] == Life::Bug {
                sum += 2usize.pow((i as u32) * (GRID_X as u32) + j as u32);
            }
        }
    }
    sum
}

fn total_bugs(grid: GridLevel) -> usize {
    let mut sum: usize = 0;
    for i in 0..GRID_X {
        for j in 0..GRID_Y {
            if grid[i][j] == Life::Bug {
                sum += 1;
            }
        }
    }
    sum
}

#[allow(dead_code)]
fn print_generation(grid: [[Life; GRID_X]; GRID_Y]) {
    for i in 0..GRID_X {
        for j in 0..GRID_Y {
            match grid[i][j] {
                Life::Empty => print!("."),
                Life::Bug => print!("#"),
            }
        }
        println!();
    }
    println!();
}

fn new_generation(grid: &mut [[Life; GRID_X]; GRID_Y]) {
    let mut grid_expanded = [[Life::Empty; GRID_X + 2]; GRID_Y + 2];
    for i in 0..GRID_X {
        for j in 0..GRID_Y {
            grid_expanded[i + 1][j + 1] = grid[i][j];
        }
    }
    for i in 1..=GRID_X {
        for j in 1..=GRID_Y {
            grid[i - 1][j - 1] = next_state(
                grid_expanded[i][j],
                &[
                    grid_expanded[i - 1][j],
                    grid_expanded[i + 1][j],
                    grid_expanded[i][j - 1],
                    grid_expanded[i][j + 1],
                ],
            );
        }
    }
}

fn next_state(current_state: Life, adjacents: &[Life]) -> Life {
    let s = adjacents
        .iter()
        .map(|a| if *a == Life::Bug { 1 } else { 0 })
        .sum::<u32>();
    match current_state {
        Life::Bug => {
            if s == 1 {
                Life::Bug
            } else {
                Life::Empty
            }
        }
        Life::Empty => {
            if s == 1 || s == 2 {
                Life::Bug
            } else {
                Life::Empty
            }
        }
    }
}

fn new_generation_recursive(grids: &mut VecDeque<GridLevel>) {
    let gc = grids.clone();
    for g in 1..grids.len() - 1 {
        // skip first and last grids since we know they're empty and it makes the algorithm work in all cases
        for i in 0..GRID_X {
            for j in 0..GRID_Y {
                match (i, j) {
                    (0, 0) => {
                        grids[g][i][j] = next_state(
                            gc[g][i][j],
                            &[
                                gc[g - 1][GRID_X / 2][GRID_Y / 2 - 1],
                                gc[g][i][j + 1],
                                gc[g][i + 1][j],
                                gc[g - 1][GRID_X / 2 - 1][GRID_Y / 2],
                            ],
                        )
                    }
                    (1, 0) | (2, 0) | (3, 0) => {
                        // TODO: generalize arms like this for GRID_X and GRID_Y
                        grids[g][i][j] = next_state(
                            gc[g][i][j],
                            &[
                                gc[g - 1][GRID_X / 2][GRID_Y / 2 - 1],
                                gc[g][i][j + 1],
                                gc[g][i + 1][j],
                                gc[g][i - 1][j],
                            ],
                        )
                    }
                    (4, 0) => {
                        grids[g][i][j] = next_state(
                            gc[g][i][j],
                            &[
                                gc[g - 1][GRID_X / 2][GRID_Y / 2 - 1],
                                gc[g - 1][GRID_X / 2 + 1][GRID_Y / 2],
                                gc[g][i - 1][j],
                                gc[g][i][j + 1],
                            ],
                        )
                    }
                    (0, 4) => {
                        grids[g][i][j] = next_state(
                            gc[g][i][j],
                            &[
                                gc[g][i][j - 1],
                                gc[g - 1][GRID_X / 2][GRID_Y / 2 + 1],
                                gc[g][i + 1][j],
                                gc[g - 1][GRID_X / 2 - 1][GRID_Y / 2],
                            ],
                        )
                    }
                    (0, 1) | (0, 2) | (0, 3) => {
                        grids[g][i][j] = next_state(
                            gc[g][i][j],
                            &[
                                gc[g][i][j - 1],
                                gc[g][i][j + 1],
                                gc[g][i + 1][j],
                                gc[g - 1][GRID_X / 2 - 1][GRID_Y / 2],
                            ],
                        )
                    }
                    (1, 4) | (2, 4) | (3, 4) => {
                        // TODO: generalize arms like this for GRID_X and GRID_Y
                        grids[g][i][j] = next_state(
                            gc[g][i][j],
                            &[
                                gc[g][i][j - 1],
                                gc[g - 1][GRID_X / 2][GRID_Y / 2 + 1],
                                gc[g][i + 1][j],
                                gc[g][i - 1][j],
                            ],
                        )
                    }
                    (4, 1) | (4, 2) | (4, 3) => {
                        // TODO: generalize arms like this for GRID_X and GRID_Y
                        grids[g][i][j] = next_state(
                            gc[g][i][j],
                            &[
                                gc[g][i][j - 1],
                                gc[g][i][j + 1],
                                gc[g - 1][GRID_X / 2 + 1][GRID_Y / 2],
                                gc[g][i - 1][j],
                            ],
                        )
                    }
                    (4, 4) => {
                        grids[g][i][j] = next_state(
                            gc[g][i][j],
                            &[
                                gc[g][i][j - 1],
                                gc[g - 1][GRID_X / 2][GRID_Y / 2 + 1],
                                gc[g - 1][GRID_X / 2 + 1][GRID_Y / 2],
                                gc[g][i - 1][j],
                            ],
                        )
                    }
                    (2, 2) => {}
                    (2, 1) => {
                        grids[g][i][j] = next_state(
                            gc[g][i][j],
                            &[
                                gc[g][i][j - 1],
                                gc[g][i - 1][j],
                                gc[g][i + 1][j],
                                gc[g + 1][0][0],
                                gc[g + 1][1][0],
                                gc[g + 1][2][0],
                                gc[g + 1][3][0],
                                gc[g + 1][4][0],
                            ],
                        )
                    }
                    (1, 2) => {
                        grids[g][i][j] = next_state(
                            gc[g][i][j],
                            &[
                                gc[g][i][j - 1],
                                gc[g][i][j + 1],
                                gc[g][i - 1][j],
                                gc[g + 1][0][0],
                                gc[g + 1][0][1],
                                gc[g + 1][0][2],
                                gc[g + 1][0][3],
                                gc[g + 1][0][4],
                            ],
                        )
                    }
                    (3, 2) => {
                        grids[g][i][j] = next_state(
                            gc[g][i][j],
                            &[
                                gc[g][i][j - 1],
                                gc[g][i][j + 1],
                                gc[g][i + 1][j],
                                gc[g + 1][4][0],
                                gc[g + 1][4][1],
                                gc[g + 1][4][2],
                                gc[g + 1][4][3],
                                gc[g + 1][4][4],
                            ],
                        )
                    }
                    (2, 3) => {
                        grids[g][i][j] = next_state(
                            gc[g][i][j],
                            &[
                                gc[g][i - 1][j],
                                gc[g][i][j + 1],
                                gc[g][i + 1][j],
                                gc[g + 1][0][4],
                                gc[g + 1][1][4],
                                gc[g + 1][2][4],
                                gc[g + 1][3][4],
                                gc[g + 1][4][4],
                            ],
                        )
                    }
                    (1, 1) | (3, 1) | (1, 3) | (3, 3) => {
                        grids[g][i][j] = next_state(
                            gc[g][i][j],
                            &[
                                gc[g][i][j - 1],
                                gc[g][i][j + 1],
                                gc[g][i + 1][j],
                                gc[g][i - 1][j],
                            ],
                        )
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
        }
    }
}

#[aoc(day24, part1)]
pub fn biodiversity_of_first_repeated_state(grid: &[[Life; GRID_X]; GRID_Y]) -> usize {
    let mut grid_mutable = *grid;
    let mut bio: HashSet<usize> = HashSet::new();
    while bio.insert(biodiversity(grid_mutable)) {
        new_generation(&mut grid_mutable);
    }
    biodiversity(grid_mutable)
}

#[aoc(day24, part2)]
pub fn total_bugs_200_iterations(grid: &GridLevel) -> usize {
    total_bugs_n_iterations(grid, 200)
}

fn total_bugs_n_iterations(grid: &GridLevel, n: usize) -> usize {
    let mut grids: VecDeque<GridLevel> = VecDeque::new();
    grids.push_front(grid.clone());
    // prefill more than enough inner and outer grids, a pair per generation, to avoid adding logic for detecting when they need to be added
    let outer_grid = [[Life::Empty; GRID_X]; GRID_Y];
    let inner_grid = [[Life::Empty; GRID_X]; GRID_Y];
    for _iterations in 0..n {
        grids.push_back(outer_grid.clone());
        grids.push_front(inner_grid.clone());
    }

    for _iterations in 0..n {
        new_generation_recursive(&mut grids);
    }
    grids.iter().fold(0, |acc, g| acc + total_bugs(*g))
}

#[cfg(test)]
mod tests {
    use day24::biodiversity_of_first_repeated_state;
    use day24::generator;
    use day24::total_bugs_200_iterations;
    use day24::total_bugs_n_iterations;
    use std::fs;
    const ANSWER_24A: usize = 1_151_290;
    const ANSWER_24B: usize = 1953;
    const UNIT_ANSWER_24A: usize = 2_129_920;
    const UNIT_ANSWER_24B: usize = 99;
    const UNIT_INPUT_24A: &str = r"....#
#..#.
#..##
..#..
#....";
    const UNIT_INPUT_24B: &str = r"....#
#..#.
#..##
..#..
#....";

    #[test]
    fn t24a() {
        assert_eq!(
            ANSWER_24A,
            biodiversity_of_first_repeated_state(&generator(
                &fs::read_to_string("input/2019/day24.txt").unwrap().trim()
            ))
        );
    }
    #[test]
    fn t24b() {
        assert_eq!(
            ANSWER_24B,
            total_bugs_200_iterations(&generator(
                &fs::read_to_string("input/2019/day24.txt").unwrap().trim()
            ))
        );
    }
    #[test]
    fn t24a_supplied_inputs() {
        assert_eq!(
            UNIT_ANSWER_24A,
            biodiversity_of_first_repeated_state(&generator(UNIT_INPUT_24A))
        );
    }
    #[test]
    fn t24b_supplied_inputs() {
        assert_eq!(
            UNIT_ANSWER_24B,
            total_bugs_n_iterations(&generator(UNIT_INPUT_24B), 10)
        );
    }
}
