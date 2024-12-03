use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

use color_print::cprintln;

pub fn day_01() {
    part1();
    part2();
    println!();
}

fn parse(list_one: &mut Vec<usize>, list_two: &mut Vec<usize>) {
    let input = File::open("./inputs/day01.txt");
    if let Err(e) = input {
        panic!("Error opening input file {:?}", e);
    }

    let reader = BufReader::new(input.expect("Failed to open file"));
    reader.lines()
        .filter_map(|line| {
            line.ok().and_then(|content| {
                let parts: Vec<&str> = content.split_whitespace().collect();
                if parts.len() == 2 {
                    Some((
                        parts[0].parse().ok(),
                        parts[1].parse().ok()
                    ))
                } else {
                    None
                }
            })
        })
        .for_each(|(a, b)| {
            if let (Some(a), Some(b)) = (a, b) {
                list_one.push(a);
                list_two.push(b);
            }
        });
}

fn part1() {
    let mut list_one: Vec<usize> = Vec::new();
    let mut list_two : Vec<usize>= Vec::new();

    parse(&mut list_one, &mut list_two);

    list_one.sort();
    list_two.sort();

    let result = list_one.into_iter().zip(list_two)
        .fold(0, |acc, pair| {
            acc + (pair.0.abs_diff(pair.1))
        });

    cprintln!("The total distance is: <b>{}</>", result);
}

fn part2() {
    let mut list_one: Vec<usize> = Vec::new();
    let mut list_two : Vec<usize>= Vec::new();

    parse(&mut list_one, &mut list_two);

    let mut count_map: HashMap<usize, usize> = HashMap::new();
    list_two.iter().for_each(|&num| {
        count_map.insert(num, count_map.get(&num).unwrap_or(&0) + 1);
    });

    let similarity_score = list_one.iter()
        .map(|&num| count_map.get(&num).unwrap_or(&0) * num)
        .fold(0, |acc, num| acc + num);

    cprintln!("The total distance is: <b>{}</>", similarity_score);
}