use std::collections::HashMap;
use std::fs;

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

fn find_start<const N: usize>(input: &String) -> usize {
    let mut counter: HashMap<char, usize> = HashMap::new();
    let chars: Vec<char> = input.chars().collect();
    for (idx, c) in chars.iter().enumerate() {
        let mut count;
        count = counter.entry(c.clone()).or_insert(0);
        *count += 1;
        if idx >= N {
            let remove_char = &chars[idx - N];
            count = counter.get_mut(remove_char).unwrap();
            *count -= 1;
            if *count == 0 {
                counter.remove(remove_char);
            }
        }
        if idx >= N - 1 {
            if counter.keys().len() == N {
                return idx + 1;
            }
        }
    }
    panic!("Invalid input")
}

fn part1() {
    let input = read_input("input.txt");
    let res = find_start::<4>(&input);
    println!("{}", res);
}

fn part2() {
    let input = read_input("input.txt");
    let res = find_start::<14>(&input);
    println!("{}", res);
}

fn main() {
    part1();
    part2();
}
