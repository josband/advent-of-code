use std::{fs::read_to_string, str::FromStr};

use color_print::cprintln;

const TICKS: i64 = 100;

pub fn day_01() {
    let input = parse();
    part1(&input);
    part2(&input);
}

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn get_net_effect(&self, val: i64) -> i64 {
        val * match self {
            Direction::Right => 1,
            Direction::Left => -1,
        }
    }
}

impl FromStr for Direction {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid Input",
            )),
        }
    }
}

struct Rotation {
    dir: Direction,
    dist: i64,
}

impl Rotation {
    fn do_rotation(&self, val: i64) -> i64 {
        (val + self.dir.get_net_effect(self.dist)).rem_euclid(TICKS)
    }
}

fn parse() -> Vec<Rotation> {
    read_to_string("./inputs/2025/day01.txt")
        .expect("Can't read Day 1 of 2025")
        .lines()
        .map(|line| {
            let (dir, count) = line.split_at(1);
            Rotation {
                dir: Direction::from_str(dir).unwrap(),
                dist: i64::from_str(count).unwrap(),
            }
        })
        .collect()
}

fn part1(rotations: &[Rotation]) {
    let mut count = 0;
    let mut curr = 50;

    rotations.iter().for_each(|r| {
        curr = r.do_rotation(curr);
        if curr == 0 {
            count += 1;
        }
    });

    cprintln!("Total times 0 is reached: <b>{}</>", count);
}

fn part2(rotations: &[Rotation]) {
    let mut count: i64 = 0;
    let mut curr = 50;
    rotations.iter().for_each(|r| {
        let net = r.dir.get_net_effect(r.dist);
        let sum = curr + net;
        count += sum.abs() / TICKS;
        if sum <= 0 && net != sum {
            count += 1;
        }

        curr = r.do_rotation(curr);
    });

    cprintln!("Total times 0 is crossed: <b>{}</>", count);
}
