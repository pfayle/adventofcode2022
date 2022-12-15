use std::cmp::{min,max};
use std::io;
use std::io::Read;

const START: (usize, usize) = (500, 0);

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    let points = parse_input(buf);
    let mut space = space(&points);
    let mut sand_count = 0;
    while !sand_falls_to_floor(START, &mut space) {
        sand_count += 1;
    }
    println!("{sand_count} units of sand fell");
    sand_count += 1;
    while !sand_filled(START, &mut space) {
        sand_count += 1;
    }
    sand_count += 1;
    println!("{sand_count} units of sand fell");
}

fn parse_input(input: String) -> Vec<(usize, usize)> {
    let mut ret = vec![];
    for line in input.lines() {
        let split = line.split(" -> ");
        for window in split.collect::<Vec<&str>>()[..].windows(2) {
            let point1 = window[0].split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let point2 = window[1].split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            for m in min(point1[0], point2[0])..=max(point1[0], point2[0]) {
                for n in min(point1[1], point2[1])..=max(point1[1], point2[1]) {
                    ret.push((m, n));
                }
            }
            ret.sort_unstable();
            ret.dedup();
        }
    }
    ret
}

fn sand_falls_to_floor(start: (usize, usize), space: &mut Vec<Vec<bool>>) -> bool {
    let mut sand_position = start;
    while move_sand_down(&mut sand_position, space) {
        if sand_position.1 + 1 >= space.len() {
            break;
        }
    }
    space[sand_position.1][sand_position.0] = true;
    sand_position.1 >= space.len() - 3
}

fn sand_filled(start: (usize, usize), space: &mut [Vec<bool>]) -> bool {
    let mut sand_position = start;
    while move_sand_down(&mut sand_position, space) {
    }
    space[sand_position.1][sand_position.0] = true;
    sand_position == START
}

fn move_sand_down(position: &mut (usize, usize), space: &[Vec<bool>]) -> bool {
    if !space[position.1 + 1][position.0] {
        *position = (position.0, position.1 + 1);
        return true;
    }
    if !space[position.1 + 1][position.0 - 1] {
        *position = (position.0 - 1, position.1 + 1);
        return true;
    }
    if !space[position.1 + 1][position.0 + 1] {
        *position = (position.0 + 1, position.1 + 1);
        return true;
    }
    false
}

fn space(points: &[(usize, usize)]) -> Vec<Vec<bool>> {
    let mut pts = points.to_owned();
    pts.push((500, 0));
    let depth = pts.iter().map(|p| p.1).max().unwrap();
    let mut ret = vec![];
    for _ in 0..=depth+1 {
        ret.push(vec![false; 1001]);
    }
    ret.push(vec![true; 1001]);
    for pt in pts {
        ret[pt.1][pt.0] = true;
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_one_line() {
        let input = "498,4 -> 498,6".to_string();
        let result = parse_input(input);
        assert_eq!(result, vec![(498, 4), (498, 5), (498, 6)]);
    }

    #[test]
    fn parse_one_line_two_segments() {
        let input = "498,4 -> 498,6 -> 496,6".to_string();
        let result = parse_input(input);
        assert_eq!(result, vec![(496, 6), (497, 6), (498, 4), (498, 5), (498, 6)]);
    }

    #[test]
    fn parse_two_lines() {
        let input = "498,4 -> 498,6\n400,3 -> 400,4".to_string();
        let result = parse_input(input);
        assert_eq!(result, vec![(400, 3), (400, 4), (498, 4), (498, 5), (498, 6)]);
    }

    // .+.
    // .o.
    // ###
    #[test]
    fn sand_hits_bottom_of_box() {
        let mut space = vec![vec![false, false, false], vec![false, false, false], vec![true, true, true], vec![false, false, false], vec![true, true, true]];
        sand_falls_to_floor((1, 0),&mut space);
        assert!(space[1][1]);
    }

    #[test]
    fn move_sand_down_one() {
        let space = vec![vec![false, false, false], vec![false, false, false], vec![true, true, true]];
        let mut position = (1, 0);
        let result = move_sand_down(&mut position, &space);
        assert!(result);
        assert_eq!(position, (1, 1));
    }

    // .+.
    // o#.
    // ###
    #[test]
    fn move_sand_down_and_left() {
        let space = vec![vec![false, false, false], vec![false, true, false], vec![true, true, true]];
        let mut position = (1, 0);
        let result = move_sand_down(&mut position, &space);
        assert!(result);
        assert_eq!(position, (0, 1));
    }

    // .+.
    // ##o
    // ###
    #[test]
    fn move_sand_down_and_right() {
        let space = vec![vec![false, false, false], vec![true, true, false], vec![true, true, true]];
        let mut position = (1, 0);
        let result = move_sand_down(&mut position, &space);
        assert!(result);
        assert_eq!(position, (2, 1));
    }

    // .+.
    // ...
    // ...
    // ...
    // .o.
    // ###
    #[test]
    fn sand_hits_bottom_of_deeper_box() {
        let mut space = vec![vec![false, false, false], vec![false, false, false], vec![false, false, false], vec![false, false, false], vec![false, false, false], vec![true, true, true], vec![false, false, false], vec![true, true, true]];
        let result = sand_falls_to_floor((1, 0), &mut space);
        assert!(space[4][1]);
        assert!(!result);
    }

    // .+.
    // ...
    // ...
    #[test]
    fn sand_falls_through_bottom() {
        let mut space = vec![vec![false, false, false], vec![false, false, false], vec![false, false, false]];
        let result = sand_falls_to_floor((1, 0), &mut space);
        assert!(result);
    }

    #[test]
    fn bound_space() {
        let points = vec![(0, 1), (20, 30), (600, 25)];
        let space = space(&points);
        assert_eq!(space.len(), 33);
        assert_eq!(space[0].len(), 1001);
        assert!(space[1][0]);
        assert!(!space[2][0]);
    }

    // .+.
    // .#.
    // o..
    // ###
    #[test]
    fn sand_to_floor() {
        let mut space = vec![vec![false, true, false], vec![false, false, false], vec![true, true, true]];
        let result = sand_falls_to_floor((1, 0), &mut space);
        assert!(result);
        assert!(space[2][0]);
    }
}
