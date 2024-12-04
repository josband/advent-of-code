use std::{collections::HashMap, fs::read_to_string};

use color_print::cprintln;

pub fn day_01() {
    part1();
    part2();
    println!();
}

fn parse() -> (Vec<usize>, Vec<usize>) {
    let (mut list_one, mut list_two) = (vec![], vec![]);
    read_to_string("./inputs/day01.txt").expect("Can't open 2024 Day 1").lines().for_each(|line| {
        let v: Vec<&str> = line.split_whitespace().collect();
        assert!(v.len() == 2);
        list_one.push(v[0].parse().unwrap());
        list_two.push(v[1].parse().unwrap());
    });

    (list_one, list_two)
}

fn part1() {
    let (mut list_one, mut list_two) = parse();

    list_one.sort();
    list_two.sort();

    let distance: usize = list_one.into_iter().zip(list_two).map(|(a, b)| a.abs_diff(b)).sum();

    cprintln!("The total distance is: <b>{}</>", distance);
}

fn part2() {
    let (list_one, list_two) = parse();

    let mut count_map: HashMap<usize, usize> = HashMap::new();
    list_two.iter().for_each(|&num| {
        count_map.insert(num, count_map.get(&num).unwrap_or(&0) + 1);
    });

    let similarity_score: usize = list_one.iter().filter_map(|num| count_map.get(num).map(|count| num * count)).sum();

    cprintln!("The similarity score is: <b>{}</>", similarity_score);
}