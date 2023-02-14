use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

const PART1_TIME: usize = 30;
const PART1_FILE: &str = "input.txt";
const PART2_TIME: usize = 26;
const PART2_FILE: &str = "input.txt";

struct Valve {
    rate: usize,
    tunnels: Vec<usize>,
    opened: bool,
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

fn parse_line(line: &str) -> (&str, usize, Vec<&str>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^Valve (?P<name>[A-Z]+) has flow rate=(?P<rate>\d+); tunnels? leads? to valves? (?P<tunnels>.+)$"
        )
        .unwrap();
    }
    let caps = RE.captures(line).unwrap();
    (
        caps.name("name").unwrap().as_str(),
        caps.name("rate")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap(),
        caps.name("tunnels")
            .unwrap()
            .as_str()
            .split(", ")
            .collect::<Vec<&str>>(),
    )
}

fn parse_input(input: &String) -> (usize, Vec<Valve>) {
    let lines: Vec<(&str, usize, Vec<&str>)> =
        input.split("\n").map(|line| parse_line(line)).collect();

    let start_idx = lines.iter().position(|line| line.0 == "AA").unwrap();
    let mut name_to_idx: HashMap<&str, usize> = HashMap::new();
    lines.iter().enumerate().for_each(|(idx, line)| {
        name_to_idx.insert(line.0, idx);
    });
    (
        start_idx,
        lines
            .iter()
            .map(|line| Valve {
                rate: line.1,
                tunnels: line.2.iter().map(|name| name_to_idx[name]).collect(),
                opened: false,
            })
            .collect(),
    )
}

fn floyd_warshall(valves: &Vec<Valve>) -> Vec<Vec<usize>> {
    let n = valves.len();
    let mut d = vec![vec![std::usize::MAX; n]; n];
    for (idx, valve) in valves.iter().enumerate() {
        d[idx][idx] = 0;
        for t in valve.tunnels.iter() {
            d[idx][*t] = 1;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if d[i][k] == std::usize::MAX || d[k][j] == std::usize::MAX {
                    continue;
                }
                if d[i][j] > d[i][k] + d[k][j] {
                    d[i][j] = d[i][k] + d[k][j];
                }
            }
        }
    }
    d.iter()
        .for_each(|r| r.iter().for_each(|c| assert!(*c != std::usize::MAX)));
    d
}

fn search_part1(
    idx: usize,
    time: usize,
    opened_rate: usize,
    total: usize,
    valves: &mut Vec<Valve>,
    useful_valves: &Vec<usize>,
    distances: &Vec<Vec<usize>>,
    max_found: usize,
    max_rate: usize,
) -> usize {
    // branch & bound
    if total + (PART1_TIME - time + 1) * max_rate <= max_found {
        return max_found;
    }

    let mut res = max_found;

    // do nothing
    {
        let sub_res = total + opened_rate * (PART1_TIME - time + 1);
        res = if sub_res > res { sub_res } else { res };
    }

    for next_idx in useful_valves.iter() {
        let valve = &mut valves[*next_idx];
        if valve.opened {
            continue;
        }
        let dt = distances[idx][*next_idx] + 1;
        if time + dt > PART1_TIME {
            continue;
        }
        valve.opened = true;

        let sub_res = search_part1(
            *next_idx,
            time + dt,
            opened_rate + valve.rate,
            total + opened_rate * dt,
            valves,
            useful_valves,
            distances,
            res,
            max_rate,
        );
        res = if sub_res > res { sub_res } else { res };

        let valve = &mut valves[*next_idx]; // need to borrow again
        valve.opened = false;
    }
    res
}

fn get_max_pressure_part1(
    start_idx: usize,
    valves: &mut Vec<Valve>,
    distances: &Vec<Vec<usize>>,
) -> usize {
    let mut useful_valves: Vec<usize> = vec![];
    let mut max_rate = 0usize; // for branch & bound
    valves.iter().enumerate().for_each(|(idx, valve)| {
        if valve.rate > 0 {
            // take only useful valves
            useful_valves.push(idx);
            max_rate += valve.rate;
        }
    });

    search_part1(
        start_idx,
        1,
        0,
        0,
        valves,
        &useful_valves,
        distances,
        0,
        max_rate,
    )
}

