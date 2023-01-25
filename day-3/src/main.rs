use std::collections::HashSet;
use std::fs;

fn get_shared(c1: &str, c2: &str) -> char {
    for i1 in c1.chars() {
        for i2 in c2.chars() {
            if i1 == i2 {
                return i1;
            }
        }
    }
    panic!("Invalid input");
}

fn item_score(item: &char) -> usize {
    if item.is_lowercase() {
        *item as usize - 96
    } else {
        *item as usize - 64 + 26
    }
}

fn process_rucksack(line: &str) -> usize {
    let c1: &str = &line[..line.len() / 2];
    let c2: &str = &line[line.len() / 2..];
    let shared = get_shared(c1, c2);
    item_score(&shared)
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

fn part1() {
    let input = read_input("input.txt");
    let res = input
        .split("\n")
        .map(|line| process_rucksack(&line))
        .sum::<usize>();
    println!("{}", res);
}

fn set_of_alphabets() -> HashSet<char> {
    let mut chars: HashSet<char> = HashSet::new();
    for c in 'a'..='z' {
        chars.insert(c);
    }
    for c in 'A'..='Z' {
        chars.insert(c);
    }
    chars
}

fn process_rucksack_group(lines: &Vec<&str>) -> usize {
    let mut chars = set_of_alphabets();
    let mut remaining_chars: HashSet<char>;
    for line in lines.iter() {
        remaining_chars = HashSet::new();
        for c in line.chars() {
            if chars.contains(&c) {
                remaining_chars.insert(c);
            }
        }
        chars = remaining_chars;
    }
    assert!(chars.len() == 1);
    let shared: char = chars.iter().next().unwrap().clone();
    item_score(&shared)
}

fn part2() {
    let input = read_input("input.txt");
    let mut lines_buffer: Vec<&str> = vec![];
    let mut res: usize = 0;
    for line in input.split("\n") {
        lines_buffer.push(line);
        if lines_buffer.len() == 3 {
            res += process_rucksack_group(&lines_buffer);
            lines_buffer.clear();
        }
    }
    println!("{}", res);
}

fn main() {
    part1();
    part2();
}
