use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::time::Instant;

// const PART1_FILENAME: &str = "sample.txt";
const PART1_FILENAME: &str = "input.txt";

// const PART2_FILENAME: &str = "sample.txt";
// /*
// set 0 as leftmost face in top row
// ..0.
// 124.
// ..53
// */
// const PART2_CUBE_PATTERN: [(usize, usize); 6] = [(0, 2), (1, 0), (1, 1), (2, 3), (1, 2), (2, 2)];
// const PART2_CUBE_PATTERN_MOVES: [[(usize, usize); 4]; 6] = [
//     [(2, 3), (0, 4), (3, 2), (2, 1)],
//     [(0, 2), (2, 5), (1, 3), (2, 0)],
//     [(0, 4), (3, 5), (0, 1), (1, 0)],
//     [(2, 0), (3, 1), (0, 5), (3, 4)],
//     [(1, 3), (0, 5), (0, 2), (0, 0)],
//     [(0, 3), (2, 1), (1, 2), (0, 4)],
// ];

const PART2_FILENAME: &str = "input.txt";
/*
set 0 as leftmost face in top row
.01
.2.
34.
5..
*/
const PART2_CUBE_PATTERN: [(usize, usize); 6] = [(0, 1), (0, 2), (1, 1), (2, 0), (2, 1), (3, 0)];
const PART2_CUBE_PATTERN_MOVES: [[(usize, usize); 4]; 6] = [
    [(0, 1), (0, 2), (2, 3), (1, 5)],
    [(2, 4), (1, 2), (0, 0), (0, 5)],
    [(3, 1), (0, 4), (3, 3), (0, 0)],
    [(0, 4), (0, 5), (2, 0), (1, 2)],
    [(2, 1), (1, 5), (0, 3), (0, 2)],
    [(3, 4), (0, 1), (3, 0), (0, 3)],
];

enum Command {
    Move(usize),
    R,
    L,
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

fn parse_input(input: &String) -> (Vec<Vec<char>>, Vec<Command>) {
    let i: Vec<&str> = input.split("\n\n").collect();
    assert!(i.len() == 2);
    // parse map
    let lines: Vec<&str> = i[0].split("\n").collect();
    let width = lines.iter().map(|line| line.len()).max().unwrap();
    let map: Vec<Vec<char>> = lines
        .iter()
        .map(|line| {
            let mut line_vec = line.chars().collect::<Vec<char>>();
            if line_vec.len() < width {
                line_vec.extend(vec![' '; width - line_vec.len()]);
            }
            line_vec
        })
        .collect();
    // parse path
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)|R|L").unwrap();
    }
    let path: Vec<Command> = RE
        .find_iter(i[1])
        .map(|v| match v.as_str() {
            "R" => Command::R,
            "L" => Command::L,
            num => Command::Move(num.parse::<usize>().unwrap()),
        })
        .collect();
    (map, path)
}

#[allow(dead_code)]
fn disp(map: &Vec<Vec<char>>, path: &Vec<Command>) {
    for m in map.iter() {
        println!("{}", m.iter().collect::<String>());
    }
    let commands: Vec<String> = path
        .iter()
        .map(|c| match c {
            Command::R => String::from("R"),
            Command::L => String::from("L"),
            Command::Move(x) => format!("Move({})", x),
        })
        .collect();
    println!("[{}]", commands.join(", "));
}

fn get_next_pos(map: &Vec<Vec<char>>, pos: (usize, usize), d: usize) -> (usize, usize) {
    let (w, h) = (map[0].len(), map.len());
    match d {
        0 => {
            if pos.1 + 1 == w || map[pos.0][pos.1 + 1] == ' ' {
                (pos.0, map[pos.0].iter().position(|v| *v != ' ').unwrap())
            } else {
                (pos.0, pos.1 + 1)
            }
        }
        1 => {
            if pos.0 + 1 == h || map[pos.0 + 1][pos.1] == ' ' {
                (map.iter().position(|row| row[pos.1] != ' ').unwrap(), pos.1)
            } else {
                (pos.0 + 1, pos.1)
            }
        }
        2 => {
            if pos.1 == 0 || map[pos.0][pos.1 - 1] == ' ' {
                (
                    pos.0,
                    w - map[pos.0].iter().rev().position(|v| *v != ' ').unwrap() - 1,
                )
            } else {
                (pos.0, pos.1 - 1)
            }
        }
        3 => {
            if pos.0 == 0 || map[pos.0 - 1][pos.1] == ' ' {
                (
                    h - map.iter().rev().position(|row| row[pos.1] != ' ').unwrap() - 1,
                    pos.1,
                )
            } else {
                (pos.0 - 1, pos.1)
            }
        }
        _ => panic!(),
    }
}

