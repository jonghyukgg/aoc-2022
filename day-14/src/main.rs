use std::fs;

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

fn parse_input(input: &String) -> Vec<Vec<(i32, i32)>> {
    input
        .split("\n")
        .map(|line| {
            line.split(" -> ")
                .map(|coord| {
                    let values: Vec<i32> = coord
                        .split(",")
                        .map(|v| v.parse::<i32>().unwrap())
                        .collect();
                    assert!(values.len() == 2);
                    (values[0], values[1])
                })
                .collect()
        })
        .collect()
}

fn get_boundary(paths: &Vec<Vec<(i32, i32)>>) -> (i32, i32, i32) {
    let (mut minx, mut maxx, mut maxy) = (500, 500, 0); // miny = 0
    for path in paths.iter() {
        for coord in path.iter() {
            if coord.0 < minx {
                minx = coord.0;
            }
            if coord.0 > maxx {
                maxx = coord.0;
            }
            if coord.1 > maxy {
                maxy = coord.1;
            }
            assert!(coord.1 > 0);
        }
    }
    (minx, maxx, maxy)
}

fn draw_map(paths: &Vec<Vec<(i32, i32)>>) -> (usize, Vec<Vec<char>>) {
    let (minx, maxx, maxy) = get_boundary(paths);

    // x=minx-1 -> x=0
    // if sand reaches maxy (= map.len() - 1) it's over
    let mut map = vec![vec!['.'; (maxx - minx + 3) as usize]; (maxy + 1) as usize];
    for path in paths.iter() {
        for idx in 0..path.len() - 1 {
            let (x1, y1) = (path[idx].0 - (minx - 1), path[idx].1);
            let (x2, y2) = (path[idx + 1].0 - (minx - 1), path[idx + 1].1);
            if x1 == x2 {
                let (y_high, y_low) = if y1 > y2 { (y2, y1) } else { (y1, y2) };
                for y in y_high..=y_low {
                    map[y as usize][x1 as usize] = '#';
                }
            } else if y1 == y2 {
                let (x_left, x_right) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
                for x in x_left..=x_right {
                    map[y1 as usize][x as usize] = '#';
                }
            } else {
                panic!("invalid input");
            }
        }
    }
    ((500 - (minx - 1)) as usize, map)
}

#[allow(dead_code)]
fn display_input(start_pos: usize, map: &Vec<Vec<char>>) {
    let first_line = map[0].iter().collect::<String>();
    println!(
        "{}{}{}",
        &first_line[..start_pos],
        if map[0][start_pos] == '.' {
            '+'
        } else {
            map[0][start_pos]
        },
        &first_line[start_pos + 1..]
    );
    map.iter()
        .skip(1)
        .for_each(|line| println!("{}", line.iter().collect::<String>()));
}

fn pour(start_pos: usize, map: &mut Vec<Vec<char>>) -> bool {
    let mut p = (start_pos, 0usize);
    if map[p.1][p.0] != '.' {
        return false;
    }
    while p.1 < map.len() - 1 {
        if map[p.1 + 1][p.0] == '.' {
            p.1 += 1;
        } else if map[p.1 + 1][p.0 - 1] == '.' {
            p.1 += 1;
            p.0 -= 1;
        } else if map[p.1 + 1][p.0 + 1] == '.' {
            p.1 += 1;
            p.0 += 1;
        } else {
            break;
        }
    }
    if p.1 == map.len() - 1 {
        return false;
    }
    map[p.1][p.0] = 'o';
    return true;
}

fn part1() {
    let input = read_input("input.txt");
    let paths = parse_input(&input);
    let (start_pos, mut map) = draw_map(&paths);
    // display_input(start_pos, &map);
    let mut rounds = 0;
    while pour(start_pos, &mut map) {
        rounds += 1;
    }
    // display_input(start_pos, &map);
    println!("{}", rounds);
}

fn get_floor(paths: &Vec<Vec<(i32, i32)>>) -> Vec<(i32, i32)> {
    let (_, _, maxy) = get_boundary(paths);
    // (-inf, maxy+2) ~ (+inf, maxy+2)
    // but, cannot go further than 500 +- (maxy+2), imagine a full pyramid
    vec![(500 - (maxy + 2), maxy + 2), (500 + (maxy + 2), maxy + 2)]
}

fn part2() {
    let input = read_input("input.txt");
    let mut paths = parse_input(&input);
    paths.push(get_floor(&paths));
    let (start_pos, mut map) = draw_map(&paths);
    // display_input(start_pos, &map);
    let mut rounds = 0;
    while pour(start_pos, &mut map) {
        rounds += 1;
    }
    // display_input(start_pos, &map);
    println!("{}", rounds);
}

fn main() {
    part1();
    part2();
}
