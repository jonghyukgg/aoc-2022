use std::{borrow::BorrowMut, collections::VecDeque, fs};

enum Operation {
    Add(i64),
    Mul(i64),
    Sq,
}

struct Monkey {
    items: VecDeque<i64>,
    op: Operation,
    div: i64,
    true_idx: usize,
    false_idx: usize,
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

fn parse_input(input: &String) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    input
        .split("\n\n")
        .enumerate()
        .for_each(|(idx, monkey_input)| {
            let lines: Vec<&str> = monkey_input.split("\n").collect();
            assert!(lines.len() == 6);

            // First line
            assert!(format!("Monkey {idx}:").eq(lines[0]));

            // Starting items
            let prefix = "  Starting items: ";
            assert!(lines[1].starts_with(prefix));
            let items_str = lines[1].chars().skip(prefix.len()).collect::<String>();
            let items = items_str
                .split(", ")
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<VecDeque<i64>>();

            // Operation
            let prefix = "  Operation: new = old ";
            assert!(lines[2].starts_with(prefix));
            let operator = lines[2].chars().nth(prefix.len()).unwrap();
            let right_operand = lines[2].chars().skip(prefix.len() + 2).collect::<String>();
            let op = match operator {
                '+' => Operation::Add(right_operand.parse::<i64>().unwrap()),
                '*' => match right_operand.as_ref() {
                    "old" => Operation::Sq,
                    v => Operation::Mul(v.parse::<i64>().unwrap()),
                },
                _ => panic!("invalid input"),
            };

            // Test
            let prefix = "  Test: divisible by ";
            assert!(lines[3].starts_with(prefix));
            let div = lines[3]
                .chars()
                .skip(prefix.len())
                .collect::<String>()
                .parse::<i64>()
                .unwrap();

            // if true
            let prefix = "    If true: throw to monkey ";
            assert!(lines[4]
                .chars()
                .take(prefix.len())
                .collect::<String>()
                .eq(prefix));
            let true_idx = lines[4]
                .chars()
                .skip(prefix.len())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            // if false
            let prefix = "    If false: throw to monkey ";
            assert!(lines[5]
                .chars()
                .take(prefix.len())
                .collect::<String>()
                .eq(prefix));
            let false_idx = lines[5]
                .chars()
                .skip(prefix.len())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            assert!(idx != true_idx);
            assert!(idx != false_idx);

            monkeys.push(Monkey {
                items,
                op,
                div,
                true_idx,
                false_idx,
            })
        });
    monkeys
}

fn part1() {
    let input = read_input("input.txt");
    let mut monkeys = parse_input(&input);
    let mut monkeys_inspection_count = vec![0; monkeys.len()];
    let num_rounds = 20;
    for _ in 1..=num_rounds {
        for idx in 0..monkeys.len() {
            while !monkeys[idx].items.is_empty() {
                let mut item = monkeys[idx].borrow_mut().items.pop_front().unwrap();
                match monkeys[idx].op {
                    Operation::Add(v) => item += v,
                    Operation::Mul(v) => item *= v,
                    Operation::Sq => item *= item,
                };
                item /= 3;
                let throw_idx = if item % monkeys[idx].div == 0 {
                    monkeys[idx].true_idx
                } else {
                    monkeys[idx].false_idx
                };
                monkeys[throw_idx].items.push_back(item);
                monkeys_inspection_count[idx] += 1;
            }
        }
    }
    monkeys_inspection_count.sort_by_key(|k| -k);
    println!(
        "{}",
        monkeys_inspection_count[0] * monkeys_inspection_count[1]
    );
}

fn part2() {
    let input = read_input("input.txt");
    let mut monkeys = parse_input(&input);
    let mut monkeys_inspection_count = vec![0i64; monkeys.len()];
    let num_rounds = 10000;

    let mut modular = 1i64;
    monkeys.iter().for_each(|m| {
        modular *= m.div;
    });
    for _ in 1..=num_rounds {
        for idx in 0..monkeys.len() {
            while !monkeys[idx].items.is_empty() {
                let mut item = monkeys[idx].borrow_mut().items.pop_front().unwrap();
                match monkeys[idx].op {
                    Operation::Add(v) => item += v,
                    Operation::Mul(v) => item *= v,
                    Operation::Sq => item *= item,
                };
                item = item % modular;
                let throw_idx = if item % monkeys[idx].div == 0 {
                    monkeys[idx].true_idx
                } else {
                    monkeys[idx].false_idx
                };
                monkeys[throw_idx].items.push_back(item);
                monkeys_inspection_count[idx] += 1;
            }
        }
    }
    monkeys_inspection_count.sort_by_key(|k| -k);
    println!(
        "{}",
        monkeys_inspection_count[0] * monkeys_inspection_count[1]
    );
}

fn main() {
    part1();
    part2();
}
