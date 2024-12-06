use std::fs::read_to_string;
use color_print::cprintln;
use regex::Regex;

pub fn day_03() {
    part1();
    part2();
}

fn parse_all_enabled() -> Vec<(i32, i32)> {
    read_to_string("./inputs/day03.txt").expect("Cannot open 2024 day 3")
        .lines()
        .map(|line| {
            let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
            re.captures_iter(line).map(|capture| {
                (capture[1].parse().unwrap(), capture[2].parse().unwrap())
            }).collect::<Vec<_>>()
        }).flatten().collect()
}

fn parse_dos_and_donts() -> Vec<(i32, i32)> {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut is_enabled = true;
    re.captures_iter(&read_to_string("./inputs/day03.txt").expect("Cannot open 2024 day 3")).filter_map(|capture| {
        // println!("{}", &capture[0]);
        if &capture[0] == "do()" {
            // println!("Do");
            is_enabled = true;
        } else if &capture[0] == "don't()" {
            // println!("Don't");
            is_enabled = false;
        } else if is_enabled {
            // println!("{}", &capture[0]);
            return Some((capture[1].parse().unwrap(), capture[2].parse().unwrap()));
        }

        None
    }).collect::<Vec<_>>()
}

fn part1() {
    let result: i32 = parse_all_enabled().iter().map(|&(a, b)| a * b).sum();
    cprintln!("The sum of all multiplications is: <b>{}</>", result);
}

fn part2() {
    let result: i32 = parse_dos_and_donts().iter().map(|&(a, b)| a * b).sum();
    cprintln!("The sum of all multiplications is: <b>{}</>", result);
}