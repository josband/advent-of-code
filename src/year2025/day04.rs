use std::{collections::HashSet, fs::read_to_string};

use color_print::cprintln;

const TP: &str = "@";

const DIRS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (-1, -1),
    (0, -1),
    (0, 1),
    (1, 0),
    (1, -1),
    (1, 1),
];

pub fn day_04() {
    let input = parse();
    part1(&input);
    part2(&input);
}

fn parse() -> Vec<Vec<String>> {
    read_to_string("./inputs/2025/day04.txt")
        .expect("Can't read 2025 Day 4")
        .lines()
        .map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<_>>())
        .collect()
}

fn part1(map: &[Vec<String>]) {
    let locations = removeable_locations(map);

    cprintln!("Total accessible roll count: <b>{}</>", locations.len());
}

fn part2(map: &[Vec<String>]) {
    let mut removable_location_count = 0;
    let mut mutable_map: Vec<_> = map.to_vec();
    loop {
        let locations = removeable_locations(&mutable_map);
        if locations.is_empty() {
            break;
        }

        removable_location_count += locations.len();
        remove_locations(&locations, &mut mutable_map);
    }

    cprintln!(
        "Total removed locations: <b>{}</>",
        removable_location_count
    );
}

fn removeable_locations(map: &[Vec<String>]) -> HashSet<(usize, usize)> {
    let mut locations = HashSet::new();
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] != TP {
                continue;
            }

            let mut tp_count = 0;
            for (dr, dc) in DIRS {
                let (row, col) = (r as isize + dr, c as isize + dc);

                if row < 0
                    || row >= map.len() as isize
                    || col < 0
                    || col >= map[row as usize].len() as isize
                {
                    continue;
                }

                tp_count += if map[row as usize][col as usize] == TP {
                    1
                } else {
                    0
                };
            }

            if tp_count < 4 {
                locations.insert((r, c));
            }
        }
    }

    locations
}

fn remove_locations(locs: &HashSet<(usize, usize)>, map: &mut [Vec<String>]) {
    for &(r, c) in locs {
        map[r][c] = ".".into();
    }
}
