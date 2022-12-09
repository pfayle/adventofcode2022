use std::io;
use std::io::Read;
use std::collections::HashSet;
// break down the instruction list (R 4 -> R R R R)
// track the head through all the instructions
// work out how the tail moves for each head movement
// track the tail through all the instructions
// number of unique locations the tail's been through

// head position (x,y)
// tail position (x,y)
// tail locations [(x1,y1), (x2,y2),...]
fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    let mut inst_list = vec![];
    for line in buf.lines() {
        // R 4
        let mut res = line.split(" ");
        let inst = res.next().unwrap().chars().next().unwrap(); // R
        let count = res.next().unwrap().parse::<usize>().unwrap(); // "4"
        for _ in 0..count {
            inst_list.push(inst.clone()); // ['R', 'R', 'R', 'R']
        }
    }
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut multitail = vec![];
    for _ in 0..9 {
        multitail.push((0, 0));
    }
    let mut tail_locations = HashSet::new();
    let mut multitail_locations = HashSet::new();
    tail_locations.insert(tail.clone());
    for inst in inst_list {
        process_instruction(&mut head, inst);
        process_tail_catch_up(&mut tail, &head);
        process_multitail(&mut multitail, &head);
        tail_locations.insert(tail.clone());
        multitail_locations.insert(multitail[8].clone());
    }
    println!("The second knot has been at {} locations", tail_locations.len());
    println!("The final knot has been {} locations", multitail_locations.len());
}

fn process_instruction(head: &mut (i32, i32), inst: char) {
    if inst == 'R' {
        head.0 += 1;
    }
    if inst == 'U' {
        head.1 += 1;
    }
    if inst == 'L' {
        head.0 -= 1;
    }
    if inst == 'D' {
        head.1 -= 1;
    }
}

fn process_tail_catch_up(tail: &mut (i32, i32), head: &(i32, i32)) {
    if ((head.0 - tail.0).abs() > 1 && (head.1 - tail.1).abs() > 0) ||
        ((head.0 - tail.0).abs() > 0 && (head.1 - tail.1).abs() > 1)
    {
        // diagonal
        tail.0 += (head.0 - tail.0)/(head.0 - tail.0).abs();
        tail.1 += (head.1 - tail.1)/(head.1 - tail.1).abs();
    } else {
        // horizontal
        if head.0 - tail.0 > 1 {
            tail.0 += 1;
        }
        if tail.0 - head.0 > 1 {
            tail.0 -= 1;
        }
        if head.1 - tail.1 > 1 {
            tail.1 += 1;
        }
        if tail.1 - head.1 > 1 {
            tail.1 -= 1;
        }
    }
}

fn process_multitail(tail: &mut Vec<(i32, i32)>, head: &(i32, i32)) {
    let mut prev = head.clone();
    for segment in tail {
        process_tail_catch_up(segment, &prev);
        prev = *segment;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_instruction_increments_by_1() {
        let instruction = 'R';
        let mut head = (4, 7);
        process_instruction(&mut head, instruction);
        assert_eq!(head.0, 5);
    }

    #[test]
    fn other_instructions_behave_as_expected() {
        let mut head = (4, 7);
        process_instruction(&mut head, 'D');
        assert_eq!(head.1, 6);
        process_instruction(&mut head, 'L');
        assert_eq!(head.0, 3);
        process_instruction(&mut head, 'U');
        assert_eq!(head.1, 7);
    }

    // .T.H -> ..TH
    #[test]
    fn tail_catches_up_horizontally() {
        let head = (3, 0);
        let mut tail = (1, 0);
        process_tail_catch_up(&mut tail, &head);
        assert_eq!(tail, (2, 0));
    }

    #[test]
    fn tail_catches_up_horizontally2() {
        let head = (-3, 0);
        let mut tail = (-1, 0);
        process_tail_catch_up(&mut tail, &head);
        assert_eq!(tail, (-2, 0));
    }

    // ..H -> .TH
    // T.. -> ...
    #[test]
    fn tail_catches_up_diagonally() {
        let head = (1, 2);
        let mut tail = (0, 0);
        process_tail_catch_up(&mut tail, &head);
        assert_eq!(tail, (1, 1));
    }

    #[test]
    fn tail_catches_up_diagonally2() {
        let head = (-1, -2);
        let mut tail = (0, 0);
        process_tail_catch_up(&mut tail, &head);
        assert_eq!(tail, (-1, -1));
    }

    #[test]
    fn tail_doesnt_move() {
        let head = (1, 1);
        let mut tail = (0, 0);
        process_tail_catch_up(&mut tail, &head);
        assert_eq!(tail, (0, 0));
    }

    #[test]
    fn multitail_moves() {
        let head = (3, 2);
        let mut tails = vec![(1, 1), (0, 0)];
        process_multitail(&mut tails, &head);
        assert_eq!(head, (3, 2));
        assert_eq!(tails[0], (2, 2));
        assert_eq!(tails[1], (1, 1));
    }
}