fn part1() {
    let input = read_input(PART1_FILE);
    let (start_idx, mut valves) = parse_input(&input);
    let distances = floyd_warshall(&valves);
    let res = get_max_pressure_part1(start_idx, &mut valves, &distances);
    println!("{}", res);
}

fn search_part2(
    mut idx1: usize,
    mut hold1: usize,
    mut idx2: usize,
    mut hold2: usize,
    time: usize,
    mut opened_rate: usize,
    total: usize,
    valves: &mut Vec<Valve>,
    useful_valves: &Vec<usize>,
    distances: &Vec<Vec<usize>>,
    max_found: usize,
    max_rate: usize,
) -> usize {
    // branch & bound
    if total + (PART2_TIME - time + 1) * max_rate <= max_found {
        return max_found;
    }

    if hold1 > 0 && hold2 > 0 {
        let hold_time = if hold1 > hold2 { hold2 } else { hold1 };
        return search_part2(
            idx1,
            hold1 - hold_time,
            idx2,
            hold2 - hold_time,
            time + hold_time,
            opened_rate,
            total + hold_time * opened_rate,
            valves,
            useful_valves,
            distances,
            max_found,
            max_rate,
        );
    }

    if hold1 == 0 {
        // for the sake of simplicity
        (idx1, idx2) = (idx2, idx1);
        (hold1, hold2) = (hold2, hold1);
    }
    // now hold1 >= 0, hold2 == 0
    let opened1 = valves[idx1].opened;
    let opened2 = valves[idx2].opened;
    if hold1 == 0 && !valves[idx1].opened {
        valves[idx1].opened = true;
        opened_rate += valves[idx1].rate;
    }
    if hold2 == 0 && !valves[idx2].opened {
        valves[idx2].opened = true;
        opened_rate += valves[idx2].rate;
    }

    let mut res = max_found;

    // do nothing
    {
        let sub_res = if hold1 > 0 {
            total
                + hold1 * opened_rate
                + (PART2_TIME - time + 1 - hold1) * (opened_rate + valves[idx1].rate)
        } else {
            total + (PART2_TIME - time + 1) * opened_rate
        };
        res = if sub_res > res { sub_res } else { res };
    }

    for next_idx in useful_valves.iter() {
        if *next_idx == idx1 {
            continue;
        }
        let valve = &mut valves[*next_idx];
        if valve.opened {
            continue;
        }
        let dt = distances[idx2][*next_idx] + 1;
        if time + dt > PART2_TIME {
            continue;
        }

        let sub_res = search_part2(
            idx1,
            hold1,
            *next_idx,
            dt,
            time,
            opened_rate,
            total,
            valves,
            useful_valves,
            distances,
            res,
            max_rate,
        );
        res = if sub_res > res { sub_res } else { res };
    }

    valves[idx1].opened = opened1;
    valves[idx2].opened = opened2;

    res
}

fn get_max_pressure_part2(
    start_idx: usize,
    valves: &mut Vec<Valve>,
    distances: &Vec<Vec<usize>>,
) -> usize {
    let mut useful_valves: Vec<usize> = vec![];
    let mut max_rate = 0usize; // for branch & bound
    valves.iter().enumerate().for_each(|(idx, valve)| {
        if valve.rate > 0 && distances[start_idx][idx] + 1 < PART2_TIME {
            // take only useful valves
            useful_valves.push(idx);
            max_rate += valve.rate;
        }
    });

    search_part2(
        start_idx,
        0,
        start_idx,
        0,
        1,
        0,
        0,
        valves,
        &useful_valves,
        distances,
        0,
        max_rate,
    )
}

fn part2() {
    let input = read_input(PART2_FILE);
    let (start_idx, mut valves) = parse_input(&input);
    let distances = floyd_warshall(&valves);

    let res = get_max_pressure_part2(start_idx, &mut valves, &distances);
    println!("{}", res);
}

fn main() {
    let now = Instant::now();
    part1();
    println!("Part 1 elapsed: {:.2?}", now.elapsed());
    part2();
    println!("Part 2 elapsed: {:.2?}", now.elapsed());
}
