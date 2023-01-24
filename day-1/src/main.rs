use std::cmp::Reverse;
use std::fs;

fn get_calories(filename: &str) -> Vec<i32> {
    fs::read_to_string(filename)
        .expect("Input for the problem")
        .split("\n\n")
        .map(|items| {
            items
                .split("\n")
                .map(|item| item.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect()
}

fn part1() {
    let calories: Vec<i32> = get_calories("input1.txt");
    let res = calories.iter().max().unwrap();
    println!("{}", res);
}

fn part2() {
    let mut calories: Vec<i32> = get_calories("input2.txt");
    calories.sort_by_key(|v| Reverse(*v));
    let res = calories.iter().take(3).sum::<i32>();
    println!("{}", res);
}

fn main() {
    part1();
    part2();
}
