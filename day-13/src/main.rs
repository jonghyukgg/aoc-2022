use std::cmp;
use std::cmp::Ordering;
use std::fs;

enum ItemOrList<T> {
    Item(i32),
    List(T),
}

struct NestedList {
    items: Vec<ItemOrList<NestedList>>,
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

fn parse_list_strings(l: &str) -> NestedList {
    let v: Vec<char> = l.chars().collect();
    assert!(v[0] == '[' && v[v.len() - 1] == ']');
    if v.len() == 2 {
        return NestedList { items: vec![] };
    }
    let (mut start_idx, mut nested_count) = (1, 0);
    let mut items: Vec<ItemOrList<NestedList>> = vec![];
    for (idx, c) in v.iter().enumerate() {
        // recursive cases
        if (*c == ']' && nested_count == 1) // last char of line
        || (*c == ',' && nested_count == 1)
        // comma
        {
            if v[start_idx] == '[' {
                items.push(ItemOrList::List(parse_list_strings(&l[start_idx..idx])));
            } else {
                items.push(ItemOrList::Item(l[start_idx..idx].parse::<i32>().unwrap()));
            }
            start_idx = idx + 1;
        }

        if *c == '[' {
            nested_count += 1;
        } else if *c == ']' {
            nested_count -= 1;
            if nested_count == 0 {
                assert!(idx == l.len() - 1);
            }
        }
    }
    NestedList { items: items }
}

fn nested_list_to_string(v: &NestedList) -> String {
    let children: String = v
        .items
        .iter()
        .map(|item| match &item {
            ItemOrList::Item(num) => format!("{}", num),
            ItemOrList::List(vv) => nested_list_to_string(vv),
        })
        .collect::<Vec<String>>()
        .join(",");
    format!("[{}]", children)
}

fn compare(v1: &ItemOrList<NestedList>, v2: &ItemOrList<NestedList>) -> Ordering {
    match (v1, v2) {
        (ItemOrList::Item(i1), ItemOrList::Item(i2)) => {
            if i1 < i2 {
                Ordering::Less
            } else if i1 > i2 {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
        (ItemOrList::List(l1), ItemOrList::Item(_)) => match l1.items.len() {
            0 => Ordering::Less,
            1 => compare(&l1.items[0], v2),
            _ => match compare(&l1.items[0], v2) {
                Ordering::Equal => Ordering::Greater,
                v => v,
            },
        },
        (ItemOrList::List(l1), ItemOrList::List(l2)) => {
            let len = cmp::min(l1.items.len(), l2.items.len());
            for idx in 0..len {
                match compare(&l1.items[idx], &l2.items[idx]) {
                    Ordering::Equal => continue,
                    v => {
                        return v;
                    }
                }
            }
            if l1.items.len() < l2.items.len() {
                return Ordering::Less;
            }
            if l1.items.len() > l2.items.len() {
                return Ordering::Greater;
            }
            return Ordering::Equal;
        }
        (ItemOrList::Item(_), ItemOrList::List(_)) => compare(v2, v1).reverse(),
    }
}

fn compare_list_strings(l1: &str, l2: &str) -> Ordering {
    let (v1, v2) = (parse_list_strings(l1), parse_list_strings(l2));
    compare(&ItemOrList::List(v1), &ItemOrList::List(v2))
}

fn part1() {
    let input = read_input("input.txt");
    let mut res = 0;
    input
        .split("\n\n")
        .enumerate()
        .for_each(|(idx, input_chunk)| {
            let lines: Vec<&str> = input_chunk.split("\n").collect();
            assert!(lines.len() == 2);
            if !matches!(compare_list_strings(lines[0], lines[1]), Ordering::Greater) {
                res += idx + 1;
            }
        });
    println!("{}", res);
}

fn part2() {
    let input = read_input("input.txt");
    let mut packets: Vec<ItemOrList<NestedList>> = vec![];
    input.split("\n\n").for_each(|input_chunk| {
        let lines: Vec<&str> = input_chunk.split("\n").collect();
        assert!(lines.len() == 2);
        packets.push(ItemOrList::List(parse_list_strings(lines[0])));
        packets.push(ItemOrList::List(parse_list_strings(lines[1])));
    });
    packets.push(ItemOrList::List(parse_list_strings("[[2]]")));
    packets.push(ItemOrList::List(parse_list_strings("[[6]]")));
    packets.sort_by(|a, b| compare(&a, &b));
    let mut res = 1usize;
    for (idx, p) in packets.iter().enumerate() {
        match &p {
            ItemOrList::Item(_) => panic!("invalid input"),
            ItemOrList::List(l) => {
                let packet_str = nested_list_to_string(&l);
                // println!("{}", packet_str);
                if packet_str.eq("[[2]]") {
                    res *= idx + 1;
                }
                if packet_str.eq("[[6]]") {
                    res *= idx + 1;
                }
            }
        }
    }
    println!("{}", res);
}

fn main() {
    part1();
    part2();
}
