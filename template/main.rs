use std::fs;

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

fn part1() {
    println!("Hello, first star!");
}

fn part2() {
    println!("Hello, second star!");
}

fn main() {
    part1();
    part2();
}
