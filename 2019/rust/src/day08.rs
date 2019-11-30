const WIDTH: usize = 25;
const HEIGHT: usize = 6;

#[aoc(day08, part1, original)]
pub fn original_8a(input: &str) -> u32 {
    let s: Vec<u32> = input
        .trim()
        .chars()
        .map(|n| n.to_string().parse::<u32>().unwrap())
        .collect();
    let layers = s.len() / (WIDTH * HEIGHT);
    let mut s_iter = s.iter();
    let mut v: Vec<Vec<Vec<u32>>> = vec![vec![vec![0; WIDTH]; HEIGHT]; layers];
    let mut dc: Vec<DigitCount> = vec![
        DigitCount {
            d0: 0,
            d1: 0,
            d2: 0
        };
        layers
    ];
    for l in 0..layers {
        for r in 0..HEIGHT {
            for c in 0..WIDTH {
                let digit = *s_iter.next().unwrap();
                v[l][r][c] = digit;
                match digit {
                    0 => dc[l].d0 += 1,
                    1 => dc[l].d1 += 1,
                    2 => dc[l].d2 += 1,
                    _ => panic!(),
                }
            }
        }
    }
    let best_layer = dc.iter().min_by(|x, y| x.d0.cmp(&y.d0)).unwrap();
    best_layer.d1 * best_layer.d2
}

#[derive(Clone)]
struct DigitCount {
    d0: u32,
    d1: u32,
    d2: u32,
}

#[derive(Clone, Copy, Debug, std::cmp::PartialEq)]
enum PixelColor {
    Black,
    White,
    Transparent,
}

#[aoc(day08, part2, original)]
pub fn original_8b(input: &str) -> String {
    let s: Vec<u32> = input
        .trim()
        .chars()
        .map(|n| n.to_string().parse::<u32>().unwrap())
        .collect();
    let layers = s.len() / (WIDTH * HEIGHT);
    let mut s_iter = s.iter();
    let mut v: Vec<Vec<Vec<u32>>> = vec![vec![vec![0; WIDTH]; HEIGHT]; layers];
    let mut pixel: Vec<Vec<PixelColor>> = vec![vec![PixelColor::Transparent; WIDTH]; HEIGHT];
    for l in 0..layers {
        for r in 0..HEIGHT {
            for c in 0..WIDTH {
                let digit = *s_iter.next().unwrap();
                v[l][r][c] = digit;
                pixel[r][c] = match digit {
                    0 => layer_pixel(pixel[r][c], PixelColor::Black),
                    1 => layer_pixel(pixel[r][c], PixelColor::White),
                    2 => layer_pixel(pixel[r][c], PixelColor::Transparent),
                    _ => unreachable!(),
                }
            }
        }
    }

    for r in 0..HEIGHT {
        for c in 0..WIDTH {
            match pixel[r][c] {
                PixelColor::Black => print!(" "),
                PixelColor::White => print!("X"),
                _ => unreachable!(),
            }
        }
        println!();
    }
    "PFCAK".to_string()
}

fn layer_pixel(old: PixelColor, new: PixelColor) -> PixelColor {
    if old == PixelColor::Black || old == PixelColor::White {
        old
    } else {
        new
    }
}

#[cfg(test)]
mod tests {
    use day08::original_8a;
    use day08::original_8b;
    use std::fs;
    const ANSWER_8A: u32 = 1064;
    const ANSWER_8B: &str = "PFCAK";

    #[test]
    fn t08a() {
        assert_eq!(
            ANSWER_8A,
            original_8a(&fs::read_to_string("input/2019/day8.txt").unwrap().trim())
        );
    }
    #[test]
    fn t08b() {
        assert_eq!(
            ANSWER_8B,
            original_8b(&fs::read_to_string("input/2019/day8.txt").unwrap().trim())
        );
    }
}
