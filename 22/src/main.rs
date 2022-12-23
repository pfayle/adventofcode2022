use std::collections::HashMap;
use std::fmt::Display;
use std::io;
use std::io::Read;
use std::ops::{Add,Sub};

const DEBUG: bool = false;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    let (board, instructions) = parse_input(&buf);
    let start = board.start();
    let mut me = start;
    let mut board_visitor = board.clone();
    let mut turn = 1;
        board_visitor.visit(me, turn);
    for inst in instructions.clone() {
        me = board.process_instruction(me, inst);
        if let Instruction::Move(_) = inst {
            turn += 1;
        }
        board_visitor.visit(me, turn);
    }
    if DEBUG {
        println!("{}", board_visitor);
    }
    println!("Flat final password: {}", me.password());
    let cube = CubicBoard{board};
    let mut me = start;
    let mut board_visitor = cube.board.clone();
    let mut turn = 1;
    board_visitor.visit(me, turn);
    for inst in instructions {
        if let Instruction::Move(_) = inst {
            turn += 1;
        }
        for pos in cube.process_instruction(me, inst) {
            board_visitor.visit(pos, turn);
            me = pos;
        }
    }
    if DEBUG {
        println!("{}", board_visitor);
    }
    println!("Cube final password: {}", me.password());
}

fn parse_input(input: &str) -> (Board, Vec<Instruction>) {
    let mut iter = input.lines();
    let mut board_input = String::new();
    while let Some(line) = iter.next() {
        if line.is_empty() {
            let line = iter.next().unwrap();
            let board = Board::from(board_input.as_str());
            let instructions = Instruction::from(line);
            return (board, instructions);
        } else {
            board_input.push_str(&(line.to_string() + "\n"));
        }
    }
    unreachable!();
}

const DIRECTIONS: [Direction; 4] = [Direction::Down, Direction::Left, Direction::Up, Direction::Right];

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

use Direction::*;

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Right => Self::Left,
            Self::Left => Self::Right,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
    fn turn(&self, dir: Self) -> Self {
        match dir {
            Self::Right => match self {
                Self::Up => Self::Right,
                Self::Right => Self::Down,
                Self::Down => Self::Left,
                Self::Left => Self::Up,
            },
            Self::Left => self.turn(Self::Right).opposite(),
            Self::Up => *self,
            Self::Down => self.opposite(),
        }
    }

    /// returns the turn to get from self to next.
    fn to(&self, next: Self) -> Self {
        for dir in DIRECTIONS {
            if self.turn(dir) == next {
                return dir;
            }
        }
        unreachable!()
    }
}

impl From<u8> for Direction {
    fn from(n: u8) -> Self {
        match n % 4 {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => unreachable!()
        }
    }
}

impl From<Direction> for char {
    fn from(dir: Direction) -> Self {
        match dir {
            Up => '^',
            Left => '<',
            Right => '>',
            Down => 'v',
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Location {
    row: usize,
    col: usize,
}

impl Add for Location {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self{
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Sub for Location {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            row: self.row - rhs.row,
            col: self.col - rhs.col,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Me {
    loc: Location,
    dir: Direction,
}

impl Me {
    fn password(&self) -> usize {
        1000 * (self.loc.row + 1)
        + 4 * (self.loc.col + 1)
        + self.dir as usize
    }
}

impl Display for Me {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.loc.row, self.loc.col, <Direction as Into<char>>::into(self.dir))      
    }
}

#[derive(Clone, Copy, Debug)]
enum Tile {
    Open,
    Solid,
    Visited(usize),
}

#[derive(Debug, Clone)]
struct Board {
    tiles: HashMap<Location, Tile>,
}

impl Board {

    fn height(&self) -> usize {
        self.tiles.keys().map(|l| l.row).max().unwrap() + 1
    }

    fn width(&self) -> usize {
        self.tiles.keys().map(|l| l.col).max().unwrap() + 1
    }

    fn get(&self, loc: Location) -> Option<&Tile> {
        self.tiles.get(&loc)
    }

    /// valid for amount <= self.width()
    fn increment(&self, me: Me, amount: usize) -> Location {
        let loc = me.loc;
        match me.dir {
            Direction::Right => {
                Location{row: loc.row, col: (loc.col + amount) % self.width()}
            },
            Direction::Left => {
                Location{row: loc.row, col: (loc.col + self.width() - amount) % self.width()}
            },
            Direction::Up => {
                Location{row: (loc.row + self.height() - amount) % self.height(), col: loc.col}
            },
            Direction::Down => {
                Location{row: (loc.row + amount) % self.height(), col: loc.col}
            },
        }
    }

    fn increment_without_wrapping(&self, me: Me, amount: usize) -> Option<Location> {
        let loc = me.loc;
        let new_loc = match me.dir {
            Direction::Right => {
                if loc.col + amount > self.width() {
                    return None;
                }
                Location{row: loc.row, col: (loc.col + amount)}
            },
            Direction::Left => {
                if amount > loc.col {
                    return None;
                }
                Location{row: loc.row, col: (loc.col - amount)}
            },
            Direction::Up => {
                if amount > loc.row {
                    return None;
                }
                Location{row: (loc.row - amount), col: loc.col}
            },
            Direction::Down => {
                if loc.row + amount > self.height() {
                    return None;
                }
                Location{row: (loc.row + amount), col: loc.col}
            },
        };
        Some(new_loc)
    }

    fn next(&self, me: Me) -> Option<Location> {
        let mut next_loc = me.loc;
        loop {
            next_loc = self.increment(Me{loc: next_loc, dir: me.dir}, 1);
            if next_loc == me.loc {
                return None;
            }
            if let Some(tile) = self.get(next_loc) {
                match tile {
                    Tile::Open|Tile::Visited(_) => { return Some(next_loc); },
                    Tile::Solid => { return None;},
                }
            }
        }
    }

    fn visit(&mut self, me: Me, turn: usize) {
        *self.tiles.get_mut(&me.loc).unwrap() = Tile::Visited(turn);
    }

    fn process_instruction(&self, start: Me, instruction: Instruction) -> Me {
        let mut ret = start;
        match instruction {
            Instruction::Move(n) => {
                for _ in 0..n {
                    if let Some(loc) = self.next(ret) {
                        ret.loc = loc;
                    }
                }
            },
            Instruction::Turn(1) => {
                ret.dir = Direction::from(ret.dir as u8 + 1);
            }
            Instruction::Turn(3) => {
                ret.dir = Direction::from(ret.dir as u8 + 3);
            },
            Instruction::Turn(_) => unimplemented!()
        }
        ret
    }

    fn start(&self) -> Me {
        let mut me = Me{loc: Location { row: 0, col: 0 }, dir: Direction::Right};
        while self.get(me.loc).is_none() {
            me.loc = self.next(me).unwrap();
        }
        me
    }
}

impl From<&str> for Board {
    fn from(input: &str) -> Self {
        let mut tiles = HashMap::new();
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.trim_end().chars().enumerate() {
                let loc = Location{row, col};
                match c {
                    ' ' => {},
                    '.' => {
                        tiles.insert(loc, Tile::Open);
                    },
                    '#' => {
                        tiles.insert(loc, Tile::Solid);
                    },
                    _ => unreachable!(),
                }
            }
        }
        Self { tiles }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        for row in 0..self.height() {
            for col in 0..self.width() {
                match self.get(Location{row, col}) {
                    None => {ret.push(' ');},
                    Some(&Tile::Open) => {ret.push('.');},
                    Some(&Tile::Solid) => {ret.push('#');},
                    Some(&Tile::Visited(dir)) => {ret.push(dir.to_string().chars().last().unwrap());},
                }
            }
            ret.push('\n');
        }
        write!(f, "{}", ret)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Face {
    first: Location,
}

impl Face {
    fn offset(&self, loc: Location) -> Location {
        Location { row: loc.row - self.first.row, col: loc.col - self.first.col }
    }

    fn jump_offset(&self, length: usize, orig: Location, dirs: (Direction, Direction)) -> Location {
        match dirs {
            (Right, Down) => Location{row: 0, col: length - (orig.row + 1)},
            (Right, Left) => Location{row: length - (orig.row + 1), col: length - 1},
            (Right, Right) => Location{row: orig.row, col: 0},
            (Right, Up) => Location{row: length - 1, col: orig.row},
            (Down, Down) => Location{row: 0, col: orig.col},
            (Down, Left) => Location{row: orig.col, col: length - 1},
            (Down, Right) => Location{row: length - (orig.col + 1), col: 0},
            (Down, Up) => Location{row: length - 1, col: length - (orig.col + 1)},
            (Left, Down) => Location{row: 0, col: orig.row},
            (Left, Left) => Location{row: orig.row, col: length - 1},
            (Left, Right) => Location{row: length - (orig.row + 1), col: 0},
            (Left, Up) => Location{row: length - 1, col: length - (orig.row + 1)},
            (Up, Down) => Location{row: 0, col: length - (orig.col + 1)},
            (Up, Left) => Location{row: length - (orig.col + 1), col: length - 1},
            (Up, Right) => Location{row: orig.col, col: 0},
            (Up, Up) => Location{row: length - 1, col: orig.col },
        }
    }
}

struct CubicBoard {
    board: Board,
}

impl CubicBoard {
    fn side_length(&self) -> usize {
        ((self.board.tiles.len() / 6) as f64).sqrt() as usize
    }

    fn faces(&self) -> Vec<Face> {
        let mut ret = vec![];
        for row in (0..self.board.height()).step_by(self.side_length()) {
            for col in (0..self.board.width()).step_by(self.side_length()) {
                let first = Location{row, col};
                if self.board.get(first).is_some() {
                    ret.push(Face { first });
                }
            }
        }
        ret
    }

    fn face(&self, loc: Location) -> Option<Face> {
        let l = self.side_length();
        let first = Location{row: loc.row / l * l, col: loc.col / l * l};
        if self.board.tiles.contains_key(&first) {
            return Some(Face { first });
        }
        None
    }

    fn face_map(&self) -> HashMap<(Face, Direction), (Face, Direction)> {
        let faces = self.faces();
        let mut ret = HashMap::new();
        for face in faces {
            // check all directions: is there a face? If no, skip for now.
            // shouldn't wrap for face connection.
            for dir in DIRECTIONS {
                if let Some(adj_corner) = self.board.increment_without_wrapping(Me{loc: face.first, dir}, self.side_length()) {
                    if let Some(adj_face) = self.face(adj_corner) {
                        ret.insert((face, dir), (adj_face, dir));
                        ret.insert((adj_face, dir.opposite()), (face, dir.opposite()));
                    }
                }
            }
        }
        while ret.len() < 24 {
            for ((f1, d1), (f2, d2)) in ret.clone() {
                // get adj_face's existing adjacencies, if any,
                // call a function to resolve the relationship to face, and store.
                for (_, adj_dir) in ret.clone().keys().filter(|(f, _)| *f == f2) {
                    if let Some((f3, d3)) = ret.clone().get(&(f2, *adj_dir)).cloned() {
                        if f1 == f3 {
                            continue;
                        }
                        let turn = d2.to(*adj_dir);
                        if let Some((init_dir, final_dir)) = self.direct_direction(d1, turn, d3) {
                            ret.insert((f1, init_dir), (f3, final_dir));
                            ret.insert((f3, final_dir.opposite()), (f1, init_dir.opposite()));
                        }
                    }
                }
            }
        }
        ret
    }

    /// convert 2 steps into 1, unless faces are the same or opposite
    fn direct_direction(&self, initially_facing: Direction, turn: Direction, finally_facing: Direction) -> Option<(Direction, Direction)> {
        if turn == Up || turn == Down {
            return None;
        }
        Some((initially_facing.turn(turn), finally_facing.turn(turn.opposite())))
    }

    fn increment(&self, me: Me) -> Me {
        if let Some(loc) = self.board.increment_without_wrapping(me, 1) {
            if self.board.get(loc).is_some() {
                return Me{
                    loc,
                    dir: me.dir,
                };
            }
        }
        let current_face = self.face(me.loc).unwrap();
        let offset = current_face.offset(me.loc);
        let map = self.face_map();
        let jump = map.get(&(current_face, me.dir)).unwrap();
        let new_offset = current_face.jump_offset(self.side_length(), offset, (me.dir, jump.1));
        let loc = Location{
            row: jump.0.first.row + new_offset.row,
            col: jump.0.first.col + new_offset.col,
        };
        Me { loc, dir: jump.1 }
    }

    fn next(&self, me: Me) -> Option<Me> {
        let next_me = self.increment(me);
        match self.board.get(next_me.loc) {
            Some(Tile::Open|Tile::Visited(_)) => { Some(next_me) },
            Some(Tile::Solid) => { None },
            _ => {dbg!(me, next_me); panic!()}
        }
    }

    fn process_instruction(&self, start: Me, instruction: Instruction) -> Vec<Me> {
        let mut ret = vec![start];
        let mut current = start;
        match instruction {
            Instruction::Move(n) => {
                for _ in 0..n {
                    if let Some(loc) = self.next(current) {
                        ret.push(current);
                        current = loc;
                    }
                }
            },
            Instruction::Turn(1) => {
                current.dir = Direction::from(current.dir as u8 + 1);
            }
            Instruction::Turn(3) => {
                current.dir = Direction::from(current.dir as u8 + 3);
            },
            Instruction::Turn(_) => unimplemented!()
        }
        ret.push(current);
        ret
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Move(usize),
    /// turn a number of right angles clockwise
    Turn(usize),
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        match input {
            "R" => Self::Turn(1),
            "L" => Self::Turn(3),
            _ => Self::Move(input.parse::<usize>().unwrap()),
        }
    }
}

impl Instruction {
    fn from(input: &str) -> Vec<Self> {
        let mut ret = vec![];
        let mut num = String::new();
        for c in input.chars() {
            match c {
                'R'|'L' => {
                    if let Ok(n) = num.parse::<usize>() {
                        ret.push(Self::Move(n));
                        num.clear();
                    }
                },
                _ => {
                    num.push(c);
                },
            }
            match c {
                'R' => {
                    ret.push(Self::Turn(1));
                },
                'L' => {
                    ret.push(Self::Turn(3));
                },
                _ => {},
            }
        }
        if let Ok(n) = num.parse::<usize>() {
            ret.push(Self::Move(n));
            num.clear();
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    fn basic_cube() -> CubicBoard {
        let tiles = HashMap::from([
            (Location{row: 0, col: 1}, Tile::Open),
            (Location{row: 1, col: 0}, Tile::Open),
            (Location{row: 1, col: 1}, Tile::Open),
            (Location{row: 1, col: 2}, Tile::Open),
            (Location{row: 1, col: 3}, Tile::Open),
            (Location{row: 2, col: 1}, Tile::Open),
        ]);
        let board = Board{tiles};
        CubicBoard{board}
    }

    fn double_cube() -> CubicBoard {
        let mut tiles = HashMap::new();
        for x in [0, 1] {
            for y in [0, 1] {
                for locs in [(0, 2), (2, 0), (2, 2), (2, 4), (2, 6), (4, 2)] {
                    tiles.insert(Location{row: locs.0 + y, col: locs.1 + x}, Tile::Open);
                }
            }
        }
        let board = Board{tiles};
        CubicBoard{board}
    }

    fn example_cube() -> CubicBoard {
        let tiles = HashMap::from([
            (Location{row: 0, col: 2}, Tile::Open),
            (Location{row: 1, col: 0}, Tile::Open),
            (Location{row: 1, col: 1}, Tile::Open),
            (Location{row: 1, col: 2}, Tile::Open),
            (Location{row: 2, col: 2}, Tile::Open),
            (Location{row: 2, col: 3}, Tile::Open),
        ]);
        let board = Board{tiles};
        CubicBoard{board}
    }

    fn input_cube() -> CubicBoard {
        let tiles = HashMap::from([
            (Location{row: 0, col: 1}, Tile::Open),
            (Location{row: 0, col: 2}, Tile::Open),
            (Location{row: 1, col: 1}, Tile::Open),
            (Location{row: 2, col: 0}, Tile::Open),
            (Location{row: 2, col: 1}, Tile::Open),
            (Location{row: 3, col: 0}, Tile::Open),
        ]);
        let board = Board{tiles};
        CubicBoard{board}
    }

    #[test]
    fn combine_same_directions() {
        let cube = basic_cube();
        let result = cube.direct_direction(Down, Down, Down);
        assert!(result.is_none());
    }

    #[test]
    fn combine_different_directions() {
        let cube = basic_cube();
        let result = cube.direct_direction(Down, Right, Left);
        assert_eq!(result.unwrap(), (Left, Down));
    }

    #[test]
    fn whole_adjacency_map_for_basic_cube() {
        let cube = basic_cube();
        let result = cube.face_map();
        let expected = HashMap::from([
            ((Face{first:Location{row: 0, col: 1}}, Down), (Face{first:Location{row:1, col:1}}, Down)),
            ((Face{first:Location{row: 0, col: 1}}, Up), (Face{first:Location{row:1, col:3}}, Down)),
            ((Face{first:Location{row: 0, col: 1}}, Left), (Face{first:Location{row:1, col:0}}, Down)),
            ((Face{first:Location{row: 0, col: 1}}, Right), (Face{first:Location{row:1, col:2}}, Down)),
            ((Face{first:Location{row: 1, col: 0}}, Down), (Face{first:Location{row:2, col:1}}, Right)),
            ((Face{first:Location{row: 1, col: 0}}, Up), (Face{first:Location{row:0, col:1}}, Right)),
            ((Face{first:Location{row: 1, col: 0}}, Left), (Face{first:Location{row:1, col:3}}, Left)), //
            ((Face{first:Location{row: 1, col: 0}}, Right), (Face{first:Location{row:1, col:1}}, Right)),
            ((Face{first:Location{row: 1, col: 1}}, Down), (Face{first:Location{row:2, col:1}}, Down)),
            ((Face{first:Location{row: 1, col: 1}}, Up), (Face{first:Location{row:0, col:1}}, Up)),
            ((Face{first:Location{row: 1, col: 1}}, Left), (Face{first:Location{row:1, col:0}}, Left)),
            ((Face{first:Location{row: 1, col: 1}}, Right), (Face{first:Location{row:1, col:2}}, Right)),
            ((Face{first:Location{row: 1, col: 2}}, Down), (Face{first:Location{row:2, col:1}}, Left)),
            ((Face{first:Location{row: 1, col: 2}}, Up), (Face{first:Location{row:0, col:1}}, Left)),
            ((Face{first:Location{row: 1, col: 2}}, Left), (Face{first:Location{row:1, col:1}}, Left)),
            ((Face{first:Location{row: 1, col: 2}}, Right), (Face{first:Location{row:1, col:3}}, Right)),
            ((Face{first:Location{row: 1, col: 3}}, Down), (Face{first:Location{row:2, col:1}}, Up)),
            ((Face{first:Location{row: 1, col: 3}}, Up), (Face{first:Location{row:0, col:1}}, Down)),
            ((Face{first:Location{row: 1, col: 3}}, Left), (Face{first:Location{row:1, col:2}}, Left)),
            ((Face{first:Location{row: 1, col: 3}}, Right), (Face{first:Location{row:1, col:0}}, Right)),
            ((Face{first:Location{row: 2, col: 1}}, Down), (Face{first:Location{row:1, col:3}}, Up)),
            ((Face{first:Location{row: 2, col: 1}}, Up), (Face{first:Location{row:1, col:1}}, Up)),
            ((Face{first:Location{row: 2, col: 1}}, Left), (Face{first:Location{row:1, col:0}}, Up)),
            ((Face{first:Location{row: 2, col: 1}}, Right), (Face{first:Location{row:1, col:2}}, Up)),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn find_cube_side() {
        let cube = double_cube();
        let result = cube.side_length();
        assert_eq!(result, 2);
    }


    #[test]
    fn find_cube_face() {
        let cube = double_cube();
        let loc = Location { row: 0, col: 3 };
        let result = cube.face(loc).unwrap();
        assert_eq!(result, Face{first: Location { row: 0, col: 2 }});
    }

    #[test]
    fn cube_increments_with_offset() {
        let cube = double_cube();
        let me = Me{loc: Location { row: 0, col: 3 }, dir: Up};
        let result = cube.increment(me);
        assert_eq!(result, Me{
            loc: Location { row: 2, col: 6 },
            dir: Down,
        });
    }

    #[test]
    fn cube_blocks_with_wall() {
        let target = Location { row: 2, col: 6 };
        let mut cube = double_cube();
        *cube.board.tiles.get_mut(&target).unwrap() = Tile::Solid;
        let me = Me{loc: Location { row: 0, col: 3 }, dir: Up};
        let result = cube.next(me);
        assert_eq!(result, None);
    }

    #[test]
    fn cube_increments_away_from_boundary() {
        let cube = double_cube();
        let me = Me{loc: Location { row: 3, col: 5 }, dir: Down};
        let result = cube.increment(me);
        assert_eq!(result, Me{
            loc: Location { row: 5, col: 3 },
            dir: Left,
        });
    }

    #[test]
    fn adjacency_map_for_example_cube() {
        let cube = example_cube();
        let result = cube.face_map();
        assert_eq!(result.keys().len(), 24);
    }

    #[test]
    fn adjacency_map_for_input_cube() {
        let cube = input_cube();
        let result = cube.face_map();
        assert_eq!(result.keys().len(), 24);
    }

    #[test]
    fn double_cube_jumps() {
        let length = 2;
        let face = Face{first: Location{row:0, col:0}};
        let result = face.jump_offset(length, Location{row:0, col:0}, (Up, Down));
        assert_eq!(result, Location{row: 0, col: 1});
        let result = face.jump_offset(length, Location{row:0, col:0}, (Left, Down));
        assert_eq!(result, Location{row: 0, col: 0});
        let result = face.jump_offset(length, Location{row:1, col:0}, (Left, Down));
        assert_eq!(result, Location{row: 0, col: 1});
        let result = face.jump_offset(length, Location{row:0, col:1}, (Up, Down));
        assert_eq!(result, Location{row: 0, col: 0});
        let result = face.jump_offset(length, Location{row:0, col:1}, (Right, Down));
        assert_eq!(result, Location{row: 0, col: 1});
        let result = face.jump_offset(length, Location{row:1, col:1}, (Right, Down));
        assert_eq!(result, Location{row: 0, col: 0});
        let face = Face{first: Location{row:1, col:0}};
        let result = face.jump_offset(length, Location{row:0, col:0}, (Up, Right));
        assert_eq!(result, Location{row: 0, col: 0});
        let result = face.jump_offset(length, Location{row:0, col:1}, (Up, Right));
        assert_eq!(result, Location{row: 1, col: 0});
        let result = face.jump_offset(length, Location{row:1, col:0}, (Down, Right));
        assert_eq!(result, Location{row: 1, col: 0});
        let result = face.jump_offset(length, Location{row:1, col:1}, (Down, Right));
        assert_eq!(result, Location{row: 0, col: 0});
        let face = Face{first: Location{row:1, col:2}};
        let result = face.jump_offset(length, Location{row:0, col:0}, (Up, Left));
        assert_eq!(result, Location{row: 1, col: 1});
        let result = face.jump_offset(length, Location{row:0, col:1}, (Up, Left));
        assert_eq!(result, Location{row: 0, col: 1});
        let result = face.jump_offset(length, Location{row:1, col:0}, (Down, Left));
        assert_eq!(result, Location{row: 0, col: 1});
        let result = face.jump_offset(length, Location{row:1, col:1}, (Down, Left));
        assert_eq!(result, Location{row: 1, col: 1});
        let face = Face{first: Location{row:1, col:3}};
        let result = face.jump_offset(length, Location{row:0, col:0}, (Up, Down));
        assert_eq!(result, Location{row: 0, col: 1});
        let result = face.jump_offset(length, Location{row:0, col:1}, (Up, Down));
        assert_eq!(result, Location{row: 0, col: 0});
        let result = face.jump_offset(length, Location{row:0, col:1}, (Right, Right));
        assert_eq!(result, Location{row: 0, col: 0});
        let result = face.jump_offset(length, Location{row:1, col:0}, (Down, Up));
        assert_eq!(result, Location{row: 1, col: 1});
        let result = face.jump_offset(length, Location{row:1, col:1}, (Down, Up));
        assert_eq!(result, Location{row: 1, col: 0});
        let result = face.jump_offset(length, Location{row:1, col:1}, (Right, Right));
        assert_eq!(result, Location{row: 1, col: 0});
        let face = Face{first: Location{row:2, col:1}};
        let result = face.jump_offset(length, Location{row:0, col:0}, (Left, Up));
        assert_eq!(result, Location{row: 1, col: 1});
        let result = face.jump_offset(length, Location{row:1, col:0}, (Left, Up));
        assert_eq!(result, Location{row: 1, col: 0});
        let result = face.jump_offset(length, Location{row:1, col:0}, (Down, Up));
        assert_eq!(result, Location{row: 1, col: 1});
        let result = face.jump_offset(length, Location{row:0, col:1}, (Right, Up));
        assert_eq!(result, Location{row: 1, col: 0});
        let result = face.jump_offset(length, Location{row:1, col:1}, (Right, Up));
        assert_eq!(result, Location{row: 1, col: 1});
        let result = face.jump_offset(length, Location{row:1, col:1}, (Down, Up));
        assert_eq!(result, Location{row: 1, col: 0});
    }

    #[test]
    fn double_input_cube_jumps() {
        let length = 2;
        let face = Face{first: Location{row:0, col:3}};
        let result = face.jump_offset(length, Location{row:0, col:0}, (Up, Right));
        assert_eq!(result, Location{row: 0, col: 0});
        let result = face.jump_offset(length, Location{row:0, col:0}, (Left, Right));
        assert_eq!(result, Location{row: 1, col: 0});
        let result = face.jump_offset(length, Location{row:1, col:0}, (Left, Right));
        assert_eq!(result, Location{row: 0, col: 0});
        let result = face.jump_offset(length, Location{row:0, col:1}, (Up, Right));
        assert_eq!(result, Location{row: 1, col: 0});
        let face = Face{first: Location{row:0, col:5}};
        let result = face.jump_offset(length, Location{row:0, col:0}, (Up, Up));
        assert_eq!(result, Location{row: 1, col: 0}); //
        let result = face.jump_offset(length, Location{row:1, col:0}, (Down, Left));
        assert_eq!(result, Location{row: 0, col: 1});
        let result = face.jump_offset(length, Location{row:0, col:1}, (Up, Up));
        assert_eq!(result, Location{row: 1, col: 1}); //
        let result = face.jump_offset(length, Location{row:0, col:1}, (Right, Left));
        assert_eq!(result, Location{row: 1, col: 1});
        let result = face.jump_offset(length, Location{row:1, col:1}, (Right, Left));
        assert_eq!(result, Location{row: 0, col: 1});
        let result = face.jump_offset(length, Location{row:1, col:1}, (Down, Left));
        assert_eq!(result, Location{row: 1, col: 1});
        let face = Face{first: Location{row:3, col:3}};
        let result = face.jump_offset(length, Location{row:0, col:0}, (Left, Down));
        assert_eq!(result, Location{row: 0, col: 0});
        let result = face.jump_offset(length, Location{row:0, col:1}, (Right, Up));
        assert_eq!(result, Location{row: 1, col: 0});
        let result = face.jump_offset(length, Location{row:1, col:0}, (Left, Down));
        assert_eq!(result, Location{row: 0, col: 1});
        let result = face.jump_offset(length, Location{row:1, col:1}, (Right, Up));
        assert_eq!(result, Location{row: 1, col: 1});
        let face = Face{first: Location{row:5, col:3}};
        let result = face.jump_offset(length, Location{row:0, col:1}, (Right, Left));
        assert_eq!(result, Location{row: 1, col: 1});
        let result = face.jump_offset(length, Location{row:1, col:0}, (Down, Left));
        assert_eq!(result, Location{row: 0, col: 1});
        let result = face.jump_offset(length, Location{row:1, col:1}, (Right, Left));
        assert_eq!(result, Location{row: 0, col: 1});
        let result = face.jump_offset(length, Location{row:1, col:1}, (Down, Left));
        assert_eq!(result, Location{row: 1, col: 1});
        let face = Face{first: Location{row:5, col:1}};
        let result = face.jump_offset(length, Location{row:0, col:0}, (Left, Right));
        assert_eq!(result, Location{row: 1, col: 0});
        let result = face.jump_offset(length, Location{row:0, col:0}, (Up, Right));
        assert_eq!(result, Location{row: 0, col: 0});
        let result = face.jump_offset(length, Location{row:0, col:1}, (Up, Right));
        assert_eq!(result, Location{row: 1, col: 0});
        let result = face.jump_offset(length, Location{row:1, col:0}, (Left, Right));
        assert_eq!(result, Location{row: 0, col: 0});
        let face = Face{first: Location{row:7, col:1}};
        let result = face.jump_offset(length, Location{row:0, col:0}, (Left, Down));
        assert_eq!(result, Location{row: 0, col: 0});
        let result = face.jump_offset(length, Location{row:0, col:1}, (Right, Up));
        assert_eq!(result, Location{row: 1, col: 0});
        let result = face.jump_offset(length, Location{row:1, col:0}, (Left, Down));
        assert_eq!(result, Location{row: 0, col: 1});
        let result = face.jump_offset(length, Location{row:1, col:0}, (Down, Down));
        assert_eq!(result, Location{row: 0, col: 0}); //
        let result = face.jump_offset(length, Location{row:1, col:1}, (Down, Down));
        assert_eq!(result, Location{row: 0, col: 1}); //
        let result = face.jump_offset(length, Location{row:1, col:1}, (Right, Up));
        assert_eq!(result, Location{row: 1, col: 1});
    }

    #[test]
    fn whole_adjacency_map_for_input_cube() {
        let cube = input_cube();
        //  01
        //  2
        // 45
        // 3
        let faces = vec![
            Face{first:Location { row: 0, col: 1 }},
            Face{first:Location { row: 0, col: 2 }},
            Face{first:Location { row: 1, col: 1 }},
            Face{first:Location { row: 3, col: 0 }},
            Face{first:Location { row: 2, col: 0 }},
            Face{first:Location { row: 2, col: 1 }},
        ];
        let result = cube.face_map();
        let expected = HashMap::from([
            ((faces[0], Down), (faces[2], Down)),
            ((faces[0], Up), (faces[3], Right)),
            ((faces[0], Left), (faces[4], Right)),
            ((faces[0], Right), (faces[1], Right)),
            ((faces[1], Down), (faces[2], Left)),
            ((faces[1], Up), (faces[3], Up)),
            ((faces[1], Left), (faces[0], Left)),
            ((faces[1], Right), (faces[5], Left)),
            ((faces[2], Down), (faces[5], Down)),
            ((faces[2], Up), (faces[0], Up)),
            ((faces[2], Left), (faces[4], Down)),
            ((faces[2], Right), (faces[1], Up)),
            ((faces[3], Down), (faces[1], Down)),
            ((faces[3], Up), (faces[4], Up)),
            ((faces[3], Left), (faces[0], Down)),
            ((faces[3], Right), (faces[5], Up)),
            ((faces[4], Down), (faces[3], Down)),
            ((faces[4], Up), (faces[2], Right)),
            ((faces[4], Left), (faces[0], Right)),
            ((faces[4], Right), (faces[5], Right)),
            ((faces[5], Down), (faces[3], Left)),
            ((faces[5], Up), (faces[2], Up)),
            ((faces[5], Left), (faces[4], Left)),
            ((faces[5], Right), (faces[1], Left)),
        ]);
        assert_eq!(result, expected);
    }
}
