use std::{
    collections::HashSet,
    fs::read_to_string,
    io::{Error, ErrorKind},
    str::FromStr,
};

use color_print::cprintln;

pub fn day_06() {
    let input = parse();
    part1(&input);
    part2(&input);
}

#[derive(Debug)]
struct Problem {
    operands: Vec<i64>,
    op: Operation,
}

impl Problem {
    fn eval(&self) -> i64 {
        self.operands
            .iter()
            .copied()
            .reduce(|acc, operand| self.op.perform_op(acc, operand))
            .unwrap()
    }
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn perform_op(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Operation::Add => lhs + rhs,
            Operation::Multiply => lhs * rhs,
        }
    }
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Operation::Multiply),
            "+" => Ok(Operation::Add),
            _ => Err(Error::from(ErrorKind::InvalidData)),
        }
    }
}

fn parse() -> Vec<Vec<String>> {
    let input = read_to_string("./inputs/2025/day06.txt").expect("Can't read Day 6 of 2025");
    let lines: Vec<_> = input.lines().collect();

    let nums = &lines[..lines.len() - 1];
    let operators = lines[lines.len() - 1];
    let mut col_starts = HashSet::new();
    for (i, c) in operators.chars().enumerate() {
        if !c.is_whitespace() {
            col_starts.insert(i);
        }
    }

    let mut problem_matrix: Vec<Vec<_>> = nums
        .iter()
        .map(|l| {
            let mut nums_with_whitespace = vec![];
            let mut num_builder = String::new();
            for (i, c) in l.chars().enumerate() {
                if c.is_whitespace() && col_starts.contains(&(i + 1)) {
                    nums_with_whitespace.push(num_builder.clone());
                    num_builder.clear();
                } else {
                    num_builder.push(c);
                }
            }

            nums_with_whitespace.push(num_builder);

            nums_with_whitespace
        })
        .collect();

    problem_matrix.push(
        operators
            .split_whitespace()
            .map(|s| s.to_string())
            .collect(),
    );

    transpose(problem_matrix)
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn part1(problems: &[Vec<String>]) {
    let sum = problems
        .iter()
        .map(|p| Problem {
            operands: p[0..(p.len() - 1)]
                .iter()
                .map(|n| i64::from_str(n.trim()).unwrap())
                .collect(),
            op: Operation::from_str(p.last().unwrap()).unwrap(),
        })
        .map(|p| p.eval())
        .reduce(|acc, n| acc + n)
        .unwrap();

    cprintln!("Sum of all problems: <b>{}</>", sum);
}

fn part2(problems: &[Vec<String>]) {
    let sum = problems
        .iter()
        .map(|p| {
            let numbers_by_column: Vec<Vec<_>> = p[..(p.len() - 1)]
                .iter()
                .map(|n| n.chars().collect())
                .collect();

            let mut correct_numbers = vec![];
            let mut number_builder = String::new();
            for c in 0..numbers_by_column[0].len() {
                for digit_array in numbers_by_column.iter() {
                    number_builder.push(digit_array[c]);
                }

                let cloned_num = number_builder.clone();
                number_builder.clear();
                correct_numbers.push(i64::from_str(cloned_num.trim()).unwrap());
            }

            Problem {
                operands: correct_numbers,
                op: Operation::from_str(&p[p.len() - 1]).unwrap(),
            }
            .eval()
        })
        .reduce(|acc, n| acc + n)
        .unwrap();

    cprintln!("Sum of all problems: <b>{}</>", sum);
}
