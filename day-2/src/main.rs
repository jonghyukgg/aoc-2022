use std::fs;

fn wdl_score_1(line: &str) -> usize {
    match line {
        "A X" => 3,
        "A Y" => 6,
        "A Z" => 0,
        "B X" => 0,
        "B Y" => 3,
        "B Z" => 6,
        "C X" => 6,
        "C Y" => 0,
        "C Z" => 3,
        _ => panic!(),
    }
}

fn rps_score_1(mine: &str) -> usize {
    match mine {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!(),
    }
}

fn rps_score_2(line: &str) -> usize {
    match line {
        "A X" => 3,
        "A Y" => 1,
        "A Z" => 2,
        "B X" => 1,
        "B Y" => 2,
        "B Z" => 3,
        "C X" => 2,
        "C Y" => 3,
        "C Z" => 1,
        _ => panic!(),
    }
}

fn wdl_score_2(op: &str) -> usize {
    match op {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!(),
    }
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

fn part1() {
    let input = read_input("input.txt");
    let res = input
        .split("\n")
        .map(|line| wdl_score_1(line) + rps_score_1(&line[2..3]))
        .sum::<usize>();
    println!("{}", res);
}

fn part2() {
    let input = read_input("input.txt");
    let res = input
        .split("\n")
        .map(|line| rps_score_2(line) + wdl_score_2(&line[2..3]))
        .sum::<usize>();
    println!("{}", res);
}

fn main() {
    part1();
    part2();
}
