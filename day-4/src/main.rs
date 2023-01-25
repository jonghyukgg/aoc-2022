use itertools::Itertools;
use std::fs;

fn get_ranges(filename: &str) -> Vec<((i32, i32), (i32, i32))> {
    fs::read_to_string(filename)
        .expect("Input for the problem")
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|range| {
                    range
                        .split("-")
                        .map(|num| num.parse::<i32>().unwrap().clone())
                        .next_tuple::<(i32, i32)>()
                        .unwrap()
                })
                .next_tuple::<((i32, i32), (i32, i32))>()
                .unwrap()
        })
        .collect()
}

fn range_contains(r1: &(i32, i32), r2: &(i32, i32)) -> bool {
    (r1.0 <= r2.0 && r2.1 <= r1.1) || (r2.0 <= r1.0 && r1.1 <= r2.1)
}

fn part1() {
    let ranges = get_ranges("input.txt");
    let res = ranges.iter().filter(|r| range_contains(&r.0, &r.1)).count();
    println!("{}", res);
}

fn range_intersects(r1: &(i32, i32), r2: &(i32, i32)) -> bool {
    (r1.0 <= r2.0 && r2.0 <= r1.1)
        || (r1.0 <= r2.1 && r2.1 <= r1.1)
        || (r2.0 <= r1.0 && r1.0 <= r2.1)
        || (r2.0 <= r1.1 && r1.1 <= r2.1)
}

fn part2() {
    let ranges = get_ranges("input.txt");
    let res = ranges
        .iter()
        .filter(|r| range_intersects(&r.0, &r.1))
        .count();
    println!("{}", res);
}

fn main() {
    part1();
    part2();
}
