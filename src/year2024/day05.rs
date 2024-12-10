use std::{cmp::Ordering, collections::{HashMap, HashSet}, fs::read_to_string};

use color_print::cprintln;

pub fn day_05() {
    let mut input = parse();
    part1(&input);
    part2(&mut input);
}

type Input = (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>);

fn parse() -> Input {
    let input = read_to_string("inputs/2024/day05.txt").expect("Cannot open 2024 day 5");
    let (map_input, update_input) = input.split_once("\r\n\r\n").unwrap();
    
    let mut ordering_map: HashMap<usize, Vec<usize>> = HashMap::new();
    map_input.lines().map(|line| line.split("|").collect::<Vec<&str>>()).for_each(|pair| {
        assert!(pair.len() == 2);
        let (key, val): (usize, usize) = (pair[0].parse().unwrap(), pair[1].parse().unwrap());
        if !ordering_map.contains_key(&key) {
            ordering_map.insert(key, Vec::new());
        }

        ordering_map.get_mut(&key).unwrap().push(val);
    });

    let update_list = update_input.lines()
        .map(|line| line.split(",").map(|page| page.parse().unwrap()).collect()).collect();

    (ordering_map, update_list)
}

fn part1(input: &Input) {
    let (ordering_map, updates) = input;
    let result: usize = updates.iter().filter_map(|update| {
        let mut seen: HashSet<usize> = HashSet::new();
        for page_num in update {
            let pages = ordering_map.get(page_num);
            if let Some(pages_not_allowed_to_follow) = pages {
                for num in pages_not_allowed_to_follow {
                    if seen.contains(num) {
                        return None;
                    }
                }
            }
            seen.insert(*page_num);
        }

        Some(update[update.len() / 2])
    }).sum();

    cprintln!("Sum of properly ordered updates middle print job: <b>{}</>", result);
}

fn part2(input: &mut Input) {
    let (ordering_map, updates) = input;
    let orders: Vec<(usize, usize)> = ordering_map.iter().flat_map(|orders| {
        orders.1.iter().map(|&following| (*orders.0, following)).collect::<Vec<(usize, usize)>>()
    }).collect();

    let result: usize = updates.iter_mut().filter(|update| {
        let mut seen: HashSet<usize> = HashSet::new();
        for page_num in update.iter() {
            let pages = ordering_map.get(&page_num);
            if let Some(pages_not_allowed_to_follow) = pages {
                for num in pages_not_allowed_to_follow {
                    if seen.contains(num) {
                        return true;
                    }
                }
            }
            seen.insert(*page_num);
        }

        false
    }).map(|bad_page| {
        bad_page.sort_by(|&a, &b| {
            if orders.contains(&(a,b)) {
                Ordering::Less
            } else if orders.contains(&(b,a)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        bad_page[bad_page.len() / 2]
    }).sum();

    cprintln!("Sum of inproperly ordered updates middle print job: <b>{}</>", result);
}