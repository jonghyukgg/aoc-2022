use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

// const PART1_FILENAME: &str = "sample.txt";
const PART1_FILENAME: &str = "input.txt";

// const PART2_FILENAME: &str = "sample.txt";
const PART2_FILENAME: &str = "input.txt";

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

fn parse_input(input: &String) -> ((i32, i32), Vec<((i32, i32), usize)>) {
    let lines: Vec<&str> = input.split("\n").collect();
    let (w, h) = (lines[0].len(), lines.len());
    let mut blizzards: Vec<((i32, i32), usize)> = vec![];
    lines.iter().enumerate().for_each(|(r, line)| {
        line.chars().enumerate().for_each(|(c, v)| {
            // validation
            if r == 0 || r == h - 1 {
                if (r == 0 && c == 1) || (r == h - 1 && c == w - 2) {
                    assert!(v == '.');
                } else {
                    assert!(v == '#');
                }
            } else {
                if c == 0 || c == w - 1 {
                    assert!(v == '#');
                } else if v != '.' {
                    if c == 1 || c == w - 2 {
                        // this is weird
                        assert!(v != '^' && v != 'v');
                    }
                    match v {
                        '>' => blizzards.push(((r as i32, c as i32), 0)),
                        'v' => blizzards.push(((r as i32, c as i32), 1)),
                        '<' => blizzards.push(((r as i32, c as i32), 2)),
                        '^' => blizzards.push(((r as i32, c as i32), 3)),
                        _ => panic!(),
                    };
                }
            }
        });
    });
    ((w as i32, h as i32), blizzards)
}

#[allow(dead_code)]
fn disp((w, h): (i32, i32), blizzards: &Vec<((i32, i32), usize)>) {
    let mut b_map: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
    for b in blizzards.iter() {
        b_map.entry((b.0 .0, b.0 .1)).or_insert(vec![]).push(b.1);
    }
    for r in 0..h {
        for c in 0..w {
            if r == 0 || r == h - 1 {
                if (r == 0 && c == 1) || (r == h - 1 && c == w - 2) {
                    print!(".");
                } else {
                    print!("#");
                }
            } else {
                if c == 0 || c == w - 1 {
                    print!("#");
                } else {
                    if let Some(d) = b_map.get(&(r as i32, c as i32)) {
                        if d.len() == 1 {
                            match d[0] {
                                0 => print!(">"),
                                1 => print!("v"),
                                2 => print!("<"),
                                3 => print!("^"),
                                _ => panic!(),
                            }
                        } else {
                            print!("{}", d.len());
                        }
                    } else {
                        print!(".");
                    }
                }
            }
        }
        println!();
    }
    println!();
}

fn part1() {
    let input = read_input(PART1_FILENAME);
    let ((w, h), mut blizzards) = parse_input(&input);
    let mut e: HashSet<(i32, i32)> = HashSet::from([(0, 1)]);
    let directions: [(i32, i32); 5] = [(0, 1), (1, 0), (0, -1), (-1, 0), (0, 0)];
    let mut minutes = 0;
    loop {
        minutes += 1;
        // move blizzards
        for b in blizzards.iter_mut() {
            let (dx, dy) = directions[b.1];
            b.0 .0 += dx;
            b.0 .1 += dy;
            if b.0 .0 == 0 {
                b.0 .0 = h - 2;
            } else if b.0 .0 == h - 1 {
                b.0 .0 = 1;
            }
            if b.0 .1 == 0 {
                b.0 .1 = w - 2;
            } else if b.0 .1 == w - 1 {
                b.0 .1 = 1;
            }
        }
        // disp((w, h), &blizzards);

        let mut b_map: HashSet<(i32, i32)> = HashSet::new();
        for b in blizzards.iter() {
            b_map.insert((b.0 .0, b.0 .1));
        }

        // check E
        let mut next_e: HashSet<(i32, i32)> = HashSet::new();
        for (x, y) in e.into_iter() {
            for (dx, dy) in directions.iter() {
                let (nx, ny) = (x + dx, y + dy);
                if nx < 1 || ny < 1 || nx > h - 2 || ny > w - 2 {
                    if !(nx == 0 && ny == 1) && !(nx == h - 1 && ny == w - 2) {
                        continue;
                    }
                }
                if b_map.contains(&(nx, ny)) {
                    continue;
                }
                next_e.insert((nx, ny));
                if nx == h - 1 && ny == w - 2 {
                    println!("{}", minutes);
                    return;
                }
            }
        }
        e = next_e;
    }
}

