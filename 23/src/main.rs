use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use uuid::Uuid;
use std::io;
use std::io::Read;

const ROUNDS: usize = 10;
const STARTING_ORDER: [Direction; 4] = [N,S,W,E];
const DEBUG: bool = false;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    let mut grove = Grove::from(buf.as_str());
    let mut order: ProposalOrder = STARTING_ORDER.into();
    if DEBUG {
        println!("{grove}");
    }
    let mut turns = 0;
    loop {
        let turn = Turn{grove: grove.clone(), order, decisions: Decisions::new()};
        let end = turn.first_half();
        let end = end.second_half();
        turns += 1;
        if grove.to_string() == end.grove.to_string() {
            report_turns(turns);
            break;
        }
        if turns == ROUNDS {
            report_empty_ground(&end.grove);
        }
        grove = end.grove;
        order = end.order;
        if DEBUG {
            println!("{grove}");
        }
    }
}

fn report_empty_ground(grove: &Grove) {
    println!("Empty ground tiles after {} turns: {}", ROUNDS, grove.empty_ground());
}

fn report_turns(turns: usize) {
    println!("No further moves after {turns} turns");
}

type Point=(isize, isize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn go(&self, point: Point) -> Point {
        match self {
            N => (point.0, point.1 - 1),
            S => (point.0, point.1 + 1),
            E => (point.0 + 1, point.1),
            W => (point.0 - 1, point.1),
        }
    }

    fn adjacencies(&self, point: Point) -> [Point; 8] {
        [
            N.go(W.go(point)), N.go(point), N.go(E.go(point)),
            W.go(point), E.go(point),
            S.go(W.go(point)), S.go(point), S.go(E.go(point)),
        ]
    }

    fn points_to_check(&self, point: Point) -> [Point; 3] {
        match self {
            N => [N.go(point), N.go(W.go(point)), N.go(E.go(point))],
            S => [S.go(point), S.go(W.go(point)), S.go(E.go(point))],
            E => [E.go(point), N.go(E.go(point)), S.go(E.go(point))],
            W => [W.go(point), N.go(W.go(point)), S.go(W.go(point))],
        }
    }
}

use Direction::*;

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
struct Elf(Uuid);
impl Elf {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

type ProposalOrder=VecDeque<Direction>;

type Decisions=HashMap<Elf, Point>;

type Elves=HashMap<Point, Elf>;

#[derive(Clone)]
struct Grove {
    elves: Elves,
}

impl Grove {
    /// Will the elf propose to go in this direction?
    fn consider(&self, point: Point, dir: Direction) -> bool {
        for p in dir.points_to_check(point) {
            if self.elves.contains_key(&p) {
                return false;
            }
        }
        for p in dir.adjacencies(point) {
            if self.elves.contains_key(&p) {
                return true;
            }
        }
        false
    }

    fn rectangle(&self) -> (Point, Point) {
        let xs = self.elves.keys().map(|p| p.0);
        let (x0, x1) = (xs.clone().min().unwrap(), xs.max().unwrap());
        let ys = self.elves.keys().map(|p| p.1);
        let (y0, y1) = (ys.clone().min().unwrap(), ys.max().unwrap());
        ((x0, y0), (x1, y1))
    }

    fn rectangle_area(&self) -> usize {
        let (p1, p2) = self.rectangle();
        ((p2.0 - p1.0 + 1)*(p2.1 - p1.1 + 1)) as usize
    }

    fn empty_ground(&self) -> usize {
        self.rectangle_area() - self.elves.len()
    }
}

impl From<&str> for Grove {
    fn from(input: &str) -> Self {
        let mut elves = Elves::new();
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    elves.insert((col as isize, row as isize), Elf::new()); 
                }
            }
        }
        Self { elves }
    }
}

impl Display for Grove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        let rect = self.rectangle();
        for y in rect.0.1..=rect.1.1 {
            for x in rect.0.0..=rect.1.0 {
                if self.elves.get(&(x,y)).is_some() {
                    ret.push('#');
                } else {
                    ret.push('.');
                }
            }
            ret.push('\n');
        }
        write!(f, "{ret}")
    }
}

struct Turn {
    grove: Grove,
    order: ProposalOrder,
    decisions: Decisions,
}

