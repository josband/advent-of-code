use std::fs::read_to_string;

use color_print::cprintln;

const DIRS: [(i32, i32); 8] = [(1, 0), (0, 1), (1, 1), (1, -1), (-1, 1), (-1, -1), (-1, 0), (0, -1)];
const XMAS: &str = "XMAS";

pub fn day_04() {
    part1();
    part2();
}

fn part1() {
    let input = parse();
    let mut count = 0;

    for r in 0..input.len() {
        for c in 0..input[r].len() {
            count += match_xmas(&input, r as i32, c as i32);               
        }
    }

    cprintln!("Total number of XMAS's: <b>{}</>", count);
}

fn match_xmas(crossword: &Vec<Vec<char>>, r: i32, c: i32) -> i32 {
    let mut matches = 0;
    for (dr, dc) in DIRS {
        let mut current_row = r;
        let mut current_column = c;
        for character in XMAS.chars() {
            if current_row < 0 || current_row >= crossword.len() as i32 || current_column < 0 
                || current_column >= crossword[current_row as usize].len() as i32 || crossword[current_row as usize][current_column as usize] != character {
                    break;
            } else if character == 'S' {
                matches += 1;
            }

            current_row += dr;
            current_column += dc;
        }
    }

    matches
}

fn part2() {
    let input = parse();
    let mut count = 0;
    for r in 1..input.len()-1 {
        for c in 1..input[r].len()-1 {
            count += match_mas(&input, r, c);               
        }
    }

    cprintln!("Total number of X-MAS's: <b>{}</>", count);
}

fn match_mas(crossword: &Vec<Vec<char>>, r: usize, c: usize) -> i32 {
    let upward_slope = crossword[r+1][c-1].to_string() + &crossword[r][c].to_string() + &crossword[r-1][c+1].to_string();
    let downward_slop = crossword[r+1][c+1].to_string() + &crossword[r][c].to_string() + &crossword[r-1][c-1].to_string();
    if (upward_slope == "MAS" || upward_slope == "SAM") && (downward_slop == "MAS" || downward_slop == "SAM") {
        1
    } else {
        0
    }
}

fn parse() -> Vec<Vec<char>> {
    read_to_string("./inputs/day04.txt").expect("Cannot open 2024 day 4").lines().map(|line| {
        line.chars().collect()
    }).collect()
}

