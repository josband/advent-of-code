use std::fs::read_to_string;

use color_print::cprintln;

#[derive(Clone, Copy)]
struct Machine {
    button_a: (isize, isize),
    button_b: (isize, isize),
    target: (isize, isize)
}

type Input = Vec<Machine>;

const TARGET_ERR: isize = 10000000000000;

pub fn day_13() {
    let input = parse();
    part1(&input);
    part2(&input);
}

fn parse() -> Input {
    read_to_string("inputs/2024/day13.txt").unwrap().split("\r\n\r\n").map(|machine| {
        let machine_spec = machine.lines().map(|line| {
            line.split(", ").map(|subset| 
                subset.chars().filter(|character| character.is_numeric()).collect::<String>().parse().unwrap()
            ).collect()
        }).collect::<Vec<Vec<isize>>>();

        Machine {
            button_a: (machine_spec[0][0], machine_spec[0][1]),
            button_b: (machine_spec[1][0], machine_spec[1][1]),
            target: (machine_spec[2][0], machine_spec[2][1]),
        }
    }).collect()
}

fn part1(input: &Input) {
    let result: isize = input.iter().filter_map(|machine| compute_solution(machine)).map(|result| compute_cost(result)).sum();
    cprintln!("Tokens used: <b>{}</>", result);
}

fn compute_solution(machine: &Machine) -> Option<(isize, isize)> {
    let determinent_denom = (machine.button_a.0 * machine.button_b.1) - (machine.button_b.0 * machine.button_a.1);
    let a = (machine.button_b.1 * machine.target.0) - (machine.button_b.0 * machine.target.1);
    let b = (-machine.button_a.1 * machine.target.0) + (machine.button_a.0 * machine.target.1);
    if a % determinent_denom != 0 || b % determinent_denom != 0 {
        None
    } else {
        Some((a / determinent_denom, b / determinent_denom))
    }
}

fn compute_cost(presses: (isize, isize)) -> isize {
    presses.0 * 3 + presses.1
}

fn part2(input: &Input) {
    let result: isize = input.iter().filter_map(|machine| {
        let mut modified_machine = machine.clone();
        modified_machine.target = (modified_machine.target.0 + TARGET_ERR, modified_machine.target.1 + TARGET_ERR);
        compute_solution(&modified_machine)
    }).map(|result| compute_cost(result)).sum();
    cprintln!("Tokens used: <b>{}</>", result);
}