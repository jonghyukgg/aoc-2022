use std::collections::HashSet;
use std::fs;

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

fn part1() {
    let input = read_input("input.txt");
    let mut visited = HashSet::from([(0i32, 0i32)]);
    let mut h_pos = (0i32, 0i32);
    let mut t_pos = (0i32, 0i32);
    // brute-force works!
    input.split("\n").for_each(|line| {
        let line_parse: Vec<&str> = line.split(" ").collect();
        let (d, c) = (line_parse[0].clone(), line_parse[1].parse::<i32>().unwrap());
        for _ in 0..c {
            match d {
                "R" => h_pos.1 += 1,
                "L" => h_pos.1 -= 1,
                "U" => h_pos.0 -= 1,
                "D" => h_pos.0 += 1,
                _ => panic!("Invalid input"),
            };
            // lol
            match (h_pos.0 - t_pos.0, h_pos.1 - t_pos.1) {
                (-2, -1) | (-1, -2) => {
                    t_pos.0 -= 1;
                    t_pos.1 -= 1;
                }
                (-2, 0) => t_pos.0 -= 1,
                (-2, 1) | (-1, 2) => {
                    t_pos.0 -= 1;
                    t_pos.1 += 1;
                }
                (0, -2) => t_pos.1 -= 1,
                (0, 2) => t_pos.1 += 1,
                (1, -2) | (2, -1) => {
                    t_pos.0 += 1;
                    t_pos.1 -= 1;
                }
                (2, 0) => t_pos.0 += 1,
                (1, 2) | (2, 1) => {
                    t_pos.0 += 1;
                    t_pos.1 += 1;
                }
                _ => (),
            };
            visited.insert(t_pos.clone());
        }
    });
    let res = visited.len();
    println!("{}", res);
}

fn part2() {
    let input = read_input("input.txt");
    let mut visited = HashSet::from([(0i32, 0i32)]);
    let mut knots = [(0i32, 0i32); 10];
    // brute-force works!
    input.split("\n").for_each(|line| {
        let line_parse: Vec<&str> = line.split(" ").collect();
        let (d, c) = (line_parse[0].clone(), line_parse[1].parse::<i32>().unwrap());
        for _ in 0..c {
            match d {
                "R" => knots[0].1 += 1,
                "L" => knots[0].1 -= 1,
                "U" => knots[0].0 -= 1,
                "D" => knots[0].0 += 1,
                _ => panic!("Invalid input"),
            };

            for i in 1..10 {
                match (knots[i - 1].0 - knots[i].0, knots[i - 1].1 - knots[i].1) {
                    (-2, -1) | (-1, -2) | (-2, -2) => {
                        knots[i].0 -= 1;
                        knots[i].1 -= 1;
                    }
                    (-2, 0) => knots[i].0 -= 1,
                    (-2, 1) | (-1, 2) | (-2, 2) => {
                        knots[i].0 -= 1;
                        knots[i].1 += 1;
                    }
                    (0, -2) => knots[i].1 -= 1,
                    (0, 2) => knots[i].1 += 1,
                    (1, -2) | (2, -1) | (2, -2) => {
                        knots[i].0 += 1;
                        knots[i].1 -= 1;
                    }
                    (2, 0) => knots[i].0 += 1,
                    (1, 2) | (2, 1) | (2, 2) => {
                        knots[i].0 += 1;
                        knots[i].1 += 1;
                    }
                    _ => (),
                };
            }
            visited.insert(knots[9].clone());
        }
    });
    let res = visited.len();
    println!("{}", res);
}

fn main() {
    part1();
    part2();
}
