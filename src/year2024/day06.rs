use std::{collections::HashSet, fs::read_to_string, ops::Add, vec};

use color_print::cprintln;

type Input = Vec<Vec<char>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn clockwise(&self) -> Self {
        use self::Direction::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up
        }
    }

    fn get_deltas(&self) -> (i32, i32) {
        use self::Direction::*;
        match self {
            Up => (-1, 0),
            Right => (0, 1),
            Down => (1, 0),
            Left => (0, -1)
        }
    }
}

impl Add<Direction> for (i32, i32) {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        let (dr, dc) = rhs.get_deltas();
        (self.0 + dr, self.1 + dc)
    }
}

const OBSTRUCTION: char = '#';

pub fn day_06() {
    let input = parse();
    part1(&input);
    part2(&input);
}

fn parse() -> Input {
    read_to_string("./inputs/day06.txt").expect("Cannot read 2024 day 06").lines()
            .map(|line| line.chars().collect()).collect()
}

fn part1(input: &Input) {
    let mut distinct_locations = HashSet::new();
    let (mut row, mut col) = input.iter().enumerate().filter_map(|(row, line)| {
        line.iter().enumerate().find(|(_, character)| **character == '^').map(|(col, _)| (row, col))
    }).next().unwrap();

    let mut direction = Direction::Up;
    while is_location_in_bounds((row as i32, col as i32) + direction, input) {
        let next_position = (row as i32, col as i32) + direction;
        if input[next_position.0 as usize][next_position.1 as usize] == OBSTRUCTION {
            direction = direction.clockwise();
        }

        distinct_locations.insert((row, col));
        let next_position = (row as i32, col as i32) + direction;
        (row, col) = (next_position.0 as usize, next_position.1 as usize);
    }
    
    distinct_locations.insert((row, col));
    cprintln!("Distinct Positions: <b>{}</>", distinct_locations.len());
}

fn part2(input: &Input) {
    let (start_row, start_col) = input.iter().enumerate().filter_map(|(row, line)| {
        line.iter().enumerate().find(|(_, character)| **character == '^').map(|(col, _)| (row, col))
    }).next().unwrap();

    let mut board = input.clone();
    let mut count = 0;
    let (mut row, mut col) = (start_row, start_col);
    let mut direction = Direction::Up;
    let mut seen = vec![vec![false; input[0].len()]; board.len()];
    while is_location_in_bounds((row as i32, col as i32) + direction, &board) {
        seen[row][col] = true;
        let (next_row, next_col) = (row as i32, col as i32) + direction;
        if input[next_row as usize][next_col as usize] == OBSTRUCTION {
            direction = direction.clockwise();
        } else {
            let (next_row, next_col) = (next_row as usize, next_col as usize);
            if !(next_row == start_row && next_col == start_col) && !seen[next_row][next_col] {
                board[next_row][next_col] = OBSTRUCTION;
                if contains_loop(&mut board, row, col, direction) {
                    count += 1;
                }
                board[next_row][next_col] = '.';
            }
            (row, col) = (next_row as usize, next_col as usize);
        }
    }
                
    cprintln!("Number of possible infinite loops: <b>{}</>", count);
}

fn contains_loop(board: &mut Input, start_row: usize, start_col: usize, mut direction: Direction) -> bool {
    let mut paths = HashSet::new();
    let (mut row, mut col) = (start_row, start_col);
    while is_location_in_bounds((row as i32, col as i32) + direction, &board) {
        if paths.contains(&(row, col, direction)) {
            return true;
        }

        paths.insert((row, col, direction));
        let unchecked_new_locations = (row as i32, col as i32) + direction;
        let (next_row, next_col) = (unchecked_new_locations.0 as usize, unchecked_new_locations.1 as usize);
        if board[next_row][next_col] == OBSTRUCTION {
            direction = direction.clockwise();
        } else {
            (row, col) = (next_row, next_col);
        }
    }

    false
}

fn is_location_in_bounds((row, col): (i32, i32), room: &Input) -> bool {
    row >= 0 && row < room.len() as i32 && col >= 0 && col < room[row as usize].len() as i32  
}