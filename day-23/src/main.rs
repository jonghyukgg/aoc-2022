use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

// const PART1_FILENAME: &str = "sample.txt";
const PART1_FILENAME: &str = "input.txt";
const PART1_ROUNDS: usize = 10;

// const PART2_FILENAME: &str = "sample.txt";
const PART2_FILENAME: &str = "input.txt";

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

fn parse_input(input: &String) -> HashSet<(i32, i32)> {
    let mut elves: HashSet<(i32, i32)> = HashSet::new();
    input.split("\n").enumerate().for_each(|(r, line)| {
        line.chars().enumerate().for_each(|(c, v)| {
            if v == '#' {
                elves.insert((r as i32, c as i32));
            }
        })
    });
    elves
}

fn get_next_position(e: &(i32, i32), elves: &HashSet<(i32, i32)>, d_offset: usize) -> Option<(i32, i32)> {
    let ex = [
        elves.contains(&(e.0-1, e.1-1)) || // N
        elves.contains(&(e.0-1, e.1)) || 
        elves.contains(&(e.0-1, e.1+1)),
        elves.contains(&(e.0+1, e.1-1)) || // S
        elves.contains(&(e.0+1, e.1)) || 
        elves.contains(&(e.0+1, e.1+1)),
        elves.contains(&(e.0-1, e.1-1)) || // W
        elves.contains(&(e.0, e.1-1)) || 
        elves.contains(&(e.0+1, e.1-1)),
        elves.contains(&(e.0-1, e.1+1)) || // E
        elves.contains(&(e.0, e.1+1)) || 
        elves.contains(&(e.0+1, e.1+1)),
    ];
    let d = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    if !ex[0] && !ex[1] && !ex[2] && !ex[3] {
        return None;
    }
    for d_idx in 0..4 {
        if !ex[(d_idx+d_offset)%4] {
            return Some((
                e.0+d[(d_idx+d_offset)%4].0,
                e.1+d[(d_idx+d_offset)%4].1,
            ));
        }
    }
    None
}

#[allow(dead_code)]
fn disp(elves: &HashSet<(i32, i32)>) {
    let x1 = elves.iter().map(|e| e.0).min().unwrap();
    let y1 = elves.iter().map(|e| e.1).min().unwrap();
    let x2 = elves.iter().map(|e| e.0).max().unwrap();
    let y2 = elves.iter().map(|e| e.1).max().unwrap();
    for x in x1..=x2 {
        for y in y1..=y2 {
            if elves.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn part1() {
    let input = read_input(PART1_FILENAME);
    let mut elves = parse_input(&input);
    // disp(&elves);

    for round in 0..PART1_ROUNDS {
        let mut next_moves: HashMap<(i32, i32), i32> = HashMap::new();
        for e in elves.iter() {
            let next_e = get_next_position(e, &elves, round%4);
            if let Some(v) = next_e {
                *next_moves.entry(v).or_insert(0) += 1;
            }
        }
        let mut next_elves: HashSet<(i32, i32)> = HashSet::new();
        for e in elves.iter() {
            let next_e = get_next_position(e, &elves, round%4);
            if let Some(v) = next_e {
                if next_moves[&v] == 1 {
                    next_elves.insert((v.0, v.1));
                } else {
                    next_elves.insert((e.0, e.1));
                }
            } else {
                next_elves.insert((e.0, e.1));
            }
        }
        elves = next_elves;
        // disp(&elves);
    }

    let x1 = elves.iter().map(|e| e.0).min().unwrap();
    let y1 = elves.iter().map(|e| e.1).min().unwrap();
    let x2 = elves.iter().map(|e| e.0).max().unwrap();
    let y2 = elves.iter().map(|e| e.1).max().unwrap();
    
    let res = (x2-x1+1)*(y2-y1+1) - (elves.len() as i32);
    println!("{}", res);
}

fn part2() {
    let input = read_input(PART2_FILENAME);
    let mut elves = parse_input(&input);
    // disp(&elves);

    let mut round = 0;
    loop {
        let mut next_moves: HashMap<(i32, i32), i32> = HashMap::new();
        for e in elves.iter() {
            let next_e = get_next_position(e, &elves, round%4);
            if let Some(v) = next_e {
                *next_moves.entry(v).or_insert(0) += 1;
            }
        }
        let mut moved_count = 0;
        let mut next_elves: HashSet<(i32, i32)> = HashSet::new();
        for e in elves.iter() {
            let next_e = get_next_position(e, &elves, round%4);
            if let Some(v) = next_e {
                if next_moves[&v] == 1 {
                    next_elves.insert((v.0, v.1));
                    moved_count += 1;
                } else {
                    next_elves.insert((e.0, e.1));
                }
            } else {
                next_elves.insert((e.0, e.1));
            }
        }
        round += 1;
        if moved_count == 0 {
            println!("{}", round);
            return;
        }
        elves = next_elves;
    }
}

fn main() {
    let now = Instant::now();
    part1();
    println!("Part 1 elapsed: {:.2?}", now.elapsed());

    let now = Instant::now();
    part2();
    println!("Part 2 elapsed: {:.2?}", now.elapsed());
}
