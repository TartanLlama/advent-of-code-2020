use num_complex::Complex;
use std::convert::{TryFrom, TryInto};

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl TryFrom<i32> for Direction {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Direction::East),
            90 => Ok(Direction::South),
            180 => Ok(Direction::West),
            270 => Ok(Direction::North),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(v: char) -> Result<Self, Self::Error> {
        match v {
            'E' => Ok(Direction::East),
            'S' => Ok(Direction::South),
            'W' => Ok(Direction::West),
            'N' => Ok(Direction::North),
            _ => Err(()),
        }
    }
}

fn turn(facing: i32, delta: i32) -> i32 {
    (facing + delta + 360) % 360
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let (mut x, mut y) = (0, 0);
        let mut facing = 0;
        for (dir_str, val_str) in include_str!("../input/day12.txt")
            .lines()
            .map(|line| line.split_at(1))
        {
            let val = val_str.parse().unwrap();
            let dir_char = dir_str.chars().nth(0).unwrap();

            let to_move: Option<Direction> = match dir_char {
                'E' | 'W' | 'N' | 'S' => dir_char.try_into().ok(),
                'F' => facing.try_into().ok(),
                'R' => {
                    facing = turn(facing, val);
                    None
                }
                'L' => {
                    facing = turn(facing, -val);
                    None
                }
                _ => panic!("bad instruction"),
            };

            match to_move {
                Some(direction) => match direction {
                    Direction::East => x += val,
                    Direction::West => x -= val,
                    Direction::North => y += val,
                    Direction::South => y -= val,
                },
                None => (),
            }
        }

        println!("Solution: {}", x.abs() + y.abs());
    }

    #[test]
    fn part2() {
        let mut ship = Complex::new(0, 0);
        let mut waypoint = Complex::new(10, 1);
        let translation_map: Vec<_> = [(1, 0), (0, -1), (-1, 0), (0, 1)]
            .iter()
            .map(|&(x, y)| Complex::new(x, y))
            .collect();

        for (dir_str, val_str) in include_str!("../input/day12.txt")
            .lines()
            .map(|line| line.split_at(1))
        {
            let val: i32 = val_str.parse().unwrap();
            let dir_char = dir_str.chars().nth(0).unwrap();

            match dir_char {
                'E' => waypoint += Complex::new(val, 0),
                'W' => waypoint -= Complex::new(val, 0),
                'N' => waypoint += Complex::new(0, val),
                'S' => waypoint -= Complex::new(0, val),
                'F' => ship += val * waypoint,
                'R' => waypoint *= translation_map[val as usize / 90],
                'L' => waypoint *= translation_map[(4 - (val as usize / 90)) as usize],
                _ => panic!("bad instruction"),
            };
        }

        println!("Solution: {}", ship.re.abs() + ship.im.abs());
    }
}
