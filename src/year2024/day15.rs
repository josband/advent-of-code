use std::{fs::read_to_string, ops::Add, str::FromStr};

use color_print::cprintln;

const BOX: char = 'O';
const ROBOT: char = '@';
const WALL: char = '#';
const EMPTY: char = '.';

type Input = (Vec<Vec<char>>, Vec<Direction>);

#[derive(Debug)]
struct Position {
    row: i32,
    col: i32
}

impl Add<&Direction> for &Position {
    type Output = Position;

    fn add(self, rhs: &Direction) -> Self::Output {
        let (dr, dc) = rhs.get_deltas();
        Self::Output {
            row: self.row + dr,
            col: self.col + dc
        }
    }
}

impl Position {
    fn move_recursive(&mut self, direction: &Direction, map: &mut Vec<Vec<char>>) -> bool {
        if map[self.row as usize][self.col as usize] == WALL  {
            return false;
        } else if map[self.row as usize][self.col as usize] == EMPTY {
            return true;
        }

        let mut next_position = &*self + direction;
        let res = match direction {
            Direction::Left | Direction::Right => {
                if next_position.move_recursive(direction, map) {
                    Self::swap_positions(&self, &Position { row: next_position.row, col: next_position.col }, map);
                    true
                } else {
                    false
                }
            },
            Direction::Up | Direction::Down => {
                if self.can_move_vertically(direction, map) {
                    self.shift_vertically(direction, map);
                    true
                } else {
                    false
                }
            }
        };

        if res && map[next_position.row as usize][next_position.col as usize] == ROBOT {
            self.row = next_position.row;
            self.col = next_position.col;
        }

        res
    }

    fn shift_vertically(&self, direction: &Direction, map: &mut Vec<Vec<char>>) {
        let next_position = self + direction;
        if map[next_position.row as usize][next_position.col as usize] == EMPTY {
            if map[self.row as usize][self.col as usize] == '[' {
                Self::swap_positions(self, &next_position, map);
                let right = self + &Direction::Right;
                if map[right.row as usize][right.col as usize] != EMPTY {
                    right.shift_vertically(direction, map);
                }
            } else if map[self.row as usize][self.col as usize] == ']' {
                Self::swap_positions(self, &next_position, map);
                let left = self + &Direction::Left;
                if map[left.row as usize][left.col as usize] != EMPTY {
                    left.shift_vertically(direction, map);
                }
            } else {
                Self::swap_positions(self, &next_position, map);
            }

        } else {
            if map[self.row as usize][self.col as usize] == '[' {
                let right = self + &Direction::Right;
                next_position.shift_vertically(direction, map);
                Self::swap_positions(self, &next_position, map);
                if map[right.row as usize][right.col as usize] != EMPTY {
                    right.shift_vertically(direction, map);
                }
            } else if map[self.row as usize][self.col as usize] == ']' {
                let left = self + &Direction::Left;
                next_position.shift_vertically(direction, map);
                Self::swap_positions(self, &next_position, map);
                if map[left.row as usize][left.col as usize] != EMPTY {
                    left.shift_vertically(direction, map);
                }
            } else {
                next_position.shift_vertically(direction, map);
                Self::swap_positions(self, &next_position, map);
            }
        }
    }

    fn can_move_vertically(&self, direction: &Direction, map: &Vec<Vec<char>>) -> bool {
        if map[self.row as usize][self.col as usize] == EMPTY {
            true
        } else if map[self.row as usize][self.col as usize] == WALL {
            false
        } else if map[self.row as usize][self.col as usize] == ROBOT {
            (self + direction).can_move_vertically(direction, map)
        } else {
            let mut should_move = (self + direction).can_move_vertically(direction, map);
            if map[self.row as usize][self.col as usize] == '[' {
                let right = self + &Direction::Right;
                should_move = should_move && (&right + direction).can_move_vertically(direction, map);
            } else {
                let left = self + &Direction::Left;
                should_move = should_move && (&left + direction).can_move_vertically(direction, map);
            }

            should_move               
        }
    }

    fn swap_positions(pos_one: &Position, pos_two: &Position, map: &mut Vec<Vec<char>>) {
            let temp = map[pos_one.row as usize][pos_one.col as usize];
            map[pos_one.row as usize][pos_one.col as usize] = map[pos_two.row as usize][pos_two.col as usize];
            map[pos_two.row as usize][pos_two.col as usize] = temp;
    }