fn part2() {
    let input = read_input(PART2_FILENAME);
    let ((w, h), mut blizzards) = parse_input(&input);
    let mut e_to_g: HashSet<(i32, i32)> = HashSet::from([(0, 1)]);
    let mut g_to_e: HashSet<(i32, i32)> = HashSet::new();
    let mut again_to_g: HashSet<(i32, i32)> = HashSet::new();
    let directions: [(i32, i32); 5] = [(0, 1), (1, 0), (0, -1), (-1, 0), (0, 0)];
    let mut minutes = 0;
    loop {
        minutes += 1;
        // move blizzards
        for b in blizzards.iter_mut() {
            let (dx, dy) = directions[b.1];
            b.0 .0 += dx;
            b.0 .1 += dy;
            if b.0 .0 == 0 {
                b.0 .0 = h - 2;
            } else if b.0 .0 == h - 1 {
                b.0 .0 = 1;
            }
            if b.0 .1 == 0 {
                b.0 .1 = w - 2;
            } else if b.0 .1 == w - 1 {
                b.0 .1 = 1;
            }
        }
        // disp((w, h), &blizzards);

        let mut b_map: HashSet<(i32, i32)> = HashSet::new();
        for b in blizzards.iter() {
            b_map.insert((b.0 .0, b.0 .1));
        }

        // check E
        let mut next_e_to_g: HashSet<(i32, i32)> = HashSet::new();
        let mut next_g_to_e: HashSet<(i32, i32)> = HashSet::new();
        let mut next_again_to_g: HashSet<(i32, i32)> = HashSet::new();
        for (x, y) in e_to_g.into_iter() {
            for (dx, dy) in directions.iter() {
                let (nx, ny) = (x + dx, y + dy);
                if nx < 1 || ny < 1 || nx > h - 2 || ny > w - 2 {
                    if !(nx == 0 && ny == 1) && !(nx == h - 1 && ny == w - 2) {
                        continue;
                    }
                }
                if b_map.contains(&(nx, ny)) {
                    continue;
                }

                if nx == h - 1 && ny == w - 2 {
                    next_g_to_e.insert((nx, ny));
                } else {
                    next_e_to_g.insert((nx, ny));
                }
            }
        }
        e_to_g = next_e_to_g;
        for (x, y) in g_to_e.into_iter() {
            for (dx, dy) in directions.iter() {
                let (nx, ny) = (x + dx, y + dy);
                if nx < 1 || ny < 1 || nx > h - 2 || ny > w - 2 {
                    if !(nx == 0 && ny == 1) && !(nx == h - 1 && ny == w - 2) {
                        continue;
                    }
                }
                if b_map.contains(&(nx, ny)) {
                    continue;
                }

                if nx == 0 && ny == 1 {
                    next_again_to_g.insert((nx, ny));
                } else {
                    next_g_to_e.insert((nx, ny));
                }
            }
        }
        g_to_e = next_g_to_e;
        for (x, y) in again_to_g.into_iter() {
            for (dx, dy) in directions.iter() {
                let (nx, ny) = (x + dx, y + dy);
                if nx < 1 || ny < 1 || nx > h - 2 || ny > w - 2 {
                    if !(nx == 0 && ny == 1) && !(nx == h - 1 && ny == w - 2) {
                        continue;
                    }
                }
                if b_map.contains(&(nx, ny)) {
                    continue;
                }

                if nx == h - 1 && ny == w - 2 {
                    println!("{}", minutes);
                    return;
                }
                next_again_to_g.insert((nx, ny));
            }
        }
        again_to_g = next_again_to_g;
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
