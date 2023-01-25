use std::cmp::Reverse;
use std::fs;

fn get_calories(filename: &str) -> Vec<usize> {
    fs::read_to_string(filename)
        .expect("Input for the problem")
        .split("\n\n")
        .map(|items| {
            items
                .split("\n")
                .map(|item| item.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect()
}

fn part1() {
    let calories: Vec<usize> = get_calories("input.txt");
    let res = calories.iter().max().unwrap();
    println!("{}", res);
}

fn part2() {
    let mut calories: Vec<usize> = get_calories("input.txt");
    calories.sort_by_key(|v| Reverse(*v));
    let res = calories.iter().take(3).sum::<usize>();
    println!("{}", res);
}

fn main() {
    part1();
    part2();
}
