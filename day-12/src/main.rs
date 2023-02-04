use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

fn get_positions(map: &mut Vec<Vec<char>>) -> ((usize, usize), (usize, usize)) {
    let (mut start_pos, mut end_pos): (Option<(usize, usize)>, Option<(usize, usize)>) =
        (None, None);
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] == 'S' {
                start_pos = Some((r, c));
                map[r][c] = 'a';
            } else if map[r][c] == 'E' {
                end_pos = Some((r, c));
                map[r][c] = 'z';
            }
        }
    }
    return (start_pos.unwrap(), end_pos.unwrap());
}

fn get_next_pos(map: &Vec<Vec<char>>, pos: (usize, usize), d: &str) -> Option<(usize, usize)> {
    match d {
        "up" => {
            if pos.0 == 0 {
                None
            } else {
                Some((pos.0 - 1, pos.1))
            }
        }
        "down" => {
            if pos.0 == map.len() - 1 {
                None
            } else {
                Some((pos.0 + 1, pos.1))
            }
        }
        "left" => {
            if pos.1 == 0 {
                None
            } else {
                Some((pos.0, pos.1 - 1))
            }
        }
        "right" => {
            if pos.1 == map[0].len() - 1 {
                None
            } else {
                Some((pos.0, pos.1 + 1))
            }
        }
        _ => panic!("invalid input"),
    }
}

fn part1() {
    let input = read_input("input.txt");
    let mut map: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();
    let (start_pos, end_pos) = get_positions(&mut map);
    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::from([(start_pos, 0usize)]);
    let mut visited: HashSet<(usize, usize)> = HashSet::from([start_pos]);
    while !queue.is_empty() {
        let (pos, distance) = queue.pop_front().unwrap();
        for d in ["up", "down", "left", "right"] {
            let next_pos = get_next_pos(&map, pos, d);
            match next_pos {
                None => continue,
                Some(p) => {
                    if (map[p.0][p.1] as i32) - (map[pos.0][pos.1] as i32) <= 1
                        && !visited.contains(&p)
                    {
                        visited.insert(p);
                        queue.push_back((p, distance + 1));
                        if p == end_pos {
                            println!("{}", distance + 1);
                            return;
                        }
                    }
                }
            }
        }
    }
}

fn part2() {
    let input = read_input("input.txt");
    let mut map: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();
    let (_, end_pos) = get_positions(&mut map);
    // Starting with E, find closest a
    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::from([(end_pos, 0usize)]);
    let mut visited: HashSet<(usize, usize)> = HashSet::from([end_pos]);
    while !queue.is_empty() {
        let (pos, distance) = queue.pop_front().unwrap();
        for d in ["up", "down", "left", "right"] {
            let next_pos = get_next_pos(&map, pos, d);
            match next_pos {
                None => continue,
                Some(p) => {
                    if (map[pos.0][pos.1] as i32) - (map[p.0][p.1] as i32) <= 1
                        && !visited.contains(&p)
                    {
                        visited.insert(p);
                        queue.push_back((p, distance + 1));
                        if map[p.0][p.1] == 'a' {
                            println!("{}", distance + 1);
                            return;
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    part1();
    part2();
}
