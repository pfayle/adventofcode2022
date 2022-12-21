use std::io;
use std::io::Read;
use evalexpr::eval;

const DEBUG: bool = false;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    process_input(&buf);
}

// Quick (to write) and dirty, ignoring any possibility of nice
// graph structures.
fn process_input(input: &str) {
    let mut found = true;
    let mut data = input.to_string();
    while found {
        found = false;
        for line in data.clone().lines() {
            let [name, value]: [&str; 2] = line.split(':').collect::<Vec<&str>>().try_into().unwrap();
            if value.chars().any(|c| c.is_alphabetic()) {
                found = true;
                continue;
            }
            let v = eval(value).unwrap();
            if name.chars().any(|c| c.is_alphabetic()) {
                if name == "root" {
                    println!("root = {}", v);
                    found = false;
                    break;
                }
            }
            data = data.replace(name, v.to_string().as_str());
        }
    }
    let mut found = true;
    let mut data = input.to_string();
    while found {
        found = false;
        for line in data.clone().lines() {
            let [name, value]: [&str; 2] = line.split(':').collect::<Vec<&str>>().try_into().unwrap();
            let vars = value.trim().split(' ').filter(|s| s.len() > 0).collect::<Vec<&str>>();
            if name == "root" {
                if !value.chars().any(|c| c.is_alphabetic()) {
                    continue;
                }
                if vars[0].parse::<usize>().is_ok() {
                    data = data.replace(vars[2], vars[0]);
                } else if vars[2].parse::<usize>().is_ok() {
                    data = data.replace(vars[0], vars[2]);
                }
            }
            if name == "humn" {
                continue;
            }
            if vars.len() < 3 {
                data = data.replace(name, value.to_string().as_str());
                continue;
            }
            match (name.parse::<usize>(), vars[0].parse::<usize>(), vars[2].parse::<usize>()) {
                (Err(_), Ok(_), Ok(_)) => {
                    found = true;
                    let v = eval(value).unwrap();
                    data = data.replace(name, v.to_string().as_str());
                    if name == "humn" || DEBUG {
                        println!("{} = {}", name, v);
                    }
                },
                (Ok(v1), Ok(v2), Err(_)) => {
                    found = true;
                    let val = match vars[1].chars().next().unwrap() {
                        '+' => v1 - v2,
                        '-' => v2 - v1,
                        '*' => v1 / v2,
                        '/' => v2 / v1,
                        _ => unimplemented!()
                    };
                    data = data.replace(vars[2], val.to_string().as_str());
                    if vars[2] == "humn" || DEBUG {
                        println!("{} = {}", vars[2], val);
                    }
                },
                (Ok(v1), Err(_), Ok(v2)) => {
                    found = true;
                    let val = match vars[1].chars().next().unwrap() {
                        '+' => v1 - v2,
                        '-' => v1 + v2,
                        '*' => v1 / v2,
                        '/' => v1 * v2,
                        _ => unimplemented!()
                    };
                    data = data.replace(vars[0], val.to_string().as_str());
                    if vars[0] == "humn" || DEBUG {
                        println!("{} = {}", vars[0], val);
                    }
                },
                (Ok(_), Ok(_), Ok(_)) => {
                    continue;
                },
                _ => {
                    found = true;
                    continue;
                }
            }
        }
    }
}
