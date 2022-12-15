use std::cmp::min;
use std::io;
use std::io::Read;
use std::ops::RangeInclusive;

const BRUTE_FORCE_THRESHOLD: isize = 1000;
const WINDOW_SIZE: isize = 400;

struct Sensor {
    pos: (isize, isize),
    nearest: (isize, isize),
}

impl Sensor {
    fn scan_distance(&self) -> isize {
        self.manhattan_distance(self.nearest)
    }

    fn manhattan_distance(&self, pos: (isize, isize)) -> isize {
        (self.pos.0 - pos.0).abs()
        + (self.pos.1 - pos.1).abs()
    }

    fn scan_contains(&self, pos: (isize, isize)) -> bool {
        self.manhattan_distance(pos) <= self.scan_distance()
    }

    fn hidden_beacon_impossible_positions_in_row(&self, y: isize) -> Vec<isize> {
        let mut ret = vec![];
        let row_dist = (y - self.pos.1).abs();
        let remaining_dist = self.scan_distance() - row_dist;
        for x in (self.pos.0 - remaining_dist)..=(self.pos.0 + remaining_dist) {
            if (x, y) != self.nearest {
                ret.push(x);
            }
        }
        ret
    }

    /// check whether each corner is within the Manhattan distance
    fn box_excluded(&self, top_corner: &(isize, isize), width: isize, height: isize) -> bool {
        [(0, 0), (0, height - 1), (width - 1, 0), (width - 1, height - 1)].iter().map(|add| (top_corner.0 + add.0, top_corner.1 + add.1))
            .all(|p| self.scan_contains(p))
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    let sensors = parse_input(&buf);
    println!("{} positions in row {} where a beacon cannot be present", count_row(&sensors, 10), 10);
    println!("{} positions in row {} where a beacon cannot be present", count_row(&sensors, 2000000), 2000000);

    if let Some(res) = find_hidden_beacon(0..=20, &sensors) {
        println!("Hidden beacon at ({}, {}) with tuning frequency {}", res.0, res.1, tuning_frequency(res));
    }

    if let Some(res) = find_hidden_beacon(0..=4000000, &sensors) {
        println!("Hidden beacon at ({}, {}) with tuning frequency {}", res.0, res.1, tuning_frequency(res));
    }
}

fn parse_input(input: &str) -> Vec<Sensor> {
    let mut ret = vec![];
    for line in input.lines() {
        let split = line.replace(['x', '=', 'y', ',', ':'], "").split(' ')
            .map(|s| s.parse::<isize>())
            .filter(|s| s.is_ok())
            .map(|s| s.unwrap())
            .collect::<Vec<isize>>();
        ret.push(Sensor {
            pos: (split[0], split[1]),
            nearest: (split[2], split[3]),
        });
    }
    ret
}

fn count_row(sensors: &Vec<Sensor>, row: isize) -> usize {
    let mut positions = vec![];
    for s in sensors {
        positions.append(&mut s.hidden_beacon_impossible_positions_in_row(row));
    }
    positions.sort_unstable();
    positions.dedup();
    positions.len()
}

fn tuning_frequency(pos: (isize, isize)) -> isize {
    pos.0 * 4000000 + pos.1
}

fn find_in_box(sensors: &Vec<Sensor>, top_corner: &(isize, isize), height: isize, width: isize) -> Option<(isize, isize)> {
    if sensors.iter().any(|s| s.box_excluded(top_corner, width, height)) {
        return None;
    }
    if height == 1 && width == 1 {
        return Some(*top_corner);
    }
    if width * height <= BRUTE_FORCE_THRESHOLD {
        for y in 0..=height {
            let pos = (0..=width).map(|x| (top_corner.0 + x, top_corner.1 + y))
                .find(|p| !sensors.iter().any(|s| s.scan_contains(*p)));
            if pos.is_some() {
                return pos;
            }
        }
        return None;
    }
    let mid_height = (height + 1) / 2;
    let mid_width = (width + 1) / 2;
    let mut next_gen = vec![
        (*top_corner, mid_height, mid_width),
    ];
    if height > 1 {
        next_gen.push(((top_corner.0, top_corner.1 + mid_height), height - mid_height, width));
    }
    if width > 1 {
        next_gen.push(((top_corner.0 + mid_width, top_corner.1), mid_height, width - mid_width));
    }
    if height > 1 && width > 1 {
        next_gen.push(((top_corner.0 + mid_width, top_corner.1 + mid_height), height - mid_height, width - mid_width));
    }
    next_gen.iter()
        .map(|(top_corner, height, width)| find_in_box(sensors, top_corner, *height, *width))
        .find(|o| o.is_some())
        .map(|o| o.unwrap())

}

fn find_hidden_beacon(range: RangeInclusive<isize>, sensors: &Vec<Sensor>) -> Option<(isize, isize)> {
    let window_size = WINDOW_SIZE;
    for x in range.clone().step_by(window_size as usize) {
        for y in range.clone().step_by(window_size as usize) {
            let res = find_in_box(&sensors, &(x, y), min(window_size, range.end() - y), min(window_size, range.end() - x));
            if res.is_some() {
                return res;
            }
        }
    }
    None
}
