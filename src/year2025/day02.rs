use std::{collections::HashSet, fs::read_to_string, str::FromStr};

use color_print::cprintln;

pub fn day_02() {
    let input = parse();
    part1(&input);
    part2(&input);
}

fn parse() -> Vec<(u64, u64)> {
    read_to_string("./inputs/2025/day02.txt")
        .expect("Can't read 2025 day 2")
        .trim()
        .split(",")
        .map(|s| {
            let (lo, hi) = s.split_once("-").unwrap();
            (u64::from_str(lo).unwrap(), u64::from_str(hi).unwrap())
        })
        .collect()
}

fn part1(ranges: &[(u64, u64)]) {
    let mut sum: u64 = 0;
    for &(lo, hi) in ranges.iter() {
        for n in lo..=hi {
            let str_num = n.to_string();
            let (h1, h2) = str_num.split_at(str_num.len() / 2);
            if h1 == h2 {
                sum += n;
            }
        }
    }

    cprintln!("Sum of invalid IDs: <b>{}</>", sum);
}

fn part2(ranges: &[(u64, u64)]) {
    let mut sum: u64 = 0;
    for &(lo, hi) in ranges.iter() {
        for n in lo..=hi {
            let str_num = n.to_string();
            'a: for i in 1..str_num.len() {
                let (num, mut remainder) = str_num.split_at(i);
                while !remainder.is_empty() && remainder.starts_with(num) {
                    remainder = remainder.strip_prefix(num).unwrap();
                }

                if remainder.is_empty() {
                    sum += n;
                    break 'a;
                }
            }
        }
    }

    cprintln!("Sum of invalid IDs: <b>{}</>", sum);
}
