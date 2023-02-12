use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::collections::HashSet;
use std::fs;

struct Interval {
    // inclusive in this problem
    start: i32,
    end: i32,
}

struct Intervals {
    intervals: Vec<Interval>,
}

impl Interval {
    pub fn intersects(&self, i: &Interval) -> bool {
        // inclusive & cell-based coordinate
        (self.start - 1 <= i.start && i.start <= self.end + 1)
            || (self.start - 1 <= i.end && i.end <= self.end + 1)
            || (i.start - 1 <= self.start && self.start <= i.end + 1)
            || (i.start - 1 <= self.end && self.end <= i.end + 1)
    }
}

impl Intervals {
    pub fn add(&mut self, i: Interval) {
        if self.intervals.len() == 0 || self.intervals.last().unwrap().end < i.start {
            self.intervals.push(i);
            return;
        }
        // merging
        let mut intersect_idx: Option<(usize, usize)> = None;
        for (idx, interval) in self.intervals.iter().enumerate() {
            if i.intersects(interval) {
                intersect_idx = match intersect_idx {
                    None => Some((idx, idx)),
                    Some((v, _)) => Some((v, idx)),
                }
            }
        }
        let mut new_interval = Interval {
            start: i.start,
            end: i.end,
        };
        if let Some((v, w)) = intersect_idx {
            new_interval = Interval {
                start: cmp::min(self.intervals[v].start, i.start),
                end: cmp::max(self.intervals[w].end, i.end),
            };
            self.intervals.drain(v..=w);
        }
        // push new
        if self.intervals.len() == 0 || self.intervals.last().unwrap().end < new_interval.start {
            self.intervals.push(new_interval);
            return;
        }
        for (idx, interval) in self.intervals.iter().enumerate() {
            if new_interval.end < interval.start {
                self.intervals.insert(idx, new_interval);
                return;
            }
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for i in self.intervals.iter() {
            println!("{} ~ {}", i.start, i.end);
        }
    }
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)$"
        )
        .unwrap();
    }
    let caps = RE.captures(line).unwrap();
    let sx = caps.name("sx").unwrap().as_str().parse::<i32>().unwrap();
    let sy = caps.name("sy").unwrap().as_str().parse::<i32>().unwrap();
    let bx = caps.name("bx").unwrap().as_str().parse::<i32>().unwrap();
    let by = caps.name("by").unwrap().as_str().parse::<i32>().unwrap();
    ((sx, sy), (bx, by))
}

fn parse_input(input: &String) -> Vec<((i32, i32), (i32, i32))> {
    input.split("\n").map(|line| parse_line(line)).collect()
}

fn dist_1d(x1: &i32, x2: &i32) -> i32 {
    if x1 < x2 {
        x2 - x1
    } else {
        x1 - x2
    }
}

fn dist_2d_l1(x1: &i32, y1: &i32, x2: &i32, y2: &i32) -> i32 {
    dist_1d(x1, x2) + dist_1d(y1, y2)
}

fn part1() {
    // let (input, y_test) = (read_input("sample.txt"), 10);
    let (input, y_test) = (read_input("input.txt"), 2000000);
    let reports = parse_input(&input);

    let mut x_intervals: Intervals = Intervals { intervals: vec![] };
    let mut beacons_present = HashSet::new();

    for ((sx, sy), (bx, by)) in reports.iter() {
        let d = dist_2d_l1(sx, sy, bx, by);
        let dy = dist_1d(sy, &y_test);
        // y = y1 -> x1-d ~ x1+d
        // y = y1 +- dy (dy <= d) -> x1-(d-dy) ~ x2-(d-dy)
        if dy > d {
            continue;
        }
        if *by == y_test {
            beacons_present.insert(bx);
        }
        x_intervals.add(Interval {
            start: sx - (d - dy),
            end: sx + (d - dy),
        });
    }
    let res = x_intervals
        .intervals
        .iter()
        .map(|i| i.end - i.start + 1)
        .sum::<i32>()
        - beacons_present.len() as i32;
    println!("{}", res);
}

fn part2() {
    // let (input, y_max) = (read_input("sample.txt"), 20);
    let (input, y_max) = (read_input("input.txt"), 4000000);
    let reports = parse_input(&input);

    for y in 0..=y_max {
        let mut x_intervals: Intervals = Intervals { intervals: vec![] };
        for ((sx, sy), (bx, by)) in reports.iter() {
            let d = dist_2d_l1(sx, sy, bx, by);
            let dy = dist_1d(sy, &y);
            if dy > d {
                continue;
            }
            x_intervals.add(Interval {
                start: sx - (d - dy),
                end: sx + (d - dy),
            });
        }
        if x_intervals.intervals.len() > 1 {
            // println!("{}", y);
            // x_intervals.print();
            assert!(x_intervals.intervals.len() == 2);
            assert!(x_intervals.intervals[1].start - x_intervals.intervals[0].end == 2);
            let x = x_intervals.intervals[0].end + 1;
            let res = (x as i64) * 4000000i64 + (y as i64);
            println!("{}", res);
            return;
        }
    }
}

fn main() {
    part1();
    part2();
}
