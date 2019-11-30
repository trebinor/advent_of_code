const GRID_X: usize = 30000;
const GRID_Y: usize = 30000;

struct GridPoint {
    x: usize,
    y: usize,
}

#[aoc(day03, part1, original)]
pub fn original_3a(input: &str) -> u32 {
    let v: Vec<&str> = input.split('\n').collect();
    let v1 = v[0].trim().split(',').collect();
    let v2 = v[1].trim().split(',').collect();
    let mut grid1 = vec![vec![false; GRID_X]; GRID_Y];
    let mut grid2 = vec![vec![false; GRID_X]; GRID_Y];
    mut_grid_3a(&mut grid1, v1);
    mut_grid_3a(&mut grid2, v2);

    let mut manhattans = Vec::new();
    for x in 0..GRID_X {
        for y in 0..GRID_Y {
            if grid1[x][y] && grid2[x][y] {
                let manhattan = ((x as i32) - (GRID_X as i32) / 2).abs()
                    + ((y as i32) - (GRID_Y as i32) / 2).abs();
                if manhattan != 0 {
                    manhattans.push(manhattan);
                }
            }
        }
    }
    *manhattans.iter().min().unwrap() as u32
}

fn mut_grid_3a(grid: &mut Vec<Vec<bool>>, wires: Vec<&str>) {
    let mut grid_point: GridPoint = GridPoint {
        x: GRID_X / 2,
        y: GRID_Y / 2,
    };
    for w in wires {
        let dir = w.chars().next().unwrap();
        let dist = w.get(1..).unwrap().parse::<usize>().unwrap();
        if dir == 'R' {
            for d in 0..dist {
                grid[grid_point.x + d][grid_point.y] = true;
            }
            grid_point.x += dist;
        } else if dir == 'U' {
            for d in 0..dist {
                grid[grid_point.x][grid_point.y + d] = true;
            }
            grid_point.y += dist;
        } else if dir == 'L' {
            for d in 0..dist {
                grid[grid_point.x - d][grid_point.y] = true;
            }
            grid_point.x -= dist;
        } else if dir == 'D' {
            for d in 0..dist {
                grid[grid_point.x][grid_point.y - d] = true;
            }
            grid_point.y -= dist;
        }
    }
}

fn mut_grid_3b(grid: &mut Vec<Vec<bool>>, wires: &mut Vec<&str>) {
    let mut grid_point: GridPoint = GridPoint {
        x: GRID_X / 2,
        y: GRID_Y / 2,
    };
    for w in wires {
        let dir = w.chars().next().unwrap();
        let dist = w.get(1..).unwrap().parse::<usize>().unwrap();
        if dir == 'R' {
            for d in 0..dist {
                grid[grid_point.x + d][grid_point.y] = true;
            }
            grid_point.x += dist;
        } else if dir == 'U' {
            for d in 0..dist {
                grid[grid_point.x][grid_point.y + d] = true;
            }
            grid_point.y += dist;
        } else if dir == 'L' {
            for d in 0..dist {
                grid[grid_point.x - d][grid_point.y] = true;
            }
            grid_point.x -= dist;
        } else if dir == 'D' {
            for d in 0..dist {
                grid[grid_point.x][grid_point.y - d] = true;
            }
            grid_point.y -= dist;
        }
    }
}
#[aoc(day03, part2, original)]
pub fn original_3b(input: &str) -> u32 {
    let v: Vec<&str> = input.split('\n').collect();
    let mut v1 = v[0].trim().split(',').collect();
    let mut v2 = v[1].trim().split(',').collect();
    let mut grid1 = vec![vec![false; GRID_X]; GRID_Y];
    let mut grid2 = vec![vec![false; GRID_X]; GRID_Y];
    mut_grid_3b(&mut grid1, &mut v1);
    mut_grid_3b(&mut grid2, &mut v2);

    let mut intersections = Vec::new();
    for x in 0..GRID_X {
        for y in 0..GRID_Y {
            if grid1[x][y] && grid2[x][y] {
                intersections.push(GridPoint { x: x, y: y });
            }
        }
    }

    // find shortest paths to each intersection and return minimum sum
    intersections
        .into_iter()
        .map(|mut i| walk_wires(&mut i, &mut v1) + walk_wires(&mut i, &mut v2))
        .min_by(|x, y| {
            if *x == 0 {
                std::cmp::Ordering::Greater
            } else if *y == 0 {
                std::cmp::Ordering::Less
            } else {
                x.cmp(y)
            }
        })
        .unwrap() as u32
}

fn walk_wires(i: &mut GridPoint, wires: &mut Vec<&str>) -> usize {
    let mut steps: usize = 0;
    let mut x = GRID_X / 2;
    let mut y = GRID_Y / 2;
    'outer: for w in wires {
        let dir = w.chars().next().unwrap();
        let dist = w.get(1..).unwrap().parse::<usize>().unwrap();
        if dir == 'R' {
            for d in 0..dist {
                if x + d == i.x && y == i.y {
                    steps += d;
                    break 'outer;
                }
            }
            x += dist;
            steps += dist;
        } else if dir == 'U' {
            for d in 0..dist {
                if x == i.x && y + d == i.y {
                    steps += d;
                    break 'outer;
                }
            }
            y += dist;
            steps += dist;
        } else if dir == 'L' {
            for d in 0..dist {
                if x - d == i.x && y == i.y {
                    steps += d;
                    break 'outer;
                }
            }
            x -= dist;
            steps += dist;
        } else if dir == 'D' {
            for d in 0..dist {
                if x == i.x && y - d == i.y {
                    steps += d;
                    break 'outer;
                }
            }
            y -= dist;
            steps += dist;
        }
    }
    steps
}

#[cfg(test)]
mod tests {
    use day03::original_3a;
    use day03::original_3b;
    use std::fs;
    const ANSWER_3A: u32 = 557;
    const ANSWER_3B: u32 = 56410;

    #[test]
    fn t03a() {
        assert_eq!(
            ANSWER_3A,
            original_3a(&fs::read_to_string("input/2019/day3.txt").unwrap().trim())
        );
    }
    #[test]
    fn t03b() {
        assert_eq!(
            ANSWER_3B,
            original_3b(&fs::read_to_string("input/2019/day3.txt").unwrap().trim())
        );
    }
}
