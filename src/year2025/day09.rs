use std::{
    cmp::{max, min},
    collections::{HashMap, VecDeque},
    fs::read_to_string,
    io::{Error, ErrorKind},
    str::FromStr,
    vec,
};

use color_print::cprintln;
use itertools::Itertools;

#[derive(PartialEq, Eq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn area(&self, other: &Point) -> usize {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(",")
            .ok_or(Error::from(ErrorKind::InvalidInput))?;
        Ok(Point {
            x: usize::from_str(x).unwrap(),
            y: usize::from_str(y).unwrap(),
        })
    }
}

pub fn day_09() {
    let input = parse();

    part1(&input);
    part2(&input);
}

fn parse() -> Vec<Point> {
    read_to_string("./inputs/2025/day09.txt")
        .expect("Can't read 2025 Day 9")
        .lines()
        .map(|l| Point::from_str(l).unwrap())
        .collect()
}

fn part1(boxes: &[Point]) {
    let mut max_area = 0;
    for (i, p1) in boxes.iter().enumerate() {
        for p2 in &boxes[i + 1..] {
            max_area = max(max_area, p1.area(p2));
        }
    }

    cprintln!("Max area: <b>{}</>", max_area);
}

fn part2(points: &[Point]) {
    // Perform coordinate compression
    let x_compress: HashMap<_, _> = points
        .iter()
        .map(|p| p.x)
        .sorted()
        .dedup()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect();

    let y_compress: HashMap<_, _> = points
        .iter()
        .map(|p| p.y)
        .sorted()
        .dedup()
        .enumerate()
        .map(|(i, y)| (y, i))
        .collect();

    // Map existing points to compressed space
    let mapped_points: Vec<_> = points
        .iter()
        .map(|p| Point {
            x: x_compress[&p.x],
            y: y_compress[&p.y],
        })
        .collect();

    // Fill in the edges
    let mut grid = vec![vec!["."; x_compress.len()]; y_compress.len()];
    let mut previous = None;
    for i in 0..=mapped_points.len() {
        let point = &mapped_points[i % mapped_points.len()];
        grid[point.y][point.x] = "#";

        previous.inspect(|prev: &&Point| {
            if prev.x == point.x {
                #[allow(clippy::needless_range_loop)]
                for j in min(prev.y, point.y) + 1..max(point.y, prev.y) {
                    grid[j][point.x] = "X";
                }
            } else {
                for j in min(prev.x, point.x) + 1..max(point.x, prev.x) {
                    grid[point.y][j] = "X";
                }
            }
        });

        previous = Some(point);
    }

    // Find inner point and fill shape from that point
    let (mut start_x, mut start_y) = (0, 0);
    'a: for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] != "." {
                // Skip border cells
                continue;
            } else if is_inside_shape(r, c, &grid) {
                (start_x, start_y) = (c, r);
                break 'a;
            }
        }
    }

    fill_shape(start_y, start_x, &mut grid);

    let mut max_size = 0;
    for (i, point) in mapped_points.iter().enumerate() {
        for (j, corner) in mapped_points
            .iter()
            .enumerate()
            .skip(i + 1)
            .collect::<Vec<_>>()
        {
            if is_valid_rectangle(point, corner, &grid) {
                let (p1, p2) = (&points[i], &points[j]);
                max_size = max(max_size, p1.area(p2));
            }
        }
    }

    cprintln!("Max area: <b>{}</>", max_size);
}

fn is_valid_rectangle(point: &Point, corner: &Point, grid: &[Vec<&str>]) -> bool {
    for x in min(point.x, corner.x)..=max(point.x, corner.x) {
        #[allow(clippy::needless_range_loop)]
        for y in min(point.y, corner.y)..=max(point.y, corner.y) {
            if grid[y][x] == "." {
                return false;
            }
        }
    }

    true
}

// Raycast to see if point is within a shape
fn is_inside_shape(row: usize, col: usize, grid: &[Vec<&str>]) -> bool {
    let mut crossings = 0;
    let mut curr_col = col + 1;

    while curr_col < grid[row].len() {
        if grid[row][curr_col] != "." {
            let curr = grid[row][curr_col];

            if curr == "#" {
                while curr_col + 1 < grid[row].len() && grid[row][curr_col + 1] != "." {
                    curr_col += 1;
                }
            } else {
                crossings += 1;
            }
        }
        curr_col += 1;
    }

    crossings % 2 == 1
}

fn fill_shape(r: usize, c: usize, grid: &mut [Vec<&str>]) {
    let mut vec_deq = VecDeque::new();
    vec_deq.push_back((r, c));
    while let Some((curr_r, curr_c)) = vec_deq.pop_front() {
        if grid[curr_r][curr_c] != "." {
            continue;
        }

        grid[curr_r][curr_c] = "X";
        add_coord_to_queue(curr_r as isize + 1, curr_c as isize, grid, &mut vec_deq);
        add_coord_to_queue(curr_r as isize - 1, curr_c as isize, grid, &mut vec_deq);
        add_coord_to_queue(curr_r as isize, curr_c as isize + 1, grid, &mut vec_deq);
        add_coord_to_queue(curr_r as isize, curr_c as isize - 1, grid, &mut vec_deq);
    }
}

fn add_coord_to_queue(
    r: isize,
    c: isize,
    grid: &[Vec<&str>],
    queue: &mut VecDeque<(usize, usize)>,
) {
    if r < 0 || c < 0 || r >= grid.len() as isize || c >= grid[r as usize].len() as isize {
        return;
    }

    queue.push_back((r as usize, c as usize));
}
