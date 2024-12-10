use std::{fs::read_to_string, vec};

use color_print::cprintln;

type Input = Vec<u32>;

pub fn day_09() {
    let input = parse();
    part1(&input);
    part2(&input);
}

fn parse() -> Input {
    read_to_string("inputs/2024/day09.txt").unwrap().chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn part1(input: &Input) {
    let mut expanded_input = expand_disk_layout(input);
    compact_disc(&mut expanded_input);    
    let checksum = compute_checksum(&expanded_input);

    cprintln!("Checksum: <b>{}</>", checksum);
}

fn compute_checksum(disk: &Vec<i32>) -> u64 {
    let mut checksum = 0;
    for (position, file_id) in disk.iter().enumerate() {
        if *file_id == -1 {
            continue;
        }

        checksum += position as u64 * *file_id as u64;
    }

    checksum
}

fn compact_disc(expanded_input: &mut Vec<i32>) {
    let (mut left, mut right) = (0, expanded_input.len() - 1);

    while left < right {
        // find first '.' for left
        while left < right && expanded_input[left] != -1 {
            left += 1;
        }

        // find first non-'.' for right
        while right > left && expanded_input[right] == -1 {
            right -= 1;
        }

        if right > left {
            expanded_input.swap(left, right);
        }
    }
}

fn expand_disk_layout(input: &Input) -> Vec<i32> {
    let mut file_id = 0;
    input.iter().enumerate().filter_map(|(index, size)| {
        let output_num = if index % 2 == 0 { 
            let res = file_id;
            file_id += 1;
            res
        } else {
            -1
        };
        Some(vec![output_num; *size as usize])
    }).flatten().collect()
}

fn part2(input: &Input) {
    let mut expanded_layout = expand_disk_layout(input);
    let max_file_num = expanded_layout.iter().rev().find(|&file_num| *file_num != -1).unwrap();

    let mut right = expanded_layout.len() - 1;
    for file_num in (0..*max_file_num + 1).rev() {
        while right > 0 && expanded_layout[right] != file_num {
            right -= 1;
        }

        let mut left = right;
        while left > 0 && expanded_layout[left - 1] == file_num {
            left -= 1;
        }

        shift_file_left(&mut expanded_layout, left, right);
    }

    let result = compute_checksum(&mut expanded_layout);
    cprintln!("Checksum: <b>{}</>", result);
}

fn shift_file_left(disk: &mut Vec<i32>, left: usize, right: usize) {
    let mut swap_start = 0;
    loop {
        if swap_start >= left {
            break;
        }

        while swap_start < left && disk[swap_start] != -1 {
            swap_start += 1;
        }

        let mut swap_end = swap_start;
        while swap_end < left && disk[swap_end + 1] == -1 {
            swap_end += 1;
        }

        if swap_end - swap_start + 1 >= right - left + 1 {
            let mut pos = swap_start;
            for i in left..right + 1 {
                disk.swap(pos, i);
                pos += 1;
            }
            break;
        } else {
            swap_start = swap_end + 1;
        }
    }
}