    fn move_robot(&mut self, direction: &Direction, map: &mut Vec<Vec<char>>) {
        if !self.can_move(direction, map) {
            return;
        }

        let mut curr_position = &*self + direction;
        while map[curr_position.row as usize][curr_position.col as usize] != EMPTY {
            curr_position = &curr_position + direction;
        }

        let opposite_dir = direction.reverse();
        let mut next = &curr_position + &opposite_dir; 
        while map[next.row as usize][next.col as usize] != ROBOT {
            Self::swap_positions(&curr_position, &next, map);
            curr_position = &curr_position + &opposite_dir;
            next = &next + &opposite_dir;
        }
        
        Self::swap_positions(&curr_position, &next, map);
        self.row = curr_position.row;
        self.col = curr_position.col;
    }

    fn can_move(&self, direction: &Direction, map: &Vec<Vec<char>>) -> bool {
        let mut curr_position = self + direction;
        while curr_position.is_in_bounds(map) && map[curr_position.row as usize][curr_position.col as usize] == BOX {
            curr_position = &curr_position + direction;
        }

        curr_position.is_in_bounds(map) && map[curr_position.row as usize][curr_position.col as usize] == EMPTY
    }

    fn is_in_bounds(&self, map: &Vec<Vec<char>>) -> bool {
        self.row > -1 && self.col > -1 && (self.row as usize) < map.len() && (self.col as usize) < map[self.row as usize].len()
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn get_deltas(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1)
        }
    }

    fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right
        }
    }
}

impl FromStr for Direction {
    type Err = std::io::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Direction::Up),
            "v" => Ok(Direction::Down),
            ">" => Ok(Direction::Right),
            "<" => Ok(Direction::Left),
            _ => Err(std::io::ErrorKind::InvalidInput)
        }
    }
}

pub fn day_15() {
    let input = parse();
    part1(&input);
    part2(&input);
}

fn parse() -> Input {
    let binding = read_to_string("inputs/2024/day15.txt").unwrap();
    let sections: Vec<&str> = binding.split("\r\n\r\n").collect();
    let map = sections[0].lines().map(|line| line.chars().collect()).collect();
    let directions = sections[1].chars().filter_map(|c| Direction::from_str(&c.to_string()).ok()).collect();

    (map, directions)
}

fn part1(input: &Input) {
    let mut position = get_start_position(&input.0).unwrap();
    let mut map = input.0.clone();
    input.1.iter().for_each(|direction| {
        position.move_robot(direction, &mut map);
    });

    let mut result = 0;
    for row in 1..(map.len() - 1) {
        for col in 1..(map[row].len() - 1) {
            if map[row][col] == BOX {
                result += 100 * row + col;
            }
        }
    }

    cprintln!("GPS Sum: <b>{}</>", result);
}

fn get_start_position(input: &Vec<Vec<char>>) -> Option<Position> {
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if input[row][col] == '@'  {
                return Some(Position {
                    row: row as i32,
                    col: col as i32
                })
            }
        }
    }

    None
}

fn part2(input: &Input) {
    let mut converted_map = convert_map(&input.0);
    let mut position = get_start_position(&converted_map).unwrap();
    input.1.iter().for_each(|direction| {
        position.move_recursive(direction, &mut converted_map); 
        
        assert!(converted_map[position.row as usize][position.col as usize] == ROBOT);
    });

    let mut result = 0;
    for row in 1..(converted_map.len() - 1) {
        for col in 2..(converted_map[row].len() - 2) {
            if converted_map[row][col] == '[' {
                result += 100 * row + col;
            }
        }
    }

    cprintln!("GPS Sum: <b>{}</>", result);
}

fn convert_map(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res = Vec::new();
    for row in 0..map.len() {
        let mut row_vec = Vec::new();
        for col in 0..map[row].len() {
            match map[row][col] {
                WALL => {
                    row_vec.push(WALL);
                    row_vec.push(WALL);
                }
                BOX => {
                    row_vec.push('[');
                    row_vec.push(']');
                }
                ROBOT => {
                    row_vec.push(ROBOT);
                    row_vec.push(EMPTY);
                }
                EMPTY => {
                    row_vec.push(EMPTY);
                    row_vec.push(EMPTY);
                }
                _ => panic!("Bad character")
            }
        }
        res.push(row_vec);
    }

    res
}