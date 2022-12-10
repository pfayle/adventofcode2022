use std::io;
use std::io::Read;

fn main() {
    let mut latest = 1;
    let mut reg_history = vec![1];
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    for line in buf.lines() {
        process_instruction(line.to_string(), &mut reg_history, &mut latest);
    }
    let strength_indexes: [usize; 6] = [20, 60, 100, 140, 180, 220];
    let sum: i32 = strength_indexes.iter().map(|n| signal_strength(*n, &reg_history)).sum();
    println!("Sum of signal strengths is {}", sum);
    let mut output = String::new();
    for (n, v) in reg_history.iter().enumerate() {
        let m = (n % 40) as i32;
        if (m-v).abs() < 2 {
            output.push('#');
        } else {
            output.push('.');
        }
        if m == 39 {
            output.push('\n');
        }
    }
    print!("{}", output);
}

fn process_instruction(inst: String, reg_history: &mut Vec<i32>, latest: &mut i32) {
    let mut split = inst.split(" ");
    match split.next() {
        Some("noop") => {
            reg_history.push(latest.clone());
        },
        Some("addx") => {
            if let Some(val) = split.next() {
                reg_history.push(latest.clone());
                *latest += val.parse::<i32>().unwrap();
                reg_history.push(latest.clone());
            }
        },
        _ => unimplemented!(),
    }
}

fn signal_strength(ix: usize, reg_history: &Vec<i32>) -> i32 {
    let ret = (ix as i32) * reg_history[ix - 1];
    ret
}
