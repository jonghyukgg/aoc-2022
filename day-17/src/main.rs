use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;

// const PART1_FILENAME: &str = "sample.txt";
const PART1_FILENAME: &str = "input.txt";
const PART1_ITER: usize = 2022;
const PART1_WIDTH: usize = 7;
const PART1_FROM_LEFT: usize = 2;
const PART1_FROM_BOTTOM: usize = 3;

// const PART2_FILENAME: &str = "sample.txt";
const PART2_FILENAME: &str = "input.txt";
// const PART2_ITER: u64 = 2022;
const PART2_ITER: u64 = 1000000000000;

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

fn get_block_shape(block_idx: usize) -> Vec<(usize, usize)> {
    match block_idx {
        0 => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        1 => vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        2 => vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
        3 => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        4 => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        _ => panic!(),
    }
}

fn does_overlap(y: usize, x: usize, block_idx: usize, rocks: &Vec<Vec<char>>) -> bool {
    let block_rocks = get_block_shape(block_idx);
    for (dy, dx) in block_rocks.iter() {
        let (by, bx) = (y + dy, x + dx);
        if by >= rocks.len() {
            continue;
        }
        if rocks[by][bx] == '#' {
            return true;
        }
    }
    return false;
}

fn jet(
    y: usize,
    x: usize,
    block_idx: usize,
    command: char,
    rocks: &Vec<Vec<char>>,
) -> (usize, usize) {
    match command {
        '>' => {
            let block_width = match block_idx {
                0 => 4,
                1 => 3,
                2 => 3,
                3 => 1,
                4 => 2,
                _ => panic!(),
            };
            if x + block_width == PART1_WIDTH {
                (y, x)
            } else {
                if does_overlap(y, x + 1, block_idx, rocks) {
                    (y, x)
                } else {
                    (y, x + 1)
                }
            }
        }
        '<' => {
            if x == 0 {
                (y, x)
            } else {
                if does_overlap(y, x - 1, block_idx, rocks) {
                    (y, x)
                } else {
                    (y, x - 1)
                }
            }
        }
        _ => {
            panic!()
        }
    }
}

fn does_rest(y: usize, x: usize, block_idx: usize, rocks: &Vec<Vec<char>>) -> bool {
    if y == 0 {
        return true;
    }
    does_overlap(y - 1, x, block_idx, rocks)
}

fn draw(y: usize, x: usize, block_idx: usize, rocks: &mut Vec<Vec<char>>) {
    let block_rocks = get_block_shape(block_idx);
    for (dy, dx) in block_rocks.iter() {
        let (by, bx) = (y + dy, x + dx);
        while by >= rocks.len() {
            rocks.push(vec!['.'; PART1_WIDTH]);
        }
        rocks[by][bx] = '#';
    }
}

#[allow(dead_code)]
fn display(rocks: &Vec<Vec<char>>) {
    for r in rocks.iter().rev() {
        println!("{}", r.iter().collect::<String>());
    }
    println!();
}

fn part1() {
    let input = read_input(PART1_FILENAME);
    let commands: Vec<char> = input.chars().collect();
    let mut command_idx = 0;
    let mut block_idx = 0;
    let mut rocks: Vec<Vec<char>> = vec![];
    for _ in 0..PART1_ITER {
        let mut y: usize = rocks.len() + PART1_FROM_BOTTOM;
        let mut x: usize = PART1_FROM_LEFT;
        loop {
            (y, x) = jet(y, x, block_idx, commands[command_idx], &rocks);
            command_idx = (command_idx + 1) % commands.len();

            if does_rest(y, x, block_idx, &rocks) {
                break;
            }
            y -= 1;
        }
        draw(y, x, block_idx, &mut rocks);
        // display(&rocks);
        block_idx = (block_idx + 1) % 5;
    }
    println!("{}", rocks.len());
}

#[rustfmt::skip]
lazy_static! {
    static ref PATTERNS: Vec<Vec<&'static str>> = vec![
        vec![
            "......#",
            "......#",
            "....###",
            "....#??",
            "...###?",
            "#####??",
        ],
        vec![
            "......#",
            "......#",
            "...####",
            "..###??",
            "...#???",
            "####???",
        ],
        vec![
            ".#....#",
            "###...#",
            "?#..###",
            "?####??",
        ],
        vec![
            "..#....",
            "..#..#.",
            "###.###",
            "?#####?",
        ],
        vec![
            "......#",
            ".#....#",
            "###.###",
            "?#####?",
        ],
        vec![
            "..#..#.",
            "..#.###",
            "###..#?",
            "??####?",
        ],
        vec![
            "..#....",
            "..#....",
            "###....",
            "??#....",
            "?###...",
            "??#####",
        ],
        vec![
            "..#....",
            "..#....",
            "####...",
            "??###..",
            "???#...",
            "???####",
        ],
    ];
}

fn is_closed_pattern(rocks: &Vec<Vec<char>>) -> Option<usize> {
    for (p_idx, p) in PATTERNS.iter().enumerate() {
        if rocks.len() < p.len() {
            continue;
        }
        let matches = (0..p.len()).all(|py| {
            let y = rocks.len() - 1 - py;
            for (x, pc) in p[py].chars().enumerate() {
                if pc == '?' {
                    continue;
                }
                if pc != rocks[y][x] {
                    return false;
                }
            }
            return true;
        });
        if matches {
            return Some(p_idx);
        }
    }
    return None;
}

fn part2() {
    let input = read_input(PART2_FILENAME);
    let commands: Vec<char> = input.chars().collect();
    let mut command_idx = 0;
    let mut block_idx = 0;
    let mut rocks: Vec<Vec<char>> = vec![];
    let mut lengths: Vec<usize> = vec![];
    let mut visited_patterns: HashMap<(usize, usize), usize> = HashMap::new();
    let mut idx = 0usize;
    loop {
        let mut y: usize = rocks.len() + PART1_FROM_BOTTOM;
        let mut x: usize = PART1_FROM_LEFT;
        loop {
            (y, x) = jet(y, x, block_idx, commands[command_idx], &rocks);
            command_idx = (command_idx + 1) % commands.len();

            if does_rest(y, x, block_idx, &rocks) {
                break;
            }
            y -= 1;
        }
        draw(y, x, block_idx, &mut rocks);
        lengths.push(rocks.len());

        if block_idx == 2 {
            let closed_pattern_idx = is_closed_pattern(&rocks);
            if let Some(v) = closed_pattern_idx {
                if let Some(prev_idx) = visited_patterns.get(&(v, command_idx)) {
                    let (i1, i2) = (*prev_idx as u64, idx as u64);
                    let (l1, l2) = (lengths[*prev_idx] as u64, rocks.len() as u64);
                    let offset_idx = (i1 + (PART2_ITER - 1 - i1) % (i2 - i1)) as usize;
                    let offset_val: u64 = lengths[offset_idx] as u64;
                    let res = offset_val + (PART2_ITER - 1 - i1) / (i2 - i1) * (l2 - l1);
                    println!("{}", res);
                    break;
                }
                visited_patterns.insert((v, command_idx), idx);
            }
        }

        block_idx = (block_idx + 1) % 5;
        idx += 1;
    }
}

fn main() {
    part1();
    part2();
}
