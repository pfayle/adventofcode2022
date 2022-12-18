use std::collections::HashMap;
use std::fmt::Display;
use std::{io, time};
use std::io::Read;
use std::thread::sleep;

const RENDER: bool = false;
const SLEEP: u64 = 50;
const WINDOW_HEIGHT: usize = 20;
const TARGET1: usize = 2022;
const TARGET2: usize = 1000000000000;
const ROW_SIZE: usize = 7;
const EMPTY_ROW: Row = Row([false; ROW_SIZE]);
const BLOCKS: [Block; 5] = [
    Block([
        [true; 4],
        [false; 4],
        [false; 4],
        [false; 4],
    ]),
    Block([
        [false, true, false, false],
        [true, true, true, false],
        [false, true, false, false],
        [false; 4],
    ]),
    Block([
        [true, true, true, false],
        [false, false, true, false],
        [false, false, true, false],
        [false; 4],
    ]),
    Block([[true, false, false, false]; 4]),
    Block([
        [true, true, false, false],
        [true, true, false, false],
        [false; 4],
        [false; 4],
    ]),
];

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    let jets = parse_input(&buf);
    let mut iter = jets.iter().enumerate().cycle();
    let mut level = Level::new();
    let mut hashmap: HashMap<(Vec<Row>, usize, usize), (usize, usize)> = HashMap::new();
    let mut height_mapping: HashMap<usize, usize> = HashMap::new();
    let mut period = None;
    while level.block_counter <= TARGET1 {
        if let Some(hashable) = level.tick(&mut iter) {
            if let Some((old_count, old_height)) = hashmap.get(&hashable) {
                if period.is_none() {
                    height_mapping.insert(level.block_counter, level.height());
                    period = Some((level.block_counter - old_count, level.height() - old_height));
                    let offset = (TARGET2+1) % period.unwrap().0;
                    let base_counter = (*old_count..level.block_counter).find(|c| c % period.unwrap().0 == offset).unwrap();
                    let base_height = *height_mapping.get(&base_counter).unwrap();
                    let periods = (TARGET2 + 1 - base_counter) / period.unwrap().0;
                    let total_height = base_height + period.unwrap().1 * periods;
                    println!("Total height: {total_height}");
                }
            } else {
                height_mapping.insert(level.block_counter, level.height());
                hashmap.insert(hashable, (level.block_counter, level.height()));
            }
        }
    }
    println!("Height after {} rocks: {}", TARGET1, level.height());
}

fn parse_input(input: &str) -> Vec<Jet> {
    input.trim().chars().map(
        |c| match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => unreachable!(),
        }
    ).collect()
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct Row([bool; ROW_SIZE]);

impl Row {
    fn contents(&self) -> &[bool; ROW_SIZE] {
        &self.0
    }

