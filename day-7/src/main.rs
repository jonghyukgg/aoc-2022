use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

struct Node {
    size: Option<usize>, // None is dir
    children: HashMap<String, Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(size: Option<usize>) -> Self {
        Node {
            size: size,
            children: HashMap::new(),
        }
    }

    pub fn add_child(&mut self, key: &str, new_node: Rc<RefCell<Node>>) {
        self.children.insert(String::from(key), new_node);
    }
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

fn traverse_node_print(node: Rc<RefCell<Node>>, name: &str, prefix: &str) {
    let node_b = node.borrow();
    println!(
        "{}- {} ({})",
        prefix,
        name,
        match node_b.size {
            Some(x) => format!("file, size={x}"),
            None => String::from("dir"),
        }
    );
    let next_prefix = String::from(prefix) + "  ";
    for (k, v) in node_b.children.iter() {
        traverse_node_print(Rc::clone(v), k, &next_prefix);
    }
}

#[allow(dead_code)]
fn traverse_print(root: Rc<RefCell<Node>>) {
    traverse_node_print(root, "/", "");
}

fn process_input(input: &str) -> Rc<RefCell<Node>> {
    let root = Rc::new(RefCell::new(Node::new(None)));
    let mut d_stack = vec![Rc::clone(&root)];
    for c in input.split("$ ") {
        let lines: Vec<&str> = c.split("\n").filter(|line| !line.is_empty()).collect();
        if lines.len() == 0 {
            continue;
        }
        let command: Vec<&str> = lines[0].split(" ").collect();
        match command[0] {
            "cd" => {
                match command[1] {
                    "/" => {
                        d_stack = vec![Rc::clone(&root)];
                    }
                    ".." => {
                        d_stack.pop();
                    }
                    d => {
                        let cwd = Rc::clone(d_stack.last().unwrap());
                        d_stack.push(Rc::clone(cwd.borrow().children.get(d).unwrap()));
                    }
                };
            }
            "ls" => {
                for l in lines.iter().skip(1) {
                    let line: Vec<&str> = l.split(" ").collect();
                    let size: Option<usize> = if line[0].eq("dir") {
                        None
                    } else {
                        Some(line[0].parse::<usize>().unwrap())
                    };
                    let child = Rc::new(RefCell::new(Node::new(size)));
                    let cwd = Rc::clone(d_stack.last_mut().unwrap());
                    cwd.borrow_mut().add_child(line[1], child);
                }
            }
            _ => panic!("Invalid Input"),
        };
    }
    // traverse_print(Rc::clone(&root));
    root
}

fn traverse_part1<const N: usize>(node: Rc<RefCell<Node>>) -> (usize, usize) {
    let node_b = node.borrow_mut();
    let mut size = node_b.size.unwrap_or(0);
    let mut res = 0;
    node_b.children.values().for_each(|c| {
        let (sub_size, sub_res) = traverse_part1::<N>(Rc::clone(c));
        size += sub_size;
        res += sub_res;
    });
    if size <= N && node_b.size.is_none() {
        res += size;
    }
    (size, res)
}

fn process_part1(root: Rc<RefCell<Node>>) -> usize {
    let (_, res) = traverse_part1::<100000>(root);
    return res;
}

fn traverse_part2_get_total(node: Rc<RefCell<Node>>) -> usize {
    let node_b = node.borrow_mut();
    let mut size = node_b.size.unwrap_or(0);
    node_b.children.values().for_each(|c| {
        size += traverse_part2_get_total(Rc::clone(c));
    });
    size
}

fn traverse_part2_min_gte(node: Rc<RefCell<Node>>, lb: usize) -> (usize, Option<usize>) {
    let node_b = node.borrow_mut();
    let mut size = node_b.size.unwrap_or(0);
    let mut res = None;
    node_b.children.values().for_each(|c| {
        let (sub_size, sub_res) = traverse_part2_min_gte(Rc::clone(c), lb);
        size += sub_size;
        if !sub_res.is_none() {
            if res.is_none() || res > sub_res {
                res = sub_res;
            }
        }
    });
    if size >= lb && node_b.size.is_none() {
        if res.is_none() || res > Some(size) {
            res = Some(size);
        }
    }
    (size, res)
}

fn process_part2(root: Rc<RefCell<Node>>) -> usize {
    const AVAIL: usize = 70000000;
    const NEED: usize = 30000000;
    let total = traverse_part2_get_total(Rc::clone(&root));
    let (_, res) = traverse_part2_min_gte(Rc::clone(&root), total + NEED - AVAIL);
    return res.unwrap();
}

fn part1() {
    let input = read_input("input.txt");
    let root = process_input(&input);
    let res = process_part1(Rc::clone(&root));
    println!("{}", res);
}

fn part2() {
    let input = read_input("input.txt");
    let root = process_input(&input);
    let res = process_part2(Rc::clone(&root));
    println!("{}", res);
}

fn main() {
    part1();
    part2();
}
