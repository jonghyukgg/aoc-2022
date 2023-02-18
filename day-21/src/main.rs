use std::collections::HashMap;
use std::fs;
use std::time::Instant;

// const PART1_FILENAME: &str = "sample.txt";
const PART1_FILENAME: &str = "input.txt";

// const PART2_FILENAME: &str = "sample.txt";
const PART2_FILENAME: &str = "input.txt";
const PART2_MY_NAME: &str = "humn";

enum Job<'a> {
    Num(usize),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

fn parse_line(line: &str) -> (&str, Job) {
    let l: Vec<&str> = line.split(": ").collect();
    assert!(l.len() == 2);
    let j: Vec<&str> = l[1].split(" ").collect();
    (
        l[0],
        if j.len() == 1 {
            Job::Num(j[0].parse::<usize>().unwrap())
        } else {
            match j[1] {
                "+" => Job::Add(j[0], j[2]),
                "-" => Job::Sub(j[0], j[2]),
                "*" => Job::Mul(j[0], j[2]),
                "/" => Job::Div(j[0], j[2]),
                _ => panic!(),
            }
        },
    )
}

#[allow(dead_code)]
fn disp(monkeys: &HashMap<&str, Job>) {
    for (name, job) in monkeys.iter() {
        print!("{}: ", name);
        match job {
            Job::Num(x) => print!("{}", x),
            Job::Add(a, b) => print!("{} + {}", a, b),
            Job::Sub(a, b) => print!("{} - {}", a, b),
            Job::Mul(a, b) => print!("{} * {}", a, b),
            Job::Div(a, b) => print!("{} / {}", a, b),
        };
        println!();
    }
}

fn part1() {
    let input = read_input(PART1_FILENAME);
    let monkeys: HashMap<&str, Job> = input.split("\n").map(|line| parse_line(line)).collect();
    // disp(&monkeys);

    let mut yell: HashMap<&str, usize> = HashMap::new();
    while yell.len() < monkeys.len() {
        for (name, job) in monkeys.iter() {
            if yell.contains_key(name) {
                continue;
            }
            match job {
                Job::Num(x) => {
                    yell.insert(name, *x);
                }
                Job::Add(a, b) => {
                    if yell.contains_key(a) && yell.contains_key(b) {
                        yell.insert(name, yell.get(a).unwrap() + yell.get(b).unwrap());
                    }
                }
                Job::Sub(a, b) => {
                    if yell.contains_key(a) && yell.contains_key(b) {
                        yell.insert(name, yell.get(a).unwrap() - yell.get(b).unwrap());
                    }
                }
                Job::Mul(a, b) => {
                    if yell.contains_key(a) && yell.contains_key(b) {
                        yell.insert(name, yell.get(a).unwrap() * yell.get(b).unwrap());
                    }
                }
                Job::Div(a, b) => {
                    if yell.contains_key(a) && yell.contains_key(b) {
                        yell.insert(name, yell.get(a).unwrap() / yell.get(b).unwrap());
                    }
                }
            }
        }
    }
    println!("{}", yell["root"]);
}

fn search(
    monkeys: &HashMap<&str, Job>,
    yell: &HashMap<&str, Option<usize>>,
    name: &str,
    x: usize,
) -> usize {
    if name == PART2_MY_NAME {
        return x;
    }
    match monkeys[name] {
        Job::Num(_) => panic!("invalid input"),
        Job::Add(a, b) => match (yell[a], yell[b]) {
            (None, None) | (Some(_), Some(_)) => panic!("invalid input"),
            (Some(aa), None) => search(monkeys, yell, b, x - aa),
            (None, Some(bb)) => search(monkeys, yell, a, x - bb),
        },
        Job::Sub(a, b) => match (yell[a], yell[b]) {
            (None, None) | (Some(_), Some(_)) => panic!("invalid input"),
            (Some(aa), None) => search(monkeys, yell, b, aa - x),
            (None, Some(bb)) => search(monkeys, yell, a, x + bb),
        },
        Job::Mul(a, b) => match (yell[a], yell[b]) {
            (None, None) | (Some(_), Some(_)) => panic!("invalid input"),
            (Some(aa), None) => {
                assert!(x % aa == 0);
                search(monkeys, yell, b, x / aa)
            }
            (None, Some(bb)) => {
                assert!(x % bb == 0);
                search(monkeys, yell, a, x / bb)
            }
        },
        Job::Div(a, b) => match (yell[a], yell[b]) {
            (None, None) | (Some(_), Some(_)) => panic!("invalid input"),
            // Assumes division w/o remainder
            (Some(aa), None) => {
                assert!(aa % x == 0);
                search(monkeys, yell, b, aa / x)
            }
            (None, Some(bb)) => search(monkeys, yell, a, bb * x),
        },
    }
}

fn part2() {
    let input = read_input(PART2_FILENAME);
    let mut monkeys: HashMap<&str, Job> = input.split("\n").map(|line| parse_line(line)).collect();
    // disp(&monkeys);
    // change "root" to Sub. a=b <=> a-b=0
    monkeys.insert(
        "root",
        match monkeys["root"] {
            Job::Num(_) => panic!("invalid input"),
            Job::Add(a, b) | Job::Sub(a, b) | Job::Mul(a, b) | Job::Div(a, b) => Job::Sub(a, b),
        },
    );
    assert!(matches!(monkeys[PART2_MY_NAME], Job::Num(_)));

    let mut yell: HashMap<&str, Option<usize>> = HashMap::new();
    while yell.len() < monkeys.len() {
        for (name, job) in monkeys.iter() {
            if yell.contains_key(name) {
                continue;
            }
            match job {
                Job::Num(x) => {
                    if *name == PART2_MY_NAME {
                        yell.insert(name, None);
                    } else {
                        yell.insert(name, Some(*x));
                    }
                }
                Job::Add(a, b) => {
                    if yell.contains_key(a) && yell.contains_key(b) {
                        if let (Some(x), Some(y)) = (yell.get(a).unwrap(), yell.get(b).unwrap()) {
                            yell.insert(name, Some(x + y));
                        } else {
                            yell.insert(name, None);
                        }
                    }
                }
                Job::Sub(a, b) => {
                    if yell.contains_key(a) && yell.contains_key(b) {
                        if let (Some(x), Some(y)) = (yell.get(a).unwrap(), yell.get(b).unwrap()) {
                            yell.insert(name, Some(x - y));
                        } else {
                            yell.insert(name, None);
                        }
                    }
                }
                Job::Mul(a, b) => {
                    if yell.contains_key(a) && yell.contains_key(b) {
                        if let (Some(x), Some(y)) = (yell.get(a).unwrap(), yell.get(b).unwrap()) {
                            yell.insert(name, Some(x * y));
                        } else {
                            yell.insert(name, None);
                        }
                    }
                }
                Job::Div(a, b) => {
                    if yell.contains_key(a) && yell.contains_key(b) {
                        if let (Some(x), Some(y)) = (yell.get(a).unwrap(), yell.get(b).unwrap()) {
                            yell.insert(name, Some(x / y));
                        } else {
                            yell.insert(name, None);
                        }
                    }
                }
            }
        }
    }

    let res = search(&monkeys, &yell, "root", 0);
    println!("{}", res);
}

fn main() {
    let now = Instant::now();
    part1();
    println!("Part 1 elapsed: {:.2?}", now.elapsed());

    let now = Instant::now();
    part2();
    println!("Part 2 elapsed: {:.2?}", now.elapsed());
}
