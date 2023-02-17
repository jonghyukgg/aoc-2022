use std::collections::{HashSet, VecDeque};
use std::fs;
use std::time::Instant;

// const PART1_FILENAME: &str = "sample.txt";
const PART1_FILENAME: &str = "input.txt";

// const PART2_FILENAME: &str = "sample.txt";
const PART2_FILENAME: &str = "input.txt";

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

fn parse_input(input: &String) -> Vec<(i32, i32, i32)> {
    input
        .split("\n")
        .map(|line| {
            let v: Vec<i32> = line.split(",").map(|v| v.parse::<i32>().unwrap()).collect();
            assert!(v.len() == 3);
            (v[0], v[1], v[2])
        })
        .collect()
}

fn adjacent_cubes(c: &(i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    vec![
        (c.0 - 1, c.1, c.2),
        (c.0, c.1 - 1, c.2),
        (c.0, c.1, c.2 - 1),
        (c.0 + 1, c.1, c.2),
        (c.0, c.1 + 1, c.2),
        (c.0, c.1, c.2 + 1),
    ]
}

fn input_area(cubes: &Vec<(i32, i32, i32)>) -> ((i32, i32, i32), (i32, i32, i32)) {
    let (mut x1, mut x2, mut y1, mut y2, mut z1, mut z2) = (
        cubes[0].0, cubes[0].0, cubes[0].1, cubes[0].1, cubes[0].2, cubes[0].2,
    );
    for c in cubes.iter() {
        if c.0 < x1 {
            x1 = c.0;
        }
        if c.0 > x2 {
            x2 = c.0;
        }
        if c.1 < y1 {
            y1 = c.1;
        }
        if c.1 > y2 {
            y2 = c.1;
        }
        if c.2 < z1 {
            z1 = c.2;
        }
        if c.2 > z2 {
            z2 = c.2;
        }
    }
    ((x1, y1, z1), (x2, y2, z2))
}

fn part0() {
    let input = read_input("input.txt");
    let cubes = parse_input(&input);
    let ((x1, y1, z1), (x2, y2, z2)) = input_area(&cubes);
    println!(
        "input size: ({}, {}, {}) ~ ({}, {}, {})",
        x1, y1, z1, x2, y2, z2
    );
}

fn part1() {
    let input = read_input(PART1_FILENAME);
    let cubes = parse_input(&input);
    let m: HashSet<(i32, i32, i32)> = cubes.iter().cloned().collect();
    let mut res = cubes.len() * 6;
    for c in cubes.iter() {
        for adj_c in adjacent_cubes(&c).iter() {
            if m.contains(adj_c) {
                res -= 1;
            }
        }
    }
    println!("{}", res);
}

fn part2() {
    let input = read_input(PART2_FILENAME);
    let cubes = parse_input(&input);
    let ((x1, y1, z1), (x2, y2, z2)) = input_area(&cubes);
    let m: HashSet<(i32, i32, i32)> = cubes.iter().cloned().collect();
    let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut q: VecDeque<(i32, i32, i32)> = VecDeque::from([(x1 - 1, y1 - 1, z2 - 1)]);
    let mut res = 0;
    while !q.is_empty() {
        let c = q.pop_front().unwrap();
        for adj_c in adjacent_cubes(&c).iter() {
            if !(x1 - 1 <= adj_c.0
                && adj_c.0 <= x2 + 1
                && y1 - 1 <= adj_c.1
                && adj_c.1 <= y2 + 1
                && z1 - 1 <= adj_c.2
                && adj_c.2 <= z2 + 1)
            {
                continue;
            }
            if m.contains(&adj_c) {
                res += 1;
            } else if !visited.contains(&adj_c) {
                q.push_back(adj_c.clone());
                visited.insert(adj_c.clone());
            }
        }
    }
    println!("{}", res);
}

fn main() {
    part0();

    let now = Instant::now();
    part1();
    println!("Part 1 elapsed: {:.2?}", now.elapsed());

    let now = Instant::now();
    part2();
    println!("Part 2 elapsed: {:.2?}", now.elapsed());
}
