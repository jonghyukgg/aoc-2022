use phf::phf_map;
use std::fs;

static WDL_SCORES: phf::Map<&'static str, i32> = phf_map! {
    "A X" => 3,
    "A Y" => 6,
    "A Z" => 0,
    "B X" => 0,
    "B Y" => 3,
    "B Z" => 6,
    "C X" => 6,
    "C Y" => 0,
    "C Z" => 3,
};

static RSP_SCORES: phf::Map<&'static str, i32> = phf_map! {
    "X" => 1,
    "Y" => 2,
    "Z" => 3,
};

static RSP_SCORES_2: phf::Map<&'static str, i32> = phf_map! {
    "A X" => 3,
    "A Y" => 1,
    "A Z" => 2,
    "B X" => 1,
    "B Y" => 2,
    "B Z" => 3,
    "C X" => 2,
    "C Y" => 3,
    "C Z" => 1,
};

static WDL_SCORES_2: phf::Map<&'static str, i32> = phf_map! {
    "X" => 0,
    "Y" => 3,
    "Z" => 6,
};

fn part1() {
    let res = fs::read_to_string("input1.txt")
        .expect("Input for the problem")
        .split("\n")
        .map(|line| WDL_SCORES[line] + RSP_SCORES[&line[2..3]])
        .sum::<i32>();
    println!("{}", res);
}

fn part2() {
    let res = fs::read_to_string("input2.txt")
        .expect("Input for the problem")
        .split("\n")
        .map(|line| RSP_SCORES_2[line] + WDL_SCORES_2[&line[2..3]])
        .sum::<i32>();
    println!("{}", res);
}

fn main() {
    part1();
    part2();
}
