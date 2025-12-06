use std::{cmp::max, collections::HashSet, fs::read_to_string, ops::RangeInclusive, str::FromStr};

use color_print::cprintln;

pub fn day_05() {
    let (ranges, nums) = parse();
    part1(&ranges, &nums);
    part2(&ranges);
}

fn parse() -> (Vec<RangeInclusive<isize>>, Vec<isize>) {
    let mut ranges = vec![];
    let mut nums = vec![];
    let mut is_ranges = true;
    let input = read_to_string("./inputs/2025/day05.txt").expect("Can't read 2025 Day 5");
    input.lines().for_each(|l| {
        if l.trim().is_empty() {
            is_ranges = false;
            return;
        }

        if is_ranges {
            ranges.push(l);
        } else {
            nums.push(l);
        }
    });

    let parsed_ranges = ranges
        .into_iter()
        .map(|r| {
            let (lo, hi) = r.split_once("-").unwrap();
            RangeInclusive::new(isize::from_str(lo).unwrap(), isize::from_str(hi).unwrap())
        })
        .collect();

    let parsed_nums = nums
        .into_iter()
        .map(|n| isize::from_str(n).unwrap())
        .collect();

    (parsed_ranges, parsed_nums)
}

fn part1(ranges: &[RangeInclusive<isize>], nums: &[isize]) {
    let mut valid_ids = HashSet::new();
    for &num in nums {
        for range in ranges {
            if range.contains(&num) {
                valid_ids.insert(num);
            }
        }
    }

    cprintln!("Fresh ID count: <b>{}</>", valid_ids.len());
}

fn part2(ranges: &[RangeInclusive<isize>]) {
    // To whoever may read this code (likely nobody), I apologize.
    let mut sorted_ranges = ranges.to_vec();
    sorted_ranges.sort_by(|l, r| l.start().cmp(r.start()));

    let mut merged_ranges = vec![];
    let mut curr_range = sorted_ranges[0].clone();
    for (i, range) in sorted_ranges.iter().enumerate() {
        println!("{:?} {:?}", curr_range, range);
        if range.start() <= curr_range.end() {
            curr_range =
                RangeInclusive::new(*curr_range.start(), max(*curr_range.end(), *range.end()))
        } else {
            merged_ranges.push(curr_range.clone());
            curr_range = range.clone();
        }

        if i == sorted_ranges.len() - 1 {
            merged_ranges.push(curr_range.clone());
        }
    }

    let mut valid_id_count = 0;
    for range in merged_ranges.into_iter() {
        valid_id_count += range.count();
    }

    cprintln!("Total valid ID count: <b>{}</>", valid_id_count);
}
