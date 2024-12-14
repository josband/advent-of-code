use std::{collections::HashMap, fs::read_to_string};

use color_print::cprintln;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

type Input = Vec<Robot>;

struct Robot {
    position: (i32, i32),
    velocity: (i32, i32)
}

impl Robot {
    fn elapse_time(&self, delta_t: i32, width: i32, height: i32) -> Robot {
        let unwrapped_pos = (
            self.position.0 + self.velocity.0 * delta_t, 
            self.position.1 + self.velocity.1 * delta_t
        );
        let wrapped_pos = (unwrapped_pos.0.rem_euclid(width), unwrapped_pos.1.rem_euclid(height));
        Robot {
            position: wrapped_pos,
            ..*self
        }
    }
}

pub fn day_14() {
    let input = parse();
    part1(&input);
    part2(&input);
}

fn parse() -> Input {
    read_to_string("inputs/2024/day14.txt").unwrap().lines().map(|line| {
        let params = line.split_whitespace().map(|attribute| attribute[2..].split(',')
            .map(|num| num.parse().unwrap()).collect())
            .collect::<Vec<Vec<i32>>>();
        Robot {
            position: (params[0][0], params[0][1]),
            velocity: (params[1][0], params[1][1])
        }
    }).collect()
}

fn part1(input: &Input) {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    input.iter().map(|robot| robot.elapse_time(100, WIDTH, HEIGHT)).filter_map(|new_robot| {
        let mid_width = WIDTH / 2;
        let mid_height = HEIGHT / 2;
        if new_robot.position.0 == mid_width || new_robot.position.1 == mid_height {
            None
        } else if new_robot.position.0 < mid_width && new_robot.position.1 < mid_height {
            Some(0)
        } else if new_robot.position.0 > mid_width && new_robot.position.1 < mid_height {
            Some(1)
        } else if new_robot.position.0 < mid_width && new_robot.position.1 > mid_height {
            Some(2)
        } else {
            Some(3)
        }
    }).for_each(|quad| *counts.entry(quad).or_insert(0) += 1);

    let result: i32 = counts.iter().map(|(_, &count)| count).product();
    cprintln!("Safety Factor: <b>{}</>", result);
}

fn part2(input: &Input) {
    let mut count = 0;
    let mut position = input.iter().map(|robot| robot.elapse_time(0, WIDTH, HEIGHT)).collect::<Vec<Robot>>();
    let mut grid: Vec<char> = Vec::new();
    grid.resize((WIDTH * HEIGHT)as usize, ' ');
    loop {
        for robot in position.iter() {
            grid[(robot.position.1 * WIDTH + robot.position.0) as usize] = '#';
        }

        if grid.iter().collect::<String>().contains("##########") {
            break;
        }
        grid.fill(' ');
        position = position.iter().map(|robot| robot.elapse_time(1, WIDTH, HEIGHT)).collect();
        count += 1;
    }

    cprintln!("Iterations: <b>{}</>", count);
}