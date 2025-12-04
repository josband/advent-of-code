use std::{fs::read_to_string, str::FromStr};

use color_print::cprintln;

pub fn day_03() {
    let input = parse();
    part1(&input);
    part2(&input);
}

fn parse() -> Vec<Vec<usize>> {
    read_to_string("./inputs/2025/day03.txt")
        .expect("Can't read 2025 Day 3")
        .lines()
        .map(get_digits)
        .collect()
}

fn get_digits(number: &str) -> Vec<usize> {
    number
        .chars()
        .map(|char| usize::from_str(&char.to_string()).unwrap())
        .collect()
}

fn part1(batteries: &[Vec<usize>]) {
    let mut joltage = 0;
    for digits in batteries {
        let (mut hi_digit, mut lo_digit) = (0, 1);
        let mut curr = 1;
        while curr < digits.len() {
            let digit = digits[curr];
            if curr != digits.len() - 1 && digit > digits[hi_digit] {
                hi_digit = curr;
                lo_digit = curr + 1;
            } else if digit > digits[lo_digit] {
                lo_digit = curr;
            }

            curr += 1;
        }

        joltage += digits[hi_digit] * 10 + digits[lo_digit];
    }

    cprintln!("Total joltage: <b>{}</>", joltage);
}

const ENABLED_BATTERIES: usize = 12;

fn part2(batteries: &[Vec<usize>]) {
    let mut joltage = 0;
    for digits in batteries {
        let mut activated_batteries = vec![];
        let mut last_selected_index = usize::MAX; // Ugly, but use wrapping
        for n in 0..ENABLED_BATTERIES {
            let mut curr = last_selected_index.wrapping_add(1);
            let mut max_index = curr;
            // println!(
            //     "{} {} {}",
            //     n,
            //     digits.len(),
            //     digits.len() - (ENABLED_BATTERIES - n - 1)
            // );
            while curr < digits.len() - (ENABLED_BATTERIES - n - 1) {
                // println!("{}", digits[curr]);
                if digits[curr] > digits[max_index] {
                    max_index = curr;
                }

                curr += 1;
            }

            activated_batteries.push(digits[max_index]);
            last_selected_index = max_index;
        }

        // println!("{}", get_joltage(&activated_batteries));
        joltage += get_joltage(&activated_batteries);
    }

    cprintln!("Total Joltage: <b>{}</>", joltage);
}

fn get_joltage(batteries: &Vec<usize>) -> usize {
    let mut res = 0;
    for &battery in batteries {
        res = res * 10 + battery;
    }

    res
}
