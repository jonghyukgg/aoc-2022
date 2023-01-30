use std::{collections::VecDeque, fs};

enum Instruction {
    Noop,
    AddX(i32),
}

struct Execution {
    instruction: Instruction,
    duration: u32,
}

impl Execution {
    pub fn after_cycle(&mut self) {
        self.duration -= 1;
    }

    pub fn is_end(&self) -> bool {
        return self.duration == 0;
    }
}

struct Computer {
    lines: VecDeque<String>,
    x: i32,
    cycle: u32,
    execution: Option<Execution>,
}

impl Iterator for Computer {
    type Item = i32;

    // returns x during cycle
    fn next(&mut self) -> Option<Self::Item> {
        // leftover execution
        if self.execution.is_some() {
            let e = self.execution.as_mut().unwrap();
            e.after_cycle();
            if e.is_end() {
                match e.instruction {
                    Instruction::Noop => (),
                    Instruction::AddX(v) => self.x += v,
                }
                self.execution = None;
            }
        }
        self.cycle += 1;

        // before cycle
        if self.execution.is_none() {
            if self.lines.len() == 0 {
                return None;
            }
            let line = self.lines.pop_front().unwrap();
            let words: Vec<&str> = line.split(" ").collect();
            match words[0] {
                "noop" => {
                    self.execution = Some(Execution {
                        instruction: Instruction::Noop,
                        duration: 1,
                    });
                }
                "addx" => {
                    self.execution = Some(Execution {
                        instruction: Instruction::AddX(words[1].parse::<i32>().unwrap()),
                        duration: 2,
                    });
                }
                _ => {
                    panic!("Invalid input");
                }
            };
        }

        return Some(self.x.clone());
    }
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

fn part1() {
    let lines: VecDeque<String> = read_input("input.txt")
        .split("\n")
        .map(|line| String::from(line))
        .collect();
    let mut com = Computer {
        lines: lines,
        x: 1,
        cycle: 0,
        execution: None,
    };
    let mut res = 0;
    loop {
        let x = com.next();
        if x.is_none() {
            break;
        }
        if (com.cycle + 20) % 40 == 0 {
            let s = (com.cycle as i32) * x.unwrap();
            // println!("{} {}", c.cycle, s);
            res += s;
        }
    }

    println!("{}", res);
}

fn part2() {
    let lines: VecDeque<String> = read_input("input.txt")
        .split("\n")
        .map(|line| String::from(line))
        .collect();
    let mut com = Computer {
        lines: lines,
        x: 1,
        cycle: 0,
        execution: None,
    };
    let mut screen = [['.'; 40]; 6];
    for r in 0usize..6 {
        for c in 0usize..40 {
            let x = com.next().unwrap();
            if x - 1 <= (c as i32) && (c as i32) <= x + 1 {
                screen[r][c] = '#';
            }
        }
        println!("{}", screen[r].iter().collect::<String>());
    }
}

fn main() {
    part1();
    part2();
}
