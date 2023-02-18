use std::fs;
use std::time::Instant;

// const PART1_FILENAME: &str = "sample.txt";
const PART1_FILENAME: &str = "input.txt";

// const PART2_FILENAME: &str = "sample.txt";
const PART2_FILENAME: &str = "input.txt";
const PART2_KEY: i64 = 811589153;
const PART2_ITER: usize = 10;

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

#[allow(dead_code)]
fn disp(v: &Vec<(usize, i64)>) {
    print!("( ");
    for (_, x) in v.iter() {
        print!("{} ", x);
    }
    println!(")");
}

fn index_after(idx: usize, d: i64, n: usize) -> usize {
    let idx = idx as i64;
    let n = n as i64;
    let d = d % (n - 1);
    if d < 0 {
        if -d <= idx {
            (idx + d) as usize
        } else {
            (n - 1 - (-d - idx)) as usize
        }
    } else {
        if d < n - idx {
            (idx + d) as usize
        } else {
            (d - (n - 1 - idx)) as usize
        }
    }
}

fn part1() {
    let input = read_input(PART1_FILENAME);
    let mut v: Vec<(usize, i64)> = input
        .split("\n")
        .enumerate()
        .map(|(idx, line)| (idx, line.parse::<i64>().unwrap()))
        .collect();
    let n: usize = v.len();
    for i in 0..n {
        // disp(&v);
        let mut idx = v.iter().position(|x| x.0 == i).unwrap();
        let target_idx = index_after(idx, v[idx].1, n);
        if target_idx < idx {
            for _ in 0..(idx - target_idx) {
                (v[idx], v[idx - 1]) = (v[idx - 1], v[idx]);
                idx -= 1;
            }
        } else if target_idx > idx {
            for _ in 0..(target_idx - idx) {
                (v[idx], v[idx + 1]) = (v[idx + 1], v[idx]);
                idx += 1;
            }
        }
    }
    // disp(&v);
    let zero_idx = v.iter().position(|x| x.1 == 0).unwrap();
    let v1 = v[(zero_idx + 1000) % n].1;
    let v2 = v[(zero_idx + 2000) % n].1;
    let v3 = v[(zero_idx + 3000) % n].1;
    println!("{}", v1 + v2 + v3);
}

fn part2() {
    let input = read_input(PART2_FILENAME);
    let mut v: Vec<(usize, i64)> = input
        .split("\n")
        .enumerate()
        .map(|(idx, line)| (idx, line.parse::<i64>().unwrap() * PART2_KEY))
        .collect();
    let n: usize = v.len();
    for _ in 0..PART2_ITER {
        for i in 0..n {
            // disp(&v);
            let mut idx = v.iter().position(|x| x.0 == i).unwrap();
            let target_idx = index_after(idx, v[idx].1, n);
            if target_idx < idx {
                for _ in 0..(idx - target_idx) {
                    (v[idx], v[idx - 1]) = (v[idx - 1], v[idx]);
                    idx -= 1;
                }
            } else if target_idx > idx {
                for _ in 0..(target_idx - idx) {
                    (v[idx], v[idx + 1]) = (v[idx + 1], v[idx]);
                    idx += 1;
                }
            }
        }
        // disp(&v);
    }
    let zero_idx = v.iter().position(|x| x.1 == 0).unwrap();
    let v1 = v[(zero_idx + 1000) % n].1;
    let v2 = v[(zero_idx + 2000) % n].1;
    let v3 = v[(zero_idx + 3000) % n].1;
    println!("{}", v1 + v2 + v3);
}

fn main() {
    let now = Instant::now();
    part1();
    println!("Part 1 elapsed: {:.2?}", now.elapsed());

    let now = Instant::now();
    part2();
    println!("Part 2 elapsed: {:.2?}", now.elapsed());
}
