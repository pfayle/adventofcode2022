use std::io;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    let mut pairs: Vec<Vec<usize>> = vec![];
    for line in buf.lines() {
        let split = line.split([',', '-']);
        pairs.push(split.map(|c| c.parse::<usize>().unwrap()).collect());
    }
    let contained_pairs = pairs.iter().filter(|v| (
        (v[0] <= v[2] && v[1] >= v[3]) ||
        (v[0] >= v[2] && v[1] <= v[3])
    )).count();
    println!("{} pairs with one range fully containing the other", contained_pairs);
    let overlapping_pairs = pairs.iter().filter(|v|  (
        !(v[1] < v[2] || v[3] < v[0])
    )).count();
    println!("{} pairs with some overlap", overlapping_pairs);
}