impl Turn {
    fn first_half(&self) -> Self {
        let mut decisions = Decisions::new();
        for (point, elf) in self.grove.clone().elves {
            for dir in &self.order {
                if self.grove.consider(point, *dir) {
                    decisions.insert(elf, dir.go(point));
                    break;
                }
            }
        }
        let mut ret = Self{
            grove: self.grove.clone(),
            order: self.order.clone(),
            decisions,
        };
        for (_, elves) in ret.decisions_rev() {
            if elves.len() == 1 {
                continue;
            }
            for elf in elves {
                ret.decisions.remove(&elf);
            }
        }
        ret
    }

    fn second_half(&self) -> Self {
        let mut grove = Grove{elves: Elves::new()};
        for (point, elf) in &self.grove.elves {
            if let Some(next_point) = self.decisions.get(elf) {
                grove.elves.insert(*next_point, *elf);
            } else {
                grove.elves.insert(*point, *elf);
            }
        }
        let mut order = self.order.clone();
        let dir = order.pop_front().unwrap();
        order.push_back(dir);
        Turn { grove, order, decisions: Decisions::new() }
    }

    fn decisions_rev(&self) -> HashMap<Point, Vec<Elf>> {
        let mut ret: HashMap<Point, Vec<Elf>> = HashMap::new();
        for (k, v) in &self.decisions {
            if let Some(e) = ret.get_mut(v) {
                e.push(*k);
            } else {
                ret.insert(*v, vec![*k]);
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn elves_from_string() {
        let input = "..#..\n#.##.\n";
        let grove = Grove::from(input);
        let expected = [
            ((2,0)),
            ((0,1)),
            ((2,1)),
            ((3,1)),
        ].into();
        assert_eq!(grove.elves.keys().cloned().collect::<HashSet<Point>>(), expected);
    }

    #[test]
    fn direction_transforms_point() {
        let dir = N;
        let point = (-1, 4);
        let result = dir.go(point);
        assert_eq!(result, (-1, 3));
    }

    #[test]
    fn find_diagonal_elf() {
        let grove = Grove{elves: Elves::from([
            ((0,0), Elf::new()), ((1,1), Elf::new()),
        ])};
        let point = (1,1);
        let result = grove.consider(point, N);
        assert!(!result);
    }

    #[test]
    fn find_duplicate_proposal() {
        let grove = Grove{elves: Elves::from([
            ((0,0), Elf::new()), ((0,1), Elf::new()), ((0,3), Elf::new()), ((0,4), Elf::new()),
        ])};
        let order = vec![N,S].into();
        let turn = Turn{grove, order, decisions: Decisions::new()};
        let step = turn.first_half();
        assert_eq!(step.decisions.len(), 2);
    }

    #[test]
    fn elves_move() {
        let grove = Grove{elves: Elves::from([
            ((0,0), Elf::new()), ((0,1), Elf::new()), ((0,3), Elf::new()), ((0,4), Elf::new()),
        ])};
        let order = vec![N,S].into();
        let turn = Turn{grove, order, decisions: Decisions::new()};
        let step = turn.first_half().second_half();
        let expected = [
            ((0,-1)),
            ((0,1)),
            ((0,3)),
            ((0,5)),
        ].into();
        assert_eq!(step.grove.elves.keys().cloned().collect::<HashSet<Point>>(), expected);
    }

    #[test]
    fn order_changes() {
        let grove = Grove{elves: Elves::new()};
        let order = vec![N,S].into();
        let turn = Turn{grove, order, decisions: Decisions::new()};
        let step = turn.first_half().second_half();
        let expected = vec![S,N];
        assert_eq!(step.order, expected);
    }

    #[test]
    fn rectangle() {
        let grove = Grove{elves: Elves::from([
            ((0,0), Elf::new()), ((0,1), Elf::new()), ((0,3), Elf::new()), ((0,4), Elf::new()),
        ])};
        let result = grove.rectangle_area();
        assert_eq!(result, 5);
    }

    #[test]
    fn empty_ground() {
        let grove = Grove{elves: Elves::from([
            ((0,0), Elf::new()), ((0,1), Elf::new()), ((0,3), Elf::new()), ((0,4), Elf::new()),
        ])};
        let result = grove.empty_ground();
        assert_eq!(result, 1);
    }

    #[test]
    fn lone_elves_stay_still() {
        let grove = Grove{elves: Elves::from([
            ((0,0), Elf::new()),
        ])};
        let result = grove.consider((0,0), N);
        assert!(!result);
    }
}