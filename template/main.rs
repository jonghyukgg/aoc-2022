use std::fs;
use std::time::Instant;

const PART1_FILENAME: &str = "sample.txt";
// const PART1_FILENAME: &str = "input.txt";

const PART2_FILENAME: &str = "sample.txt";
// const PART2_FILENAME: &str = "input.txt";

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
    let now = Instant::now();
    part1();
    println!("Part 1 elapsed: {:.2?}", now.elapsed());

    let now = Instant::now();
    part2();
    println!("Part 2 elapsed: {:.2?}", now.elapsed());
}
