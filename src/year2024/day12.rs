use std::{collections::HashSet, fs::read_to_string};

use color_print::cprintln;

type Input = Vec<Vec<char>>;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn day_12() {
    let input = parse();
    part1(&input);
    part2(&input);
}

fn parse() -> Input {
    read_to_string("inputs/2024/day12.txt").unwrap().lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &Input) {
    let mut seen = HashSet::new();
    let mut regions = Vec::new();
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if !seen.contains(&(row, col)) {
                regions.push(traverse_region(&mut seen, input, row, col));
            }
        } 
    }

    let result: usize = regions.iter().map(|(perimeter, area)| *perimeter * *area).sum();
    cprintln!("Total price: <b>{}</>", result);
}

fn traverse_region(seen: &mut HashSet<(usize, usize)>, map: &Input, row: usize, col: usize) -> (usize, usize) {
    seen.insert((row, col));
    let (mut area, mut perimeter) = (0, 0);
    for (dr, dc) in DIRS {
        let (next_r, next_c) = (row as isize + dr, col as isize + dc);
        if is_in_bounds(next_r, next_c, map.len(), map[0].len()) && !seen.contains(&(next_r as usize, next_c as usize))
        && map[row][col] == map[next_r as usize][next_c as usize] {
            let next_step = traverse_region(seen, map, next_r as usize, next_c as usize);
            (area, perimeter) = (area + next_step.0, perimeter + next_step.1);
        } else if !is_in_bounds(next_r, next_c, map.len(), map[0].len()) || map[row][col] != map[next_r as usize][next_c as usize] {
            perimeter = perimeter + 1;
        }
    }

    (area + 1, perimeter)
}

fn is_in_bounds(row: isize, col: isize, height: usize, width: usize) -> bool {
    row >= 0 && col >= 0 && row < height as isize && col < width as isize
}

fn part2(input: &Input) {
    let mut seen = HashSet::new();
    let mut regions = Vec::new();
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if !seen.contains(&(row, col)) {
                let mut edges = HashSet::new();
                let area = traverse_region_with_sides(&mut seen, &mut edges, input, row, col);
                let pair = (area, count_edges(&mut edges));
                regions.push(pair);
            }
        } 
    }

    let result: usize = regions.iter().map(|(perimeter, area)| *perimeter * *area).sum();
    cprintln!("Total price: <b>{}</>", result);
}

fn traverse_region_with_sides(seen: &mut HashSet<(usize, usize)>, edges: &mut HashSet<[isize; 3]>, map: &Input, row: usize, col: usize) -> usize {
    seen.insert((row, col));
    let mut area = 0;
    for (index, (dr, dc)) in DIRS.iter().enumerate() {
        let (next_r, next_c) = (row as isize + dr, col as isize + dc);
        if is_in_bounds(next_r, next_c, map.len(), map[0].len()) && !seen.contains(&(next_r as usize, next_c as usize))
        && map[row][col] == map[next_r as usize][next_c as usize] {
            area += traverse_region_with_sides(seen, edges, map, next_r as usize, next_c as usize);
        } else if !is_in_bounds(next_r, next_c, map.len(), map[0].len()) || map[row][col] != map[next_r as usize][next_c as usize] {
            edges.insert([row as isize, col as isize, index as isize]);
        }
    }

    area + 1
}

fn count_edges(edges: &mut HashSet<[isize; 3]>) -> usize {
    let mut result = 0;
    let mut seen_edges = HashSet::new();
    for &[row, col, dir] in edges.iter() {
        if seen_edges.contains(&[row, col, dir]) {
            continue;
        }

        result += 1;
        seen_edges.insert([row, col, dir]);
        add_adjacent_edges(row, col, dir, edges, &mut seen_edges);
    }

    result
}

fn add_adjacent_edges(row: isize, col: isize, dir: isize, edges: &HashSet<[isize; 3]>, seen_edges: &mut HashSet<[isize; 3]>) {
    let (dr, dc) = if dir % 2 == 0 { (0, 1) } else { (1, 0) };

    let (mut curr_row, mut curr_col) = (row, col);
    while !seen_edges.contains(&[curr_row + dr, curr_col + dc, dir]) && edges.contains(&[curr_row + dr, curr_col + dc, dir]) {
        curr_row += dr;
        curr_col += dc;
        seen_edges.insert([curr_row, curr_col, dir]);
    }

    let (mut curr_row, mut curr_col) = (row, col);
    while !seen_edges.contains(&[curr_row - dr, curr_col - dc, dir]) && edges.contains(&[curr_row - dr, curr_col - dc, dir]) {
        curr_row -= dr;
        curr_col -= dc;
        seen_edges.insert([curr_row, curr_col, dir]);
    }
}