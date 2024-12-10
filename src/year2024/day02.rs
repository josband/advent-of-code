use std::{fs::File, io::{BufRead, BufReader}};

use color_print::cprintln;

pub fn day_02() {
    part1();
    part2();
}

fn parse() -> Vec<Vec<i32>> {
    let input = File::open("./inputs/2024/day02.txt");
    if let Err(e) = input {
        panic!("Failed to open 2024 day 2 input {:?}", e);
    }

    BufReader::new(input.unwrap()).lines().map(|line| {
        match line {
            Ok(content) => {
                content.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>()
            },
            _ => panic!("Failed to parse")
        }
    }).collect()
}

fn part1() {
    let reports = parse();
    let result = reports.iter().filter(|&nums| is_safe(nums)).count();

    cprintln!("Total safe reports: <b>{}</>", result);
}

fn is_safe(nums: &Vec<i32>) -> bool {
    if nums.len() <= 1 {
        return true;
    }
    let increasing = nums[1] > nums[0];

    for pair in nums.windows(2) {
        if is_level_unsafe(increasing, pair[0], pair[1]) {
            return false;
        }
    }
    
    true
}

fn part2() {
    let reports = parse();
    let result = reports.iter().filter(|&report| {
        if is_safe(report) {
            return true;
        }

        for index in 0..report.len() {
            let mut cloned_report = report.clone();
            cloned_report.remove(index);
            if is_safe(&cloned_report) {
                return true;
            }
        }     

        false
    }).count();
    
    cprintln!("Total safe reports: <b>{}</>", result);
}

fn is_level_unsafe(increasing: bool, left: i32, right:i32) -> bool {
    (increasing && right <= left) || (!increasing && right >= left) || right.abs_diff(left) > 3
}