use std::io;
use std::io::Read;
use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    let mut iter = buf.lines();
    let mut stacks = vec![];
    while let Some(line) = iter.next() {
        if line.is_empty() {
            break;
        }
        if stacks.is_empty() {
            for _ in 0..(line.len() / 4 + 1) {
                stacks.push(VecDeque::new());
            }
        }
        if line.contains("1") {
            continue;
        }
        for (n, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c == ' ' {
                continue;
            }
            stacks[n].push_front(c);
        }
    }

    let mut stacks9001 = stacks.clone();

    while let Some(line) = iter.next() {
        let mut split = line.split(" ");
        let count = split.nth(1).unwrap().parse::<usize>().unwrap();
        let from = split.nth(1).unwrap().parse::<usize>().unwrap();
        let to = split.nth(1).unwrap().parse::<usize>().unwrap();
        mov(&mut stacks, from, to, count);
        mov9001(&mut stacks9001, from, to, count);
    }

    let msg = stacks.iter().map(|stk| stk.back().unwrap()).collect::<String>();
    let msg9001 = stacks9001.iter().map(|stk| stk.back().unwrap()).collect::<String>();
    println!("{}", msg);
    println!("{}", msg9001);
}

fn mov(stacks: &mut Vec<VecDeque<char>>, from: usize, to: usize, count: usize) {
    for _ in 0..count {
        let crt = stacks[from - 1].pop_back().unwrap();
        stacks[to - 1].push_back(crt);
    }
}

fn mov9001(stacks: &mut Vec<VecDeque<char>>, from: usize, to: usize, count: usize) {
    let mut crts = vec![];
    for _ in 0..count {
        crts.push(stacks[from - 1].pop_back().unwrap());
    }
    crts.reverse();
    for crt in crts {
        stacks[to - 1].push_back(crt);
    }
}
