use std::path::Path;

use advent_of_code::year2024::*;

use color_print::cprintln;

const NUMBER_OF_DAYS: usize = 25;

fn main() {
    cprintln!("<bold><g>Advent</> <r>of</> <g>Code</> <r>2024</>:</>");

    for day in 1..=NUMBER_OF_DAYS {
        if !Path::new(&format!("./inputs/day{day:0>2}.txt")).exists() {
            break;
        }

        cprintln!("\n[<b>Day {:>2}</>: <g, bold>EXISTS</>]\n", day);
        match day {
            1 => day01::day_01(),
            2 => day02::day_02(),
            3 => day03::day_03(),
            4 => day04::day_04(),
            5 => day05::day_05(),
            _ => panic!()
        }
    }
}
