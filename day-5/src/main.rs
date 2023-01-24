use std::fs;

fn parse_input(filename: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let input: String = fs::read_to_string(filename).expect("Input for the problem");
    let splited_input: Vec<&str> = input.split("\n\n").collect();

    // parse stacks
    let stacks_input: Vec<&str> = splited_input[0].rsplit("\n").collect();
    let mut stacks: Vec<Vec<char>> = vec![];
    stacks_input[0]
        .split(" ")
        .filter(|s| !s.is_empty())
        .for_each(|_| {
            stacks.push(vec![]);
        });
    stacks_input.iter().skip(1).for_each(|line| {
        for (idx, c) in line.chars().enumerate() {
            if (idx + 3) % 4 == 0 && c != ' ' {
                stacks[(idx - 1) / 4].push(c);
            }
        }
    });

    // parse commands
    let commands: Vec<(usize, usize, usize)> = splited_input[1]
        .split("\n")
        .map(|line| {
            let words: Vec<&str> = line.split(" ").collect();
            (
                words[1].parse::<usize>().unwrap().clone(),
                // convert to zero-index
                words[3].parse::<usize>().unwrap().clone() - 1,
                words[5].parse::<usize>().unwrap().clone() - 1,
            )
        })
        .collect();

    (stacks, commands)
}

fn part1() {
    let (mut stacks, commands) = parse_input("input1.txt");
    // for (idx, s) in stacks.iter().enumerate() {
    //     print!("{}: ", idx + 1);
    //     s.iter().for_each(|c| {
    //         print!("{} ", c);
    //     });
    //     println!();
    // }
    // for c in commands.iter() {
    //     println!("{} {} {}", c.0, c.1, c.2);
    // }

    for c in commands.iter() {
        for _ in 0..c.0 {
            let cr = stacks[c.1].pop().unwrap();
            stacks[c.2].push(cr);
        }
    }
    // for (idx, s) in stacks.iter().enumerate() {
    //     print!("{}: ", idx + 1);
    //     s.iter().for_each(|c| {
    //         print!("{} ", c);
    //     });
    //     println!();
    // }

    let res = stacks
        .iter()
        .map(|s| s.last().unwrap().clone())
        .collect::<String>();
    println!("{}", res);
}

fn part2() {
    let (mut stacks, commands) = parse_input("input2.txt");

    for c in commands.iter() {
        let mut temp_stack: Vec<char> = vec![];
        for _ in 0..c.0 {
            let cr = stacks[c.1].pop().unwrap();
            temp_stack.push(cr);
        }
        for temp_c in temp_stack.into_iter().rev() {
            stacks[c.2].push(temp_c);
        }
    }
    // for (idx, s) in stacks.iter().enumerate() {
    //     print!("{}: ", idx + 1);
    //     s.iter().for_each(|c| {
    //         print!("{} ", c);
    //     });
    //     println!();
    // }

    let res = stacks
        .iter()
        .map(|s| s.last().unwrap().clone())
        .collect::<String>();
    println!("{}", res);
}

fn main() {
    part1();
    part2();
}
