use std::{
    cmp::Reverse, collections::BinaryHeap, fs::read_to_string, num::ParseIntError, str::FromStr,
};

use color_print::cprintln;

const NUMBER_EDGES: usize = 1000;

pub fn day_08() {
    let input = parse();

    part1(&input);
    part2(&input);
}

#[derive(Debug, Clone)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn dist_squared(&self, rhs: &JunctionBox) -> u64 {
        self.x.abs_diff(rhs.x).pow(2)
            + self.y.abs_diff(rhs.y).pow(2)
            + self.z.abs_diff(rhs.z).pow(2)
    }
}

impl FromStr for JunctionBox {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let list = s.split(",").collect::<Vec<_>>();

        Ok(JunctionBox {
            x: i64::from_str(list[0])?,
            y: i64::from_str(list[1])?,
            z: i64::from_str(list[2])?,
        })
    }
}

struct UnionFind {
    // Parent index
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

// TODO: Optimize with tree flattening
impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parents: (0..size).collect(),
            sizes: vec![1; size],
        }
    }

    fn union(&mut self, i: usize, j: usize) {
        let i_parent = self.find(i);
        let j_parent = self.find(j);

        if i_parent == j_parent {
            return;
        }

        if self.sizes[i_parent] < self.sizes[j_parent] {
            self.parents[i_parent] = j_parent;
            self.sizes[j_parent] += self.sizes[i_parent];
        } else {
            self.parents[j_parent] = i_parent;
            self.sizes[i_parent] += self.sizes[j_parent];
        }
    }

    fn find(&mut self, index: usize) -> usize {
        assert!(index < self.parents.len());

        if self.parents[index] == index {
            return index;
        }

        self.parents[index] = self.find(self.parents[index]);
        self.parents[index]
    }

    fn product_of_three_largest(&self) -> usize {
        // println!("{:?}", self.parents);
        let mut binary_heap: BinaryHeap<usize> = BinaryHeap::new();
        binary_heap.extend(self.sizes.iter());

        binary_heap.pop().unwrap() * binary_heap.pop().unwrap() * binary_heap.pop().unwrap()
    }

    fn is_fully_connected(&mut self) -> bool {
        // println!("{:?}", self.parents);
        for i in 0..(self.parents.len() - 1) {
            if self.find(i) != self.find(i + 1) {
                return false;
            }
        }

        true
    }
}

fn parse() -> Vec<JunctionBox> {
    read_to_string("./inputs/2025/day08.txt")
        .expect("Can't read 2025 Day 8")
        .lines()
        .map(|l| JunctionBox::from_str(l).unwrap())
        .collect()
}

fn part1(boxes: &[JunctionBox]) {
    // Compute weights of edges
    let mut pq = BinaryHeap::new();
    for (i, jb1) in boxes.iter().enumerate() {
        for (j, jb2) in boxes.iter().enumerate() {
            if j <= i {
                continue;
            }

            pq.push((Reverse(jb1.dist_squared(jb2)), i, j));
        }
    }

    let mut union_find = UnionFind::new(boxes.len());
    for _ in 0..NUMBER_EDGES {
        let (_, jb1_index, jb2_index) = pq.pop().unwrap();
        union_find.union(jb1_index, jb2_index);
    }

    cprintln!(
        "Product of 3 largest clusters: <b>{}</>",
        union_find.product_of_three_largest()
    );
}

fn part2(boxes: &[JunctionBox]) {
    // Compute weights of edges
    let mut pq = BinaryHeap::new();
    for (i, jb1) in boxes.iter().enumerate() {
        for (j, jb2) in boxes.iter().enumerate() {
            if j <= i {
                continue;
            }

            pq.push((Reverse(jb1.dist_squared(jb2)), i, j));
        }
    }

    let mut union_find = UnionFind::new(boxes.len());
    let mut x_products = 0;
    while !union_find.is_fully_connected() {
        let (_, jb1_index, jb2_index) = pq.pop().unwrap();
        union_find.union(jb1_index, jb2_index);
        x_products = boxes[jb1_index].x * boxes[jb2_index].x;
    }

    cprintln!(
        "Product of x coord that connects graph: <b>{}</>",
        x_products
    );
}
