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

fn part1() {
    let ranges = get_ranges("input1.txt");

    let res = ranges
        .iter()
        .filter(|r| {
            (r.0 .0 <= r.1 .0 && r.1 .1 <= r.0 .1) || (r.1 .0 <= r.0 .0 && r.0 .1 <= r.1 .1)
        })
        .count();

    println!("{}", res);
}

fn part2() {
    let ranges = get_ranges("input2.txt");

    let res = ranges
        .iter()
        .filter(|r| {
            (r.0 .0 <= r.1 .0 && r.1 .0 <= r.0 .1)
                || (r.0 .0 <= r.1 .1 && r.1 .1 <= r.0 .1)
                || (r.1 .0 <= r.0 .0 && r.0 .0 <= r.1 .1)
                || (r.1 .0 <= r.0 .1 && r.0 .1 <= r.1 .1)
        })
        .count();

    println!("{}", res);
}

fn main() {
    part1();
    part2();
}
