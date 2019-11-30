use num::Integer;
use regex::Regex;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Velocity {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    pub fn new() -> Position {
        Position { x: 0, y: 0, z: 0 }
    }
}

impl Velocity {
    pub fn new() -> Velocity {
        Velocity { x: 0, y: 0, z: 0 }
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct PlanetaryBody {
    position: Position,
    velocity: Velocity,
}

impl PlanetaryBody {
    pub fn new() -> PlanetaryBody {
        PlanetaryBody {
            position: Position::new(),
            velocity: Velocity::new(),
        }
    }
    pub fn potential_energy(&self) -> i32 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }

    pub fn kinetic_energy(&self) -> i32 {
        self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()
    }
}

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Vec<Position> {
    // This is the most annoying data format yet, but I have a feeling I'll need this generator again.
    let re = Regex::new(r"<x=([-0-9]+), y=([-0-9]+), z=([-0-9]+)>").unwrap();
    let mut positions: Vec<Position> = Vec::new();
    for l in input.lines() {
        let caps = re.captures(l.trim()).unwrap();
        let p = Position {
            x: caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            y: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            z: caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
        };
        positions.push(p);
    }
    positions
}

const ITERATIONS: u32 = 1000;

#[aoc(day12, part1)]
pub fn total_energy_all_planets(v: &[Position]) -> u64 {
    let mut bodies: Vec<PlanetaryBody> = Vec::new();
    for position in v {
        let mut body = PlanetaryBody::new();
        body.position = *position;
        bodies.push(body);
    }
    for _i in 1..=ITERATIONS {
        //TODO: Is there a more rustic way to double-iterate over a vec?
        for a in 0..bodies.len() {
            for b in 0..bodies.len() {
                if a == b {
                    continue;
                }
                bodies[a].velocity.x += if bodies[b].position.x > bodies[a].position.x {
                    1
                } else if bodies[b].position.x < bodies[a].position.x {
                    -1
                } else {
                    0
                };
                bodies[a].velocity.y += if bodies[b].position.y > bodies[a].position.y {
                    1
                } else if bodies[b].position.y < bodies[a].position.y {
                    -1
                } else {
                    0
                };
                bodies[a].velocity.z += if bodies[b].position.z > bodies[a].position.z {
                    1
                } else if bodies[b].position.z < bodies[a].position.z {
                    -1
                } else {
                    0
                };
            }
        }
        for a in 0..bodies.len() {
            bodies[a].position.x += bodies[a].velocity.x;
            bodies[a].position.y += bodies[a].velocity.y;
            bodies[a].position.z += bodies[a].velocity.z;
        }
    }

    bodies
        .iter()
        .map(|p| p.potential_energy() as u64 * p.kinetic_energy() as u64)
        .sum()
}

#[aoc(day12, part2)]
pub fn steps_to_repeat(v: &[Position]) -> u64 {
    let mut bodies: Vec<PlanetaryBody> = Vec::new();
    for position in v {
        let mut body = PlanetaryBody::new();
        body.position = *position;
        bodies.push(body);
    }
    let original_bodies = bodies.clone();
    let mut i: u64 = 0;
    let mut x_period: Option<u64> = None;
    let mut y_period: Option<u64> = None;
    let mut z_period: Option<u64> = None;
    loop {
        for a in 0..bodies.len() {
            for b in 0..bodies.len() {
                if a == b {
                    continue;
                }
                bodies[a].velocity.x += if bodies[b].position.x > bodies[a].position.x {
                    1
                } else if bodies[b].position.x < bodies[a].position.x {
                    -1
                } else {
                    0
                };
                bodies[a].velocity.y += if bodies[b].position.y > bodies[a].position.y {
                    1
                } else if bodies[b].position.y < bodies[a].position.y {
                    -1
                } else {
                    0
                };
                bodies[a].velocity.z += if bodies[b].position.z > bodies[a].position.z {
                    1
                } else if bodies[b].position.z < bodies[a].position.z {
                    -1
                } else {
                    0
                };
            }
        }

        for a in 0..bodies.len() {
            bodies[a].position.x += bodies[a].velocity.x;
            bodies[a].position.y += bodies[a].velocity.y;
            bodies[a].position.z += bodies[a].velocity.z;
        }
        i += 1;
        if bodies[0].position.x == original_bodies[0].position.x
            && bodies[1].position.x == original_bodies[1].position.x
            && bodies[2].position.x == original_bodies[2].position.x
            && bodies[3].position.x == original_bodies[3].position.x
            && x_period.is_none()
        {
            x_period = Some(i);
        }
        if bodies[0].position.y == original_bodies[0].position.y
            && bodies[1].position.y == original_bodies[1].position.y
            && bodies[2].position.y == original_bodies[2].position.y
            && bodies[3].position.y == original_bodies[3].position.y
            && y_period.is_none()
        {
            y_period = Some(i);
        }
        if bodies[0].position.z == original_bodies[0].position.z
            && bodies[1].position.z == original_bodies[1].position.z
            && bodies[2].position.z == original_bodies[2].position.z
            && bodies[3].position.z == original_bodies[3].position.z
            && z_period.is_none()
        {
            z_period = Some(i);
        }

        if x_period.is_some() && y_period.is_some() && z_period.is_some() {
            break;
        }
    }
    // Positions repeat one step at the calculated period because velocity is 0. Add 1 to each to compensate.
    (x_period.unwrap() + 1).lcm(&(y_period.unwrap() + 1).lcm(&(z_period.unwrap() + 1)))
}

#[cfg(test)]
mod tests {
    use day12::generator;
    use day12::steps_to_repeat;
    use day12::total_energy_all_planets;
    use std::fs;
    const ANSWER_12A: u64 = 9139;
    const ANSWER_12B: u64 = 420_788_524_631_496;

    #[test]
    fn t12a() {
        assert_eq!(
            ANSWER_12A,
            total_energy_all_planets(&generator(
                &fs::read_to_string("input/2019/day12.txt").unwrap().trim()
            ))
        );
    }
    #[test]
    fn t12b() {
        assert_eq!(
            ANSWER_12B,
            steps_to_repeat(&generator(
                &fs::read_to_string("input/2019/day12.txt").unwrap().trim()
            ))
        );
    }
}
