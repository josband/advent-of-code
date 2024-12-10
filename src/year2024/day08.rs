use std::{collections::{HashMap, HashSet}, fs::read_to_string};

use color_print::cprintln;

type Input = (Vec<Vec<char>>, HashMap<char, Vec<(usize, usize)>>);

pub fn day_08() {
    let input = parse();
    part1(&input);
    part2(&input);
}

fn parse() -> Input {
    let mut location_map = Vec::new();
    let mut sensor_map = HashMap::new();
    read_to_string("inputs/2024/day08.txt").unwrap().lines().enumerate().for_each(|line| {
        let mut line_values = Vec::new();
        line.1.chars().enumerate().filter_map(|c| {
            line_values.push(c.1);
            if c.1 == '.' {
                return None;
            }

            Some((c.1, (line.0, c.0)))
        }).for_each(|(ch, location)| {
            sensor_map.entry(ch).or_insert(Vec::new()).push(location);
        });

        location_map.push(line_values);
    });

    (location_map, sensor_map)
}

fn part1(input: &Input) {
    let mut locations = HashSet::new();
    input.1.iter().for_each(|(_, sensor_locations)| {
        for (row, location_one) in sensor_locations.iter().enumerate() {
            for (_, location_two) in (&sensor_locations[row+1..]).iter().enumerate() {
                let (rise, run) = (
                    location_one.0 as isize - location_two.0 as isize,
                    location_one.1 as isize - location_two.1 as isize
                );
                let (row_plus, col_plus) = (
                    location_one.0 as isize + rise,
                    location_one.1 as isize + run
                );
                let (row_minus, col_minus) = (
                    location_two.0 as isize - rise,
                    location_two.1 as isize - run
                );

                if is_in_bounds(row_plus, col_plus, input.0[0].len(), input.0.len()) {
                    locations.insert((row_plus, col_plus));
                    
                }

                if is_in_bounds(row_minus, col_minus,input.0[0].len(), input.0.len()) {
                    locations.insert((row_minus, col_minus));
                }
            }
        }
    });

    cprintln!("Number of distinct locations: <b>{}</>", locations.len());
}

fn is_in_bounds(row: isize, col: isize, width: usize, height: usize) -> bool {
    !(row.is_negative() || col.is_negative() || row as usize >= height || col as usize >= width)
}

fn part2(input: &Input) {
    let mut locations = HashSet::new();
    input.1.iter().for_each(|(_, sensor_locations)| {
        if sensor_locations.len() > 1 {
            for (row, location_one) in sensor_locations.iter().enumerate() {
                locations.insert(*location_one);
                for (_, location_two) in (&sensor_locations[row+1..]).iter().enumerate() {
                    let (rise, run) = (
                        location_one.0 as isize - location_two.0 as isize,
                        location_one.1 as isize - location_two.1 as isize
                    );
                    let (mut row_plus, mut col_plus) = (
                        location_one.0 as isize + rise,
                        location_one.1 as isize + run
                    );
                    let (mut row_minus, mut col_minus) = (
                        location_two.0 as isize - rise,
                        location_two.1 as isize - run
                    );

                    while is_in_bounds(row_plus, col_plus, input.0[0].len(), input.0.len()) 
                            || is_in_bounds(row_minus, col_minus,input.0[0].len(), input.0.len()) {
                        if is_in_bounds(row_plus, col_plus, input.0[0].len(), input.0.len()) {
                            locations.insert((row_plus as usize, col_plus as usize));
                            row_plus += rise;
                            col_plus += run;
                        }

                        if is_in_bounds(row_minus, col_minus,input.0[0].len(), input.0.len()) {
                            locations.insert((row_minus as usize, col_minus as usize));
                            row_minus -= rise;
                            col_minus -= run;
                        }
                    }

                }
            }
        }
    });

    cprintln!("Number of distinct locations: <b>{}</>", locations.len());
}