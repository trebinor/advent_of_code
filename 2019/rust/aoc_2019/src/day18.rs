#[aoc_generator(day18)]
pub fn generator(input: &str) -> Maze {
    let v = input.lines().map(|l| l).collect::<Vec<&str>>();
    let mut maze: Maze = vec![vec![' '; v[0].len()]; v.len()];

    for (y, l) in v.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            maze[y][x] = c;
        }
    }
    maze
}

#[aoc(day18, part1)]
pub fn shortest_path(input: &Maze) -> usize {
    let mut goal = Point(0, 0, 0);
    let mut origin_x: i8 = 0;
    let mut origin_y: i8 = 0;
    for (j, y) in input.iter().enumerate() {
        for (i, _x) in y.iter().enumerate() {
            match input[j][i] {
                '@' => {
                    origin_x = i as i8;
                    origin_y = j as i8;
                }
                key @ 'a'..='z' => goal.add_key(key),
                _ => {}
            }
        }
    }
    let origin = Point(origin_x, origin_y, 0);
    let shortest_path =
        pathfinding::directed::bfs::bfs(&origin, |p| p.successors(input), |p| p.2 == goal.2);
    shortest_path.unwrap().len() - 1 // subtract starting point which is included in path
}

#[aoc(day18, part2)]
pub fn shortest_path_with_quadbots_ignore_doors(input: &Maze) -> usize {
    let mut quadmaze = input.clone();
    let mut goal = Point(0, 0, 0);
    let mut origin_x: i8 = 0;
    let mut origin_y: i8 = 0;
    let mut key_points: Vec<Point> = Vec::new();
    for (j, y) in input.iter().enumerate() {
        for (i, _x) in y.iter().enumerate() {
            match input[j][i] {
                '@' => {
                    origin_x = i as i8;
                    origin_y = j as i8;
                }
                key @ 'a'..='z' => {
                    goal.add_key(key);
                    let mut p = Point(i as i8, j as i8, 0);
                    p.add_key(key);
                    key_points.push(p);
                }
                _ => {}
            }
        }
    }
    let mut bots: [Point; 4] = [Point(0, 0, 0); 4];
    for (idx, o) in [(-1, -1), (1, -1), (1, 1), (-1, 1)].iter().enumerate() {
        let i = origin_x as i8 + o.0;
        let j = origin_y as i8 + o.1;
        bots[idx] = Point(i, j, 0);
        quadmaze[j as usize][i as usize] = '@'
    }
    // println!("key_points = {:?}", key_points);
    for (bi, b) in bots.clone().iter().enumerate() {
        let bot_location = Location(b.0, b.1);
        // println!("Bot {} at {} {}", bi, b.0, b.1);
        let path = pathfinding::directed::bfs::bfs_reach(bot_location, |l| {
            l.successors_pass_locked_doors(&quadmaze)
        })
        .collect::<Vec<_>>();
        for k in key_points.clone() {
            if path.iter().any(|l| l.0 == k.0 && l.1 == k.1) {
                for j in 0..bots.len() {
                    if bi == j {
                        continue;
                    }
                    bots[j].2 |= k.2
                }
            }
        }
    }
    // println!("bots: = {:?}", bots);
    for o in [(0, 0), (-1, 0), (0, 1), (1, 0), (0, -1)].iter() {
        let i = origin_x as i8 + o.0;
        let j = origin_y as i8 + o.1;
        quadmaze[j as usize][i as usize] = '#'
    }
    // display(&quadmaze);
    let mut paths: Vec<Vec<Point>> = Vec::new();
    for b in bots.iter() {
        paths.push(
            pathfinding::directed::bfs::bfs(b, |p| p.successors(&quadmaze), |p| p.2 == goal.2)
                .unwrap(),
        );
    }
    paths.iter().map(|v| v.len() - 1).sum()
}

pub fn shortest_path_with_quadbots(input: &Maze) -> usize {
    let mut quadmaze = input.clone();
    let mut goal = Point(0, 0, 0);
    let mut origin_x: i8 = 0;
    let mut origin_y: i8 = 0;
    let mut key_points: Vec<Point> = Vec::new();
    for (j, y) in input.iter().enumerate() {
        for (i, _x) in y.iter().enumerate() {
            match input[j][i] {
                '@' => {
                    origin_x = i as i8;
                    origin_y = j as i8;
                }
                key @ 'a'..='z' => {
                    goal.add_key(key);
                    let mut p = Point(i as i8, j as i8, 0);
                    p.add_key(key);
                    key_points.push(p);
                }
                _ => {}
            }
        }
    }
    let mut bots: [Point; 4] = [Point(0, 0, 0); 4];
    for (idx, o) in [(-1, -1), (1, -1), (1, 1), (-1, 1)].iter().enumerate() {
        let i = origin_x as i8 + o.0;
        let j = origin_y as i8 + o.1;
        bots[idx] = Point(i, j, 0);
        quadmaze[j as usize][i as usize] = '@'
    }
    for o in [(0, 0), (-1, 0), (0, 1), (1, 0), (0, -1)].iter() {
        let i = origin_x as i8 + o.0;
        let j = origin_y as i8 + o.1;
        quadmaze[j as usize][i as usize] = '#'
    }

    let quadbots = QuadbotPoint {
        quadbots: vec![bots[0], bots[1], bots[2], bots[3]],
    };
    let path = pathfinding::directed::bfs::bfs(
        &quadbots,
        |p| p.successors(&quadmaze),
        |p| {
            p.quadbots[0].2 == goal.2
                && p.quadbots[1].2 == goal.2
                && p.quadbots[2].2 == goal.2
                && p.quadbots[3].2 == goal.2
        },
    );
    path.unwrap().len() - 1
}

type Maze = Vec<Vec<char>>;

#[allow(dead_code)]
fn display(maze: &Maze) {
    for (j, y) in maze.iter().enumerate() {
        for (i, _x) in y.iter().enumerate() {
            print!("{}", maze[j][i]);
        }
        println!();
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point(i8, i8, u32);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Location(i8, i8);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct QuadbotPoint {
    quadbots: Vec<Point>,
}

impl Point {
    fn key_present(&self, key: char) -> bool {
        self.2 & (1 << (key as u8 - 97u8)) != 0
    }

    fn add_key(&mut self, key: char) {
        match key {
            'a'..='z' => self.2 |= 1 << (key as u8 - 97u8),
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    fn print_keys(&self) {
        for i in 'a'..'z' {
            if self.key_present(i) {
                print!("{}", i);
            }
        }
        println!();
    }

    fn successors(&self, input: &Maze) -> Vec<Point> {
        let &Point(x, y, k) = self;
        let mut path = Vec::new();
        for m in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let i: i8 = x + m.0;
            let j: i8 = y + m.1;
            let mut keys = k;
            let mut key_point = Point(0, 0, k);
            match input[j as usize][i as usize] {
                '#' => continue,
                door @ 'A'..='Z' => {
                    if !self.key_present(door.to_ascii_lowercase()) {
                        continue;
                    }
                }
                key @ 'a'..='z' => {
                    keys = {
                        key_point.add_key(key);
                        key_point.2
                    }
                }
                _ => {}
            }
            path.push(Point(i, j, keys));
        }
        path
    }
}

impl QuadbotPoint {
    fn successors(&self, input: &Maze) -> Vec<QuadbotPoint> {
        let mut path = Vec::new();
        let QuadbotPoint { quadbots: qbs } = self;
        for (bi, _b) in qbs.into_iter().enumerate() {
            let Point(x, y, k) = qbs[bi];
            for m in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
                let i: i8 = x + m.0;
                let j: i8 = y + m.1;
                let mut keys = k;
                let mut key_point = Point(0, 0, k);
                match input[j as usize][i as usize] {
                    '#' => continue,
                    door @ 'A'..='Z' => {
                        if !key_point.key_present(door.to_ascii_lowercase()) {
                            continue;
                        }
                    }
                    key @ 'a'..='z' => {
                        keys = {
                            key_point.add_key(key);
                            key_point.2
                        }
                    }
                    _ => {}
                }
                if bi == 0 {
                    path.push(QuadbotPoint {
                        quadbots: vec![
                            Point(i, j, keys),
                            Point(qbs[1].0, qbs[1].1, keys),
                            Point(qbs[2].0, qbs[2].1, keys),
                            Point(qbs[3].0, qbs[3].1, keys),
                        ],
                    });
                } else if bi == 1 {
                    path.push(QuadbotPoint {
                        quadbots: vec![
                            Point(qbs[0].0, qbs[0].1, keys),
                            Point(i, j, keys),
                            Point(qbs[2].0, qbs[2].1, keys),
                            Point(qbs[3].0, qbs[3].1, keys),
                        ],
                    });
                } else if bi == 2 {
                    path.push(QuadbotPoint {
                        quadbots: vec![
                            Point(qbs[0].0, qbs[0].1, keys),
                            Point(qbs[1].0, qbs[1].1, keys),
                            Point(i, j, keys),
                            Point(qbs[3].0, qbs[3].1, keys),
                        ],
                    });
                } else if bi == 3 {
                    path.push(QuadbotPoint {
                        quadbots: vec![
                            Point(qbs[0].0, qbs[0].1, keys),
                            Point(qbs[1].0, qbs[1].1, keys),
                            Point(qbs[2].0, qbs[2].1, keys),
                            Point(i, j, keys),
                        ],
                    });
                }
            }
        }
        path
    }
}

impl Location {
    fn successors_pass_locked_doors(&self, input: &Maze) -> Vec<Location> {
        let &Location(x, y) = self;
        let mut path = Vec::new();
        for m in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let i: i8 = x + m.0;
            let j: i8 = y + m.1;
            match input[j as usize][i as usize] {
                '#' => continue,
                '@' => continue,
                _ => {}
            }
            path.push(Location(i, j));
        }
        path
    }
}
