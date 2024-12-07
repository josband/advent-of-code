use std::fs::read_to_string;

use color_print::cprintln;

type Input = Vec<(usize, Vec<usize>)>;

pub fn day_07() {
    let input = parse();
    part1(&input);
    part2(&input);
}

fn parse() -> Input {
    read_to_string("inputs/day07.txt").unwrap().lines().map(|line| {
        line.split_once(":").map(|(res, operands)| {
            (
                res.parse().unwrap(),
                operands.split_whitespace().map(|operand| operand.parse().unwrap()).collect()
            )
        }).unwrap()
    }).collect()
}

fn part1(input: &Input) {
    let total: usize = input.iter().filter_map(|(target, operands)| {
        if is_equation_true(target, 1, operands[0], operands) {
            Some(*target)
        } else {
            None
        }
    }).sum();
    
    cprintln!("Total Calibration: <b>{}</>", total);
}

fn is_equation_true(target: &usize, position: usize, total: usize, operands: &Vec<usize>) -> bool {
    if position == operands.len() {
        return total == *target;
    }

    is_equation_true(target, position + 1, total + operands[position], operands) 
    || is_equation_true(target, position + 1, total * operands[position], operands) 
}

fn part2(input: &Input) {
    let total: usize = input.iter().filter_map(|(target, operands)| {
        if is_equation_true_with_concat(target, 1, operands[0], operands) {
            Some(*target)
        } else {
            None
        }
    }).sum();
    
    cprintln!("Total Calibration: <b>{}</>", total);
}

fn is_equation_true_with_concat(target: &usize, position: usize, total: usize, operands: &Vec<usize>) -> bool {
    if position == operands.len() {
        return total == *target;
    }

    is_equation_true_with_concat(target, position + 1, total + operands[position], operands) 
    || is_equation_true_with_concat(target, position + 1, total * operands[position], operands) 
    || is_equation_true_with_concat(target, position + 1, concat(total, operands[position]), operands)
}

fn concat(lhs: usize, rhs: usize) -> usize {
    (lhs * 10_usize.pow(rhs.ilog10() + 1)) + rhs
}