use std::{collections::{BinaryHeap, HashSet}, fs::read_to_string, ops::Add, usize};

use color_print::cprintln;

const WALL: char = '#';
const END: char = 'E';

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
struct Position {
    row: usize,
    col: usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

    fn clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        }
    }

    fn counter_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down
        }
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, rhs: Direction) -> Self::Output {
        let (dr, dc) = rhs.get_deltas();
        Self::Output {
            row: (self.row as i32 + dr) as usize,
            col: (self.col as i32 + dc) as usize
        }
    }
}

type Input = Vec<Vec<char>>;

pub fn day_16() {
    let input = parse();
    part1(&input);
    part2(&input);
}

fn parse() -> Input {
    read_to_string("inputs/2024/day16.txt").unwrap().lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &Input) {
    let start = find_start(input);
    let end = find_end(input);
    let (distances, _) = djikstras(input, start, end, Direction::Right);

    cprintln!("Shortest Distance to End is: <b>{}</b>", distances[end.row][end.col]);
}

fn part2(input: &Input) {
    let start = find_start(input);
    let end = find_end(input);
    let (distances_one, previous_one) = djikstras(input, start, end, Direction::Right);

    let previous_location = previous_one[end.row][end.col];
    let mut starting_dir = Direction::Left;
    for dir in [Direction::Left, Direction::Right, Direction::Up, Direction::Down] {
        if end + dir == previous_location {
            starting_dir = dir;
            break;
        }
    }

    let (distances_two, _) = djikstras(input, end, start, starting_dir);

    let mut tiles = HashSet::new();
    let target = distances_one[end.row][end.col];
    for i in 1..(input.len() - 1) {
        for j in 1..(input[0].len() - 1) {
            if distances_one[i][j] != u32::MAX && distances_two[i][j] != u32::MAX && 
            (distances_one[i][j] + distances_two[i][j] == target || (distances_one[i][j] + distances_two[i][j]) == (target - 1000)) {
                tiles.insert(Position { row: i, col: j });
            }
        }
    }

    cprintln!("Number of Tiles: <b>{}</b>", tiles.len());
}

fn find_start(input: &Input) -> Position {
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if input[row][col] == 'S' {
                return Position { row, col };
            }
        }
    }

    panic!("Didn't find start");
}

fn find_end(input: &Input) -> Position {
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if input[row][col] == END {
                return Position { row, col };
            }
        }
    }

    panic!("Didn't find start");
}

fn djikstras(map: &Vec<Vec<char>>, start: Position, end: Position, init_direction: Direction) -> (Vec<Vec<u32>>, Vec<Vec<Position>>) {
    let (mut distances, mut previous) = (Vec::new(), Vec::new());
    distances.resize_with(map.len(), || {
        let mut temp = Vec::new();
        temp.resize(map[0].len(), u32::MAX);
        temp
    });

    previous.resize_with(map.len(), || {
        let mut temp = Vec::new();
        temp.resize(map[0].len(), Position {
            row: usize::MAX,
            col: usize::MAX
        });
        temp
    });

    distances[start.row][start.col] = 0;

    let mut pq = BinaryHeap::new();
    pq.push((0_i32, start, init_direction));
    while let Some((distance, location, direction)) = pq.pop() {
        let distance: u32 = (-distance) as u32;
        if location == end {
            break;
        }

        for dir in [direction, direction.clockwise(), direction.counter_clockwise()] {
            let neighbor = location + dir;

            if map[neighbor.row][neighbor.col] == WALL {
                continue;
            }

            let delta = 1 + if dir == direction { 0 } else { 1000 };
            let alternative_distance = distance + delta;
            if alternative_distance < distances[neighbor.row][neighbor.col] {
                previous[neighbor.row][neighbor.col] = location;
                distances[neighbor.row][neighbor.col] = alternative_distance;
                pq.push((-(alternative_distance as i32), neighbor, dir));
            }
        }
    }

    (distances, previous)
}
