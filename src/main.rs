use std::path::Path;

use advent_of_code::year2024::*;

use color_print::cprintln;

const NUMBER_OF_DAYS: usize = 25;

fn main() {
    cprintln!("<s><g>Advent</> <r>of</> <g>Code</> <r>2024</>:</>");

    for day in (1..NUMBER_OF_DAYS).rev() {
        if !Path::new(&format!("./inputs/2024/day{day:0>2}.txt")).exists() {
            continue;
        }

        cprintln!("\n[<b>Day {:>2}</>: <g, s>EXISTS</>]\n", day);
        match day {
            1 => day01::day_01(),
            2 => day02::day_02(),
            3 => day03::day_03(),
            4 => day04::day_04(),
            5 => day05::day_05(),
            6 => day06::day_06(),
            7 => day07::day_07(),
            8 => day08::day_08(),
            9 => day09::day_09(),
            10 => day10::day_10(),
            _ => panic!()
        }
        break;
    }
}
