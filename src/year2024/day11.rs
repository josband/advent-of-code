use std::{collections::HashMap, fs::read_to_string};

use color_print::cprintln;

type Input = Vec<u64>;

pub fn day_11() {
    let input = parse();
    part1(&input);
    part2(&input);
}

fn parse() -> Input {
    read_to_string("inputs/2024/day11.txt").unwrap().split_whitespace().map(|num| num.parse().unwrap()).collect()
}

fn part1(input: &Input) {
    let mut result = 0;
    for num in input {
        result += get_stones_produced(*num, 25);
    }

    cprintln!("Stones after 25 blinks: <b>{}</>", result);
}

fn get_stones_produced(stone_value: u64, blinks_left: usize) -> u64 {
    if blinks_left == 0 {
        1
    } else if stone_value == 0 {
        get_stones_produced(1, blinks_left - 1)
    } else if num_digits(stone_value) % 2 == 0 {
        let (left, right) = split_num(stone_value);
        get_stones_produced(left, blinks_left - 1) + get_stones_produced(right, blinks_left - 1)
    } else {
        get_stones_produced(stone_value * 2024, blinks_left - 1)
    }
}

fn split_num(num: u64) -> (u64, u64) {
    let digit_count = num_digits(num);
    (num / 10_u64.pow(digit_count/2), num % 10_u64.pow(digit_count/2))
}

fn num_digits(num: u64) -> u32 {
    num.ilog10() + 1
}

fn part2(input: &Input) {
    let mut mem = HashMap::new();
    let mut result = 0;
    for num in input {
        result += get_stones_produced_with_memoization(*num, 75, &mut mem);
    }

    cprintln!("Stones after 75 blinks: <b>{}</>", result);
}

fn get_stones_produced_with_memoization(stone_value: u64, blinks_left: usize, mem: &mut HashMap<usize, HashMap<u64, u64>>) -> u64 {
    if !mem.contains_key(&blinks_left) {
        mem.insert(blinks_left, HashMap::new());
    }

    let res = if blinks_left == 0 {
        1
    } else if mem.get(&blinks_left).unwrap().contains_key(&stone_value) {
        *mem.get(&blinks_left).unwrap().get(&stone_value).unwrap()
    } else if stone_value == 0 {
        get_stones_produced_with_memoization(1, blinks_left - 1, mem)
    } else if num_digits(stone_value) % 2 == 0 {
        let (left, right) = split_num(stone_value);
        get_stones_produced_with_memoization(left, blinks_left - 1, mem) 
        + get_stones_produced_with_memoization(right, blinks_left - 1, mem)
    } else {
        get_stones_produced_with_memoization(stone_value * 2024, blinks_left - 1, mem)
    };
    
    // println!("{blinks_left} {stone_value} {res}");
    mem.get_mut(&blinks_left).unwrap().insert(stone_value, res);
    res
}