fn f(map: &Vec<Vec<char>>, path: &Vec<Command>) -> ((usize, usize), usize) {
    let mut pos = (0, map[0].iter().position(|v| *v != ' ').unwrap());
    let mut d = 0;
    for command in path.iter() {
        match command {
            Command::R => {
                d = (d + 1) % 4;
            }
            Command::L => {
                d = (d + 3) % 4;
            }
            Command::Move(x) => {
                for _ in 0..(*x) {
                    let next_pos = get_next_pos(map, pos, d);
                    if map[next_pos.0][next_pos.1] == '.' {
                        pos = next_pos;
                    }
                }
            }
        }
    }
    (pos, d)
}

fn part1() {
    let input = read_input(PART1_FILENAME);
    let (map, path) = parse_input(&input);
    // disp(&map, &path);
    let (pos, d) = f(&map, &path);
    let res = (pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + d;
    println!("{}", res);
}

struct CubePos {
    f_id: usize, // 0~5
    x: usize,
    y: usize,
    d: usize,
}

fn get_cube_element(f: usize, map: &Vec<Vec<char>>, pos: &CubePos) -> char {
    let (f_x, f_y) = PART2_CUBE_PATTERN[pos.f_id];
    map[f_x * f + pos.x][f_y * f + pos.y]
}

fn get_next_pos_face(f: usize, x: usize, y: usize, d: usize) -> ((usize, usize), bool) {
    match d {
        0 => {
            if y + 1 == f {
                ((x, 0), true)
            } else {
                ((x, y + 1), false)
            }
        }
        1 => {
            if x + 1 == f {
                ((0, y), true)
            } else {
                ((x + 1, y), false)
            }
        }
        2 => {
            if y == 0 {
                ((x, f - 1), true)
            } else {
                ((x, y - 1), false)
            }
        }
        3 => {
            if x == 0 {
                ((f - 1, y), true)
            } else {
                ((x - 1, y), false)
            }
        }
        _ => panic!(),
    }
}

fn rot90_cw_face(f: usize, pos: &mut CubePos, k: usize) {
    if k == 0 {
        return;
    }
    // rot90 clock-wise
    (pos.x, pos.y) = (pos.y, f - 1 - pos.x);
    pos.d = (pos.d + 1) % 4;
    rot90_cw_face(f, pos, k - 1);
}

fn move_face(f: usize, pos: &mut CubePos) {
    let (k, next_f_id) = PART2_CUBE_PATTERN_MOVES[pos.f_id][pos.d];
    rot90_cw_face(f, pos, k);
    pos.f_id = next_f_id;
}

fn get_next_pos_cube(f: usize, pos: &CubePos) -> CubePos {
    let ((x, y), face_moved) = get_next_pos_face(f, pos.x, pos.y, pos.d);
    let mut new_pos = CubePos {
        f_id: pos.f_id,
        x: x,
        y: y,
        d: pos.d,
    };
    if face_moved {
        move_face(f, &mut new_pos);
    }
    new_pos
}

fn to_global_coord(f: usize, pos: &CubePos) -> ((usize, usize), usize) {
    let (f_x, f_y) = PART2_CUBE_PATTERN[pos.f_id];
    ((f_x * f + pos.x, f_y * f + pos.y), pos.d)
}

fn g(map: &Vec<Vec<char>>, path: &Vec<Command>) -> ((usize, usize), usize) {
    let (w, h) = (map[0].len(), map.len());
    let (f_w, f_h) = (
        PART2_CUBE_PATTERN.iter().map(|p| p.1).max().unwrap() + 1,
        PART2_CUBE_PATTERN.iter().map(|p| p.0).max().unwrap() + 1,
    );
    assert!(w % f_w == 0 && h % f_h == 0 && w / f_w == h / f_h);
    let f = w / f_w; // size of face
    let mut pos = CubePos {
        f_id: 0,
        x: 0,
        y: 0,
        d: 0,
    };
    for command in path.iter() {
        // println!("{}, ({}, {}) {}", pos.f_id, pos.x, pos.y, pos.d);
        match command {
            Command::R => {
                pos.d = (pos.d + 1) % 4;
            }
            Command::L => {
                pos.d = (pos.d + 3) % 4;
            }
            Command::Move(x) => {
                for _ in 0..(*x) {
                    let next_pos = get_next_pos_cube(f, &pos);
                    if get_cube_element(f, map, &next_pos) == '.' {
                        pos = next_pos;
                    }
                }
            }
        }
    }
    to_global_coord(f, &pos)
}

fn part2() {
    let input = read_input(PART2_FILENAME);
    let (map, path) = parse_input(&input);
    // disp(&map, &path);
    let (pos, d) = g(&map, &path);
    let res = (pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + d;
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
