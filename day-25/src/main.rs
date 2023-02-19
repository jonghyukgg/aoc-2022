use std::fs;
use std::ops::AddAssign;
use std::time::Instant;

// const PART1_FILENAME: &str = "sample.txt";
const PART1_FILENAME: &str = "input.txt";

struct SNAFU {
    digits: Vec<char>,
}

impl std::fmt::Display for SNAFU {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.digits.iter().rev().collect::<String>())
    }
}

impl SNAFU {
    pub fn new(digits: &str) -> Self {
        SNAFU {
            digits: digits.chars().rev().collect(),
        }
    }
    pub fn get_digit_val(&self, idx: usize) -> i32 {
        if self.digits.len() <= idx {
            0
        } else {
            match self.digits[idx] {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => panic!(),
            }
        }
    }
}

impl AddAssign for SNAFU {
    fn add_assign(&mut self, other: Self) {
        let l = std::cmp::max(self.digits.len(), other.digits.len());
        let mut c = 0; // carry
        for idx in 0..l {
            let v = self.get_digit_val(idx) + other.get_digit_val(idx) + c;
            let new_digit: char;
            (c, new_digit) = match v {
                -5 => (-1, '0'),
                -4 => (-1, '1'),
                -3 => (-1, '2'),
                -2 => (0, '='),
                -1 => (0, '-'),
                0 => (0, '0'),
                1 => (0, '1'),
                2 => (0, '2'),
                3 => (1, '='),
                4 => (1, '-'),
                5 => (1, '0'),
                _ => panic!(),
            };
            if self.digits.len() == idx {
                self.digits.push(new_digit);
            } else {
                self.digits[idx] = new_digit;
            }
        }
        assert!(c == 0 || c == 1);
        if c == 1 {
            self.digits.push('1');
        }
    }
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

fn part1() {
    let input = read_input(PART1_FILENAME);
    let snafu_list: Vec<SNAFU> = input.split("\n").map(|line| SNAFU::new(&line)).collect();
    let mut res = SNAFU::new("0");
    for s in snafu_list {
        res += s;
    }
    println!("{}", res);
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
