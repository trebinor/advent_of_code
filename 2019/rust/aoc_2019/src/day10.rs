use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day10)]
pub fn generator(input: &str) -> AsteroidField {
    let v = input.lines().map(|l| l.trim()).collect::<Vec<&str>>();
    let mut asteroid_field: AsteroidField = AsteroidField::new();
    for (i, l) in v.iter().enumerate() {
        for (j, x) in l.chars().enumerate() {
            match x {
                '.' => (),
                '#' => asteroid_field.push(Point { x: j, y: i }),
                _ => unreachable!(),
            }
        }
    }
    asteroid_field
}

#[aoc(day10, part1)]
pub fn visible_asteroids(asteroid_field: &[Point]) -> u32 {
    let mut asteroid_sight_map = AsteroidSightMap::new();
    let best_point = &asteroid_sight_map.find_best_point(asteroid_field);
    if let Some(asteroids_visible_by_best_point) = &asteroid_sight_map.get(&best_point) {
        // how many asteroids for this point?
        asteroids_visible_by_best_point.len() as u32
    } else {
        // None, so 0 asteroids
        0
    }
}

#[aoc(day10, part2)]
pub fn math_on_200th_asteroid(asteroid_field: &[Point]) -> u32 {
    let mut asteroid_sight_map = AsteroidSightMap::new();
    let p = &asteroid_sight_map.find_best_point(asteroid_field); // answer from part 1
    let mut theta: f64 = 3.0 * std::f64::consts::FRAC_PI_2;
    let mut h: HashMap<u64, Vec<Point>> = HashMap::new(); // map of angles relative to our vantage point to list of other points.  The key is a bit representation of f64, since it can't be hashed.
    let mut asteroids_the_game = asteroid_field.to_vec().clone();
    let mut this_asteroid: Point = *p;
    for a in &asteroids_the_game {
        if a == p {
            continue; // don't shoot yourself
        }
        let dx = a.x as f64 - p.x as f64;
        let dy = a.y as f64 - p.y as f64;
        let hash_angle = if dy.atan2(dx) < 0.0 {
            dy.atan2(dx) + (std::f64::consts::PI * 2.0)
        } else {
            dy.atan2(dx)
        };
        h.entry(hash_angle.to_bits())
            .and_modify(|e| e.push(*a))
            .or_insert({
                let mut v = Vec::new();
                v.push(*a);
                v
            });
    }
    for _i in 1..=200 {
        // if there's an asteroid with this angle, destroy the one with the smallest Euclidean distance
        let asteroids_on_this_angle = h.get(&theta.to_bits());
        if asteroids_on_this_angle.is_some() {
            let min = asteroids_on_this_angle
                .unwrap()
                .iter()
                .min_by(|x, y| x.distance(p).partial_cmp(&y.distance(p)).unwrap());
            if min.is_some() {
                this_asteroid = *min.unwrap();
                asteroids_the_game
                    .iter_mut()
                    .position(|item| item == min.unwrap())
                    .map(|index| asteroids_the_game.remove(index)); // see https://github.com/rust-lang/rust/issues/40062#issuecomment-480060761
            } else {
                panic!();
            }
        }
        let angles: Vec<f64> = h.keys().map(|t| f64::from_bits(*t)).collect();

        let mut closest_clockwise_angle = 10.0; // larger than possible
        let mut smallest_positive_angle = 10.0;
        for a in angles {
            if a > theta && a < closest_clockwise_angle {
                closest_clockwise_angle = a;
            }
            if a < smallest_positive_angle {
                smallest_positive_angle = a;
            }
        }

        if closest_clockwise_angle == 10.0 {
            theta = smallest_positive_angle;
        } else {
            theta = closest_clockwise_angle;
        }
    }
    (this_asteroid.x as u32) * 100 + this_asteroid.y as u32
}

#[derive(Debug, Eq, Hash, Clone, Copy)]
pub struct Point {
    x: usize,
    y: usize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        ((other.x as f64 - self.x as f64).powf(2.0) + (other.y as f64 - self.y as f64).powf(2.0))
            .sqrt()
    }
}

trait PointAnalysis {
    fn find_best_point(&mut self, _: &[Point]) -> Point;
}

impl PointAnalysis for AsteroidSightMap {
    fn find_best_point(&mut self, asteroid_field: &[Point]) -> Point {
        // Create keys in the sight map for all asteroids in the field
        for a in asteroid_field {
            self.entry(*a).or_default();
        }

        // Apply algorithm to each asteroid and update sight map
        for source in asteroid_field {
            for target in asteroid_field {
                if source == target {
                    continue;
                }
                let dx = target.x as i32 - source.x as i32;
                let dy = target.y as i32 - source.y as i32;
                self.entry(*source).and_modify(|e| {
                    e.insert((dy as f64).atan2(dx as f64).to_bits());
                });
            }
        }

        *self
            .keys()
            .max_by(|x, y| self.get(x).unwrap().len().cmp(&self.get(y).unwrap().len()))
            .unwrap()
    }
}

type AsteroidSightMap = HashMap<Point, HashSet<u64>>;
type AsteroidField = Vec<Point>;

#[cfg(test)]
mod tests {
    use crate::day10::generator;
    use crate::day10::AsteroidSightMap;
    use crate::day10::Point;
    use crate::day10::PointAnalysis;
    use std::fs;

    const UNIT_ANSWER_10A_0: Point = Point { x: 5, y: 8 };
    const UNIT_ANSWER_10A_1: Point = Point { x: 1, y: 2 };
    const UNIT_ANSWER_10A_2: Point = Point { x: 6, y: 3 };
    const UNIT_ANSWER_10A_3: Point = Point { x: 11, y: 13 };

    #[test]
    fn t10a_unit_example0() {
        assert_eq!(
            UNIT_ANSWER_10A_0,
            AsteroidSightMap::new().find_best_point(&generator(&fs::read_to_string("input/2019/10/example0").unwrap().trim()))
        );
    }
    #[test]
    fn t10a_unit_example1() {
        assert_eq!(
            UNIT_ANSWER_10A_1,
            AsteroidSightMap::new().find_best_point(&generator(&fs::read_to_string("input/2019/10/example1").unwrap().trim()))
        );
    }
    #[test]
    fn t10a_unit_example2() {
        assert_eq!(
            UNIT_ANSWER_10A_2,
            AsteroidSightMap::new().find_best_point(&generator(&fs::read_to_string("input/2019/10/example2").unwrap().trim()))
        );
    }
    #[test]
    fn t10a_unit_example3() {
        assert_eq!(
            UNIT_ANSWER_10A_3,
            AsteroidSightMap::new().find_best_point(&generator(&fs::read_to_string("input/2019/10/example3").unwrap().trim()))
        );
    }
}
