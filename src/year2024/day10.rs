use std::{collections::HashSet, fs::read_to_string};

use color_print::cprintln;

type Input = Vec<Vec<u32>>;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn day_10() {
    let input = parse();
    part1(&input);
    part2(&input);
}

fn parse() -> Input {
    read_to_string("inputs/2024/day10.txt").unwrap().lines()
        .map(|line| line.chars().map(|height| height.to_digit(10).unwrap()).collect()).collect()
}

fn part1(input: &Input) {
    let mut result = 0;
    for (row, cols) in input.iter().enumerate() {
        for (col, height) in cols.iter().enumerate() {
            if *height == 0 {
                let mut seen = HashSet::new();
                result += get_trailhead_score(input, &mut seen, row, col, 0);
            }
        }
    }

    cprintln!("Sum of trailhead scores: <b>{}</>", result);
}

fn get_trailhead_score(input: &Input, seen: &mut HashSet<(usize, usize)>, row: usize, col: usize, height: u32) -> i32 {
    if input[row][col] == 9 {
        seen.insert((row, col));
        return 1;
    }

    let mut score = 0;
    for (dr, dc) in DIRS {
        let (new_row, new_col) = (row as isize + dr, col as isize + dc);
        if !seen.contains(&(new_row as usize, new_col as usize)) && is_in_bounds(new_row, new_col, input[0].len(), input.len())
        && input[new_row as usize][new_col as usize] == height + 1 {
            score += get_trailhead_score(input, seen, new_row as usize, new_col as usize, height + 1);
        }
    }

    score
}

fn is_in_bounds(row: isize, col: isize, width: usize, height: usize) -> bool {
    row >= 0 && col >= 0 && row < height as isize && col < width as isize
}

fn part2(input: &Input) {
    let mut result = 0;
    for (row, cols) in input.iter().enumerate() {
        for (col, height) in cols.iter().enumerate() {
            if *height == 0 {
                result += get_trailhead_rating(input, row, col, 0);
            }
        }
    }

    cprintln!("Sum of trailhead scores: <b>{}</>", result);
}

fn get_trailhead_rating(input: &Input,  row: usize, col: usize, height: u32) -> i32 {
    if input[row][col] == 9 {
        return 1;
    }

    let mut score = 0;
    for (dr, dc) in DIRS {
        let (new_row, new_col) = (row as isize + dr, col as isize + dc);
        if is_in_bounds(new_row, new_col, input[0].len(), input.len())
        && input[new_row as usize][new_col as usize] == height + 1 {
            score += get_trailhead_rating(input, new_row as usize, new_col as usize, height + 1);
        }
    }

    score
}