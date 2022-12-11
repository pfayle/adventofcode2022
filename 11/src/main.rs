use std::io;
use std::io::Read;

struct Monkey<'a> {
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize + 'a>,
    divisor: usize,
    true_monkey: usize,
    false_monkey: usize,
    inspections: usize,
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    let mut monkeys = vec![];
    let mut divisors = vec![];
    for line in buf.lines() {
        process_line(line, &mut monkeys, &mut divisors);
    }
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            monkey_turn(i, &mut monkeys, &divisors, true);
        }
    }
    println!("monkey business: {}", monkey_business(&monkeys));
    let mut monkeys = vec![];
    let mut divisors = vec![];
    for line in buf.lines() {
        process_line(line, &mut monkeys, &mut divisors);
    }
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            monkey_turn(i, &mut monkeys, &divisors, false);
        }
    }
    println!("monkey business: {}", monkey_business(&monkeys));
}

fn process_line<'a>(line: &'a str, monkeys: &mut Vec<Monkey<'a>>, divisors: &mut Vec<usize>) {
    if line.contains("Monkey") {
        monkeys.push(Monkey{
            items: vec![],
            operation: Box::new(|_| 1),
            divisor: 0,
            true_monkey: 0,
            false_monkey: 0,
            inspections: 0,
        });
    }
    if line.contains("Starting items:") {
        let split = line.split([':', ',']).skip(1);
        monkeys.last_mut().unwrap().items = split.map(|x| x.trim().parse::<usize>().unwrap()).collect();
    }
    if line.contains("Operation:") {
        let mut split = line.split(" ").skip(6);
        let op = split.next().unwrap();
        let b = split.next().unwrap();
        monkeys.last_mut().unwrap().operation = match (op, b) {
            ("+", "old") => {Box::new(|x| x+x)},
            ("*", "old") => {Box::new(|x| x*x)},
            ("+", _) => {Box::new(|x| x + b.parse::<usize>().unwrap())},
            ("*", _) => {Box::new(|x| x * b.parse::<usize>().unwrap())},
            _ => unimplemented!(),
        };
    }
    if line.contains("Test:") {
        let divisor = line.split("divisible by ").last().unwrap().parse::<usize>().unwrap();
        monkeys.last_mut().unwrap().divisor = divisor;
        divisors.push(divisor);
    }
    if line.contains("If true:") {
        monkeys.last_mut().unwrap().true_monkey = line.split("throw to monkey ").last().unwrap().parse::<usize>().unwrap();
    }
    if line.contains("If false:") {
        monkeys.last_mut().unwrap().false_monkey = line.split("throw to monkey ").last().unwrap().parse::<usize>().unwrap();
    }
}

fn monkey_turn(i: usize, monkeys: &mut Vec<Monkey>, divisors: &Vec<usize>, capped_worry: bool) {
    for item in monkeys[i].items.clone() {
        let mut worry = (monkeys[i].operation)(item);
        if capped_worry {
            worry = (worry as f32 / 3_f32).floor() as usize;
        }
        worry = worry % divisors.iter().product::<usize>();
        if worry % monkeys[i].divisor == 0 {
            let m = monkeys[i].true_monkey;
            monkeys[m].items.push(worry);
        } else {
            let m = monkeys[i].false_monkey;
            monkeys[m].items.push(worry);
        }
        monkeys[i].inspections += 1;
    }
    monkeys[i].items = vec![];
}

fn monkey_business(monkeys: &Vec<Monkey>) -> usize {
    let mut insps: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();
    insps.sort_unstable();
    insps.reverse();
    insps[0] * insps[1]
}