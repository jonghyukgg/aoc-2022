use std::collections::HashSet;
use std::fs;

fn g(p1: &str, p2: &str) -> char {
    for c1 in p1.chars() {
        for c2 in p2.chars() {
            if c1 == c2 {
                return c1;
            }
        }
    }
    panic!("Invalid input");
}

fn f(line: &str) -> i32 {
    let p1: &str = &line[..line.len() / 2];
    let p2: &str = &line[line.len() / 2..];
    let s = g(p1, p2);
    if s.is_lowercase() {
        s as i32 - 96
    } else {
        s as i32 - 64 + 26
    }
}

fn part1() {
    let res = fs::read_to_string("input1.txt")
        .expect("Input for the problem")
        .split("\n")
        .map(|line| f(&line))
        .sum::<i32>();
    println!("{}", res);
}

fn h(lines: &Vec<&str>) -> i32 {
    let mut chars: HashSet<char> = HashSet::new();
    for c in 'a'..='z' {
        chars.insert(c);
    }
    for c in 'A'..='Z' {
        chars.insert(c);
    }
    let mut new_chars: HashSet<char>;
    for line in lines.iter() {
        new_chars = HashSet::new();
        for c in line.chars() {
            if chars.contains(&c) {
                new_chars.insert(c);
            }
        }
        chars = new_chars;
    }
    let c: char = chars.iter().next().unwrap().clone();
    if c.is_lowercase() {
        c as i32 - 96
    } else {
        c as i32 - 64 + 26
    }
}

fn part2() {
    let mut lines: Vec<&str> = vec![];
    let mut res: i32 = 0;
    for line in fs::read_to_string("input2.txt")
        .expect("Input for the problem")
        .split("\n")
    {
        lines.push(line);
        if lines.len() == 3 {
            res += h(&lines);
            lines.clear();
        }
    }
    println!("{}", res);
}

fn main() {
    part1();
    part2();
}
