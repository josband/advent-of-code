use std::fs::read_to_string;

use color_print::cprintln;

const START: &str = "S";
const SPLITTER: &str = "^";
const BEAM: &str = "|";

pub fn day_07() {
    let input = parse();

    part1(&input.0, input.1);
    part2(&input.0, input.1);
}

#[allow(clippy::needless_range_loop)]
fn parse() -> (Vec<Vec<String>>, (usize, usize)) {
    let map: Vec<Vec<String>> = read_to_string("./inputs/2025/day07.txt")
        .expect("Can't open Day 7 of 2025")
        .lines()
        .map(|l| l.chars().map(|c| c.to_string()).collect())
        .collect();

    let mut start = None;
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if START == map[r][c] {
                start = Some((r, c));
                break;
            }
        }
    }

    (map, start.unwrap())
}

fn part1(map: &[Vec<String>], start: (usize, usize)) {
    let mut map = map.to_vec();
    let split_count = split_count((start.0 + 1, start.1), &mut map);

    cprintln!("Split Count: <b>{}</>", split_count);
}

fn split_count(pos: (usize, usize), map: &mut [Vec<String>]) -> usize {
    if map.get(pos.0).is_none() || map[pos.0].get(pos.1).is_none() {
        return 0;
    }

    if map[pos.0][pos.1] == BEAM {
        0
    } else if map[pos.0][pos.1] == SPLITTER {
        1 + split_count((pos.0, pos.1 - 1), map) + split_count((pos.0, pos.1 + 1), map)
    } else {
        map[pos.0][pos.1] = BEAM.to_string();
        split_count((pos.0 + 1, pos.1), map)
    }
}

fn part2(map: &[Vec<String>], start: (usize, usize)) {
    let mut mem = vec![];
    map.iter().for_each(|_| mem.push(vec![0; map[0].len()]));
    let timelines = split_count_memoized(start, map, &mut mem);

    cprintln!("Total timelines: <b>{}</>", timelines);
}

fn split_count_memoized(pos: (usize, usize), map: &[Vec<String>], mem: &mut [Vec<usize>]) -> usize {
    if map.get(pos.0).is_none() || map[pos.0].get(pos.1).is_none() {
        return 1;
    }

    if mem[pos.0][pos.1] != 0 {
        return mem[pos.0][pos.1];
    } else if map[pos.0][pos.1] == SPLITTER {
        mem[pos.0][pos.1] = split_count_memoized((pos.0, pos.1 - 1), map, mem)
            + split_count_memoized((pos.0, pos.1 + 1), map, mem);
    } else {
        mem[pos.0][pos.1] = split_count_memoized((pos.0 + 1, pos.1), map, mem);
    }

    mem[pos.0][pos.1]
}