    fn contents_mut(&mut self) -> &mut [bool; ROW_SIZE] {
        &mut self.0
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Block([[bool; 4]; 4]);
impl Block {
    fn contents(&self) -> [[bool; 4]; 4] {
        self.0
    }

    fn width(&self) -> usize {
        let mut ret = 0;
        for x in 0..4 {
            for y in 0..4 {
                if ret < x + 1 && self.0[y][x] {
                    ret = x + 1;
                }
            }
        }
        ret
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct BlockX(usize);
#[derive(PartialEq, Eq, Debug, Clone)]
struct BlockY(usize);
#[derive(PartialEq, Eq, Debug, Clone)]
struct PositionedBlock(Block, (BlockX, BlockY));

#[derive(PartialEq, Eq, Debug, Clone)]
struct Level{
    rows: Vec<Row>,
    block_counter: usize,
    block: Option<PositionedBlock>,
}

impl Level {
    fn new() -> Self {
        Self{
            rows: vec![
                EMPTY_ROW,
                EMPTY_ROW,
                EMPTY_ROW,
            ],
            block_counter: 0,
            block: None,
        }
    }

    fn height(&self) -> usize {
        self.rows.iter().cloned().filter(|row| *row != EMPTY_ROW).count()
    }

    fn trim_top_rows(&mut self) {
        for _ in (self.height() + 4)..self.rows.len() {
            self.rows.pop();
        }
    }

    fn next_block(&mut self) {
        self.block = Some(PositionedBlock(
            BLOCKS[self.block_counter % BLOCKS.len()], 
            (BlockX(2), BlockY(self.height() + 3))
        ));
        self.trim_top_rows();
        for _ in 0..4 {
            self.rows.push(EMPTY_ROW);
        }
        self.block_counter += 1;
    }

    fn render(&self) {
        if RENDER {
            sleep(time::Duration::from_millis(SLEEP));
            println!("{self}");
        }
    }

    fn tick(&mut self, jets: &mut (dyn Iterator<Item=(usize, &Jet)>)) -> Option<(Vec<Row>, usize, usize)> {
        if self.block.is_none() {
            self.next_block();
            self.render();
        }
        if let Some((i, dir)) = jets.next() {
            match dir {
                Jet::Right => {
                    if let Some(PositionedBlock(b, (BlockX(x), BlockY(y)))) = self.block {
                        if self.block_can_move_right(&b, x, y) {
                            if let Some(PositionedBlock(_, (BlockX(x), _))) = &mut self.block {
                                *x += 1;
                            }
                        }
                    }
                },
                Jet::Left => {
                    if let Some(PositionedBlock(b, (BlockX(x), BlockY(y)))) = self.block {
                        if self.block_can_move_left(&b, x, y) {
                            if let Some(PositionedBlock(_, (BlockX(x), _))) = &mut self.block {
                                *x -= 1;
                            }
                        }
                    }
                },
            }
            self.render();
            if let Some(PositionedBlock(b, (BlockX(x), BlockY(y)))) = self.block {
                if !self.block_can_move_down(&b, x, y) {
                    self.place_block(b, x, y);
                    self.next_block();
                    self.render();
                    return Some((self.top_component(), i, self.block_counter % BLOCKS.len()));
                } else {
                    if let Some(PositionedBlock(_, (_, BlockY(y)))) = &mut self.block {
                        *y -= 1;
                    }
                    self.render();
                }
            }
        }
        None
    }

    fn place_block(&mut self, b: Block, x: usize, y: usize) {
        for x1 in 0..4 {
            for y1 in 0..4 {
                if b.contents()[y1][x1] {
                    self.rows[y+y1].0[x+x1] = true;
                }
            }
        }
    }

    fn block_can_move_right(&self, b: &Block, x: usize, y: usize) -> bool {
        if x + b.width() >= ROW_SIZE {
            return false;
        }
        for x1 in 0..4 {
            for y1 in 0..4 {
                if b.contents()[y1][x1] && self.rows[y+y1].0[x+x1+1] {
                    return false;
                }
            }
        }
        true
    }

    fn block_can_move_left(&self, b: &Block, x: usize, y: usize) -> bool {
        if x == 0 {
            return false;
        }
        for x1 in 0..4 {
            for y1 in 0..4 {
                if b.contents()[y1][x1] && self.rows[y+y1].0[x+x1-1] {
                    return false;
                }
            }
        }
        true
    }

    fn block_can_move_down(&self, b: &Block, x: usize, y: usize) -> bool {
        if y == 0 {
            return false;
        }
        for x1 in 0..4 {
            for y1 in 0..4 {
                if b.contents()[y1][x1] && self.rows[y+y1 - 1].0[x+x1] {
                    return false;
                }
            }
        }
        true
    }

    fn top_component(&self) -> Vec<Row> {
        let mut ret = vec![];
        let mut previous_connected_row = Row([true; ROW_SIZE]);
        for occupied_row in self.rows.iter().rev() {
            let mut connected_row = EMPTY_ROW;
            // go down from previous row trues
            for (i, head) in previous_connected_row.contents().iter().enumerate() {
                if *head && !occupied_row.contents()[i] {
                    connected_row.contents_mut()[i] = true;
                }
            }
            // stop if there were no changes
            if connected_row == EMPTY_ROW {
                ret.push(connected_row.clone());
                break;
            }
            let mut changes = 1;
            while changes > 0 {
                changes = 0;
                // go left and right from new row trues
                if !connected_row.contents()[0] && !occupied_row.contents()[0] && connected_row.contents()[1] {
                    connected_row.contents_mut()[0] = true;
                    changes += 1;
                }
                if !connected_row.contents()[ROW_SIZE - 1] && !occupied_row.contents()[ROW_SIZE - 1] && connected_row.contents()[ROW_SIZE - 2] {
                    connected_row.contents_mut()[ROW_SIZE - 1] = true;
                    changes += 1;
                }
                for i in 1..(ROW_SIZE - 1) {
                    if !connected_row.contents()[i] && !occupied_row.contents()[i] && (connected_row.contents()[i-1] || connected_row.contents()[i+1]) {
                        connected_row.contents_mut()[i] = true;
                        changes += 1;
                    }
                }
            }
            // repeat if there were any changes
            ret.push(connected_row.clone());
            previous_connected_row = connected_row;
            // stop if there were no changes
            if previous_connected_row == EMPTY_ROW {
                break;
            }
        }
        // (stop if floor)
        ret
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        for (y, row) in self.rows.iter().enumerate().rev().take(WINDOW_HEIGHT) {
            ret.push('|');
            'out: for (x, b) in row.0.iter().enumerate() {
                if let Some(PositionedBlock(Block(b), (BlockX(x1), BlockY(y1)))) = self.block {
                    for (y11, r) in b.iter().enumerate() {
                        for (x11, _) in r.iter().enumerate() {
                            if y1 + y11 == y && x1 + x11 == x && b[y11][x11] {
                                ret.push('@');
                                continue 'out;
                            }
                        }
                    }
                }
                ret.push(match b {
                    true => '#',
                    false => '.',
                });
            }
            ret.push('|');
            ret.push('\n');
        }
        ret.push('+');
        for _ in 0..ROW_SIZE {
            ret.push('-');
        }
        ret.push('+');
        ret.push('\n');
        f.write_str(&ret)
    }
}

#[derive(PartialEq, Eq, Hash)]
enum Jet {
    Right,
    Left,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_level() {
        let level = Level::new();
        let expected = Level{
            rows: vec![
                EMPTY_ROW,
                EMPTY_ROW,
                EMPTY_ROW,
            ],
            block_counter: 0,
            block: None,
        };
        assert_eq!(level, expected);
    }

    #[test]
    fn first_rock_falls_one_doesnt_land() {
        let jets = vec![Jet::Right];
        let mut level = Level::new();
        level.tick(&mut jets.iter().enumerate());
        let expected = Level{
            rows: vec![
                EMPTY_ROW,
                EMPTY_ROW,
                EMPTY_ROW,
                EMPTY_ROW,
                EMPTY_ROW,
                EMPTY_ROW,
                EMPTY_ROW,
            ],
            block_counter: 1,
            block: Some(PositionedBlock(
                BLOCKS[0],
                (BlockX(3), BlockY(2)),
            )),
        };
        assert_eq!(level, expected);
    }

    #[test]
    fn empty_level_height() {
        let level = Level::new();
        assert_eq!(level.height(), 0);
    }

    #[test]
    fn level_height() {
        let mut level = Level::new();
        level.rows[0] = Row([true, false, false, false, false, false, false]);
        level.rows[1] = Row([true, false, false, false, false, false, false]);
        assert_eq!(level.height(), 2);
    }

    #[test]
    fn first_rock_falls_one() {
        let mut jets = [Jet::Right].iter().enumerate().cycle();
        let mut level = Level::new();
        level.tick(&mut jets);
        level.tick(&mut jets);
        let block = level.block.unwrap();
        let position = block.1;
        let y = position.1.0;
        assert_eq!(y, 1);
    }
    
    #[test]
    fn first_rock_falls_to_ground_right() {
        let mut jets = [Jet::Right].iter().enumerate().cycle();
        let mut level = Level::new();
        for _ in 1..=5 {
            level.tick(&mut jets);
        }
        assert_eq!(level.rows[0], Row([false, false, false, true, true, true, true]));
    }

    #[test]
    fn first_rock_falls_to_ground_left() {
        let mut jets = [Jet::Left].iter().enumerate().cycle();
        let mut level = Level::new();
        for _ in 1..=5 {
            level.tick(&mut jets);
        }
        assert_eq!(level.rows[0], Row([true, true, true, true, false, false, false]));
    }

    #[test]
    fn rock_stops_on_nonempty_level_below() {
        let mut jets = [Jet::Left].iter().enumerate().cycle();
        let mut level = Level::new();
        level.rows.push(EMPTY_ROW);
        level.rows[0].0[3] = true;
        level.rows[1].0[3] = true;
        level.rows[2].0[3] = true;
        level.rows[3].0[3] = true;
        for _ in 1..=10 {
            level.tick(&mut jets);
        }
        assert_eq!(level.rows[4], Row([true, true, true, true, false, false, false]));
    }

    #[test]
    fn rock_passes_non_blocking_post_on_left() {
        let mut jets = [Jet::Right, Jet::Right, Jet::Left, Jet::Left, Jet::Left, Jet::Left, Jet::Left, Jet::Left, Jet::Left, Jet::Left, Jet::Left].iter().enumerate();
        let mut level = Level::new();
        level.rows.push(EMPTY_ROW);
        level.rows[0].0[0] = true;
        level.rows[1].0[0] = true;
        level.rows[2].0[0] = true;
        level.rows[3].0[0] = true;
        for _ in 1..=10 {
            level.tick(&mut jets);
        }
        assert_eq!(level.rows[0], Row([true, true, true, true, true, false, false]));
    }

    #[test]
    fn rock_passes_non_blocking_post_on_right() {
        let mut jets = [Jet::Left, Jet::Left, Jet::Left, Jet::Right, Jet::Right, Jet::Right, Jet::Right, Jet::Right, Jet::Right, Jet::Right, Jet::Right].iter().enumerate();
        let mut level = Level::new();
        level.rows.push(EMPTY_ROW);
        level.rows[0].0[ROW_SIZE - 1] = true;
        level.rows[1].0[ROW_SIZE - 1] = true;
        level.rows[2].0[ROW_SIZE - 1] = true;
        level.rows[3].0[ROW_SIZE - 1] = true;
        for _ in 1..=10 {
            level.tick(&mut jets);
        }
        assert_eq!(level.rows[0], Row([false, false, true, true, true, true, true]));
    }

    #[test]
    fn next_block_appears() {
        let mut jets = [Jet::Left].iter().enumerate().cycle();
        let mut level = Level::new();
        for _ in 1..=5 {
            level.tick(&mut jets);
        }
        assert_eq!(level.block_counter, 2);
        assert_eq!(level.block.unwrap().0, BLOCKS[1]);
    }

    #[test]
    fn blocks_loop() {
        let mut jets = [Jet::Left].iter().enumerate().cycle();
        let mut level = Level::new();
        for _ in 1..=52 {
            level.tick(&mut jets);
        }
        assert_eq!(level.block_counter, 13);
        assert_eq!(level.block.unwrap().0, BLOCKS[2]);
    }

    #[test]
    fn basic_top_component() {
        let text = ".......\n.......\n#######\n";
        let mut rows = vec![];
        for line in text.lines().rev() {
            let row = line.chars().map(
                |c| match c {
                    '.' => false,
                    _ => true,
                }
            ).collect::<Vec<bool>>();
            rows.push(Row(row[0..ROW_SIZE].try_into().unwrap()));
        }
        let mut level = Level::new();
        level.rows = rows;
        let result = level.top_component();
        let expected = vec![
            Row([true; ROW_SIZE]),
            Row([true; ROW_SIZE]),
            Row([false; ROW_SIZE]),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn advanced_top_component() {
        let text = ".......\n.#.###.\n##..#..\n.######\n";
        let mut rows = vec![];
        for line in text.lines().rev() {
            let row = line.chars().map(
                |c| match c {
                    '.' => false,
                    _ => true,
                }
            ).collect::<Vec<bool>>();
            rows.push(Row(row[0..ROW_SIZE].try_into().unwrap()));
        }
        let mut level = Level::new();
        level.rows = rows;
        println!("{}", level);
        let result = level.top_component();
        let expected = vec![
            Row([true; ROW_SIZE]),
            Row([true, false, true, false, false, false, true]),
            Row([false, false, true, true, false, true, true]),
            Row([false; ROW_SIZE]),
        ];
        assert_eq!(result, expected);
    }
}
