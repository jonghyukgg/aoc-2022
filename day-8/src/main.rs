use std::fs;

fn read_map(filename: &str) -> Vec<Vec<u32>> {
    fs::read_to_string(filename)
        .expect("Input for the problem")
        .split("\n")
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn part1() {
    let m = read_map("input.txt");
    let (h, w) = (m.len(), m[0].len());
    let mut v = vec![vec![0; w]; h];

    let mut t;
    for r in 0..h {
        t = m[r][0];
        v[r][0] = 1;
        for c in 1..w {
            if m[r][c] > t {
                v[r][c] = 1;
                t = m[r][c];
            }
        }
        t = m[r][w - 1];
        v[r][w - 1] = 1;
        for c in (0..w - 1).rev() {
            if m[r][c] > t {
                v[r][c] = 1;
                t = m[r][c];
            }
        }
    }
    for c in 0..w {
        t = m[0][c];
        v[0][c] = 1;
        for r in 1..h {
            if m[r][c] > t {
                v[r][c] = 1;
                t = m[r][c];
            }
        }
        t = m[h - 1][c];
        v[h - 1][c] = 1;
        for r in (0..h - 1).rev() {
            if m[r][c] > t {
                v[r][c] = 1;
                t = m[r][c];
            }
        }
    }

    let res: u32 = v.iter().map(|row| row.iter().sum::<u32>()).sum::<u32>();
    println!("{}", res);
}

fn part2() {
    let m = read_map("input.txt");
    let (h, w) = (m.len(), m[0].len());

    let mut res = 0;
    for r in 0..h {
        for c in 0..w {
            let (mut ls, mut rs, mut us, mut ds) = (0, 0, 0, 0);
            for ld in 1..=c {
                ls = ld;
                if m[r][c - ld] >= m[r][c] {
                    break;
                }
            }
            for rd in 1..(w - c) {
                rs = rd;
                if m[r][c + rd] >= m[r][c] {
                    break;
                }
            }
            for ud in 1..=r {
                us = ud;
                if m[r - ud][c] >= m[r][c] {
                    break;
                }
            }
            for dd in 1..(h - r) {
                ds = dd;
                if m[r + dd][c] >= m[r][c] {
                    break;
                }
            }
            if ls * rs * us * ds > res {
                res = ls * rs * us * ds;
            }
        }
    }
    println!("{}", res);
}

fn main() {
    part1();
    part2();
}
