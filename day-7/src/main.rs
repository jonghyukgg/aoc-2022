use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

struct Node {
    name: String,
    size: Option<usize>,
    children: HashMap<String, Rc<RefCell<Node>>>, // any way to share name & key for children?
    parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(name: String, size: Option<usize>, parent: Option<Rc<RefCell<Node>>>) -> Self {
        Node {
            name: name,
            size: size,
            children: HashMap::new(),
            parent: parent,
        }
    }
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Input for the problem")
}

fn traverse_node_print(node: Rc<RefCell<Node>>, prefix: String) {
    let node_b = node.borrow_mut();
    println!("{}- {} ({})", prefix, node_b.name, node_b.size.unwrap_or(0));
    for v in node_b.children.values() {
        traverse_node_print(Rc::clone(v), prefix.clone() + "  ");
    }
}

#[allow(dead_code)]
fn traverse_print(root: Rc<RefCell<Node>>) {
    traverse_node_print(root, String::from(""));
}

fn process_input(input: &str) -> Rc<RefCell<Node>> {
    let root = Rc::new(RefCell::new(Node::new(String::from("/"), None, None)));
    let mut cwd = Rc::clone(&root);
    for c in input.split("$ ") {
        let lines: Vec<&str> = c.split("\n").filter(|line| !line.is_empty()).collect();
        if lines.len() == 0 {
            continue;
        }
        let command: Vec<&str> = lines[0].split(" ").collect();
        match command[0] {
            "cd" => {
                let cwd_clone = Rc::clone(&cwd);
                match command[1] {
                    // Not still fully understanding this...
                    "/" => {
                        cwd = Rc::clone(&root);
                    }
                    ".." => {
                        cwd = Rc::clone(cwd_clone.borrow_mut().parent.as_ref().unwrap());
                    }
                    d => {
                        cwd = Rc::clone(cwd_clone.borrow_mut().children.get_mut(d).unwrap());
                    }
                };
            }
            "ls" => {
                for l in lines.iter().skip(1) {
                    let line: Vec<&str> = l.split(" ").collect();
                    let name = String::from(line[1]);
                    let size: Option<usize> = if line[0].eq("dir") {
                        None
                    } else {
                        Some(line[0].parse::<usize>().unwrap().clone())
                    };
                    cwd.borrow_mut().children.insert(
                        name.clone(),
                        Rc::new(RefCell::new(Node::new(name, size, Some(Rc::clone(&cwd))))),
                    );
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

fn part1() {
    let input = read_input("input.txt");
    let root = process_input(&input);
    let res = process_part1(Rc::clone(&root));
    println!("{}", res);
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
