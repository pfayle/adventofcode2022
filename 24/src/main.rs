use std::{fmt::Display, collections::HashMap, io::{self, Read}};

type Point=(usize, usize);
type Time=usize;

use petgraph::{prelude::DiGraph, stable_graph::NodeIndex, algo::dijkstra, Direction::{Outgoing, Incoming}};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    let basin = Basin::from(buf.as_str());
    let lcm = basin.lcm();
    let mut states = BasinStates::new(&basin);
    let mut bg = DirectedStateGraph::from(&mut states);
    let j1 = shortest_path(&mut bg, basin.start, 0);
    println!("Shortest path: {j1}");
    bg.reverse();
    let j2 = shortest_path(&mut bg, basin.end, j1%lcm);
    println!("Shortest path back: {j2}");
    bg.reverse();
    let j3 = shortest_path(&mut bg, basin.start, (j1+j2)%lcm);
    println!("Shortest path there again: {j3}");
    println!("Shortest total path: {}", j1+j2+j3);
}

fn shortest_path(bg: &mut DirectedStateGraph, goal: Point, time: Time) -> usize {
    let results = bg.shortest_path_lengths();
    let index = bg.index_map.get(&(goal, time)).unwrap();
    *results.get(index).unwrap()
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Direction {
    W, N, E, S,
}

use Direction::*;
const DIRECTIONS: [Direction; 4] = [N,S,E,W];

impl Direction {
    fn take(&self, point: Point) -> Option<Point> {
        match self {
            W => Self::sub(point, (1,0)),
            N => Self::sub(point, (0,1)),
            E => Some((point.0 + 1, point.1)),
            S => Some((point.0, point.1 + 1)),
        }
    }

    fn sub(pt1: Point, pt2: Point) -> Option<Point> {
        if let (Some(x), Some(y)) = (pt1.0.checked_sub(pt2.0), pt1.1.checked_sub(pt2.1)) {
            Some((x, y))
        } else {
            None
        }
    }

    fn opposite(&self) -> Self {
        match self {
            N => S,
            W => E,
            E => W,
            S => N,
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(N),
            '>' => Ok(E),
            'v' => Ok(S),
            '<' => Ok(W),
            _ => Err(()),
        }
    }
}
impl From<Direction> for char {
    fn from(value: Direction) -> Self {
        match value {
            W => '<',
            N => '^',
            E => '>',
            S => 'v',
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Blizzard{point: Point, dir: Direction}

#[derive(Default, Clone, Debug)]
struct Basin {
    start: Point,
    end: Point,
    width: usize,
    height: usize,
    blizzards: HashMap<Blizzard, Point>,
    points: HashMap<Point, usize>,
    blizzards_new: HashMap<Point, Blizzard>,
}

impl Basin {
    /// lowest common multiple of width & height
    fn lcm(&self) -> usize {
        num::integer::lcm(self.width, self.height)
    }

    fn wrap(&self, point: Point) -> Point {
        let mut p = (point.0 % self.width, point.1 % self.height);
        if p.0 == 0 {
            p.0 = self.width;
        }
        if p.1 == 0 {
            p.1 = self.height;
        }
        p
    }

    fn blizzards_at(&self, point: Point) -> Vec<&Blizzard> {
        self.blizzards.iter().filter(|(_, p)| **p == point)
            .map(|(b, _)| b)
            .collect::<Vec<&Blizzard>>()
    }

    fn points(&self) -> Vec<Point> {
        self.points.keys().cloned().collect()
    }

    fn contains(&self, point: Point) -> bool {
        self.points.contains_key(&point)
    }

    fn neighbours(&self, point: Point) -> Vec<Point> {
        DIRECTIONS.iter().filter_map(|dir| dir.take(point))
            .filter(|pt| self.contains(*pt))
            .collect()
    }

    fn neighbours_and_self(&self, point: Point) -> Vec<Point> {
        let mut ret = self.neighbours(point);
        ret.push(point);
        ret
    }

    fn take_n(&self, dir: Direction, point: Point, n: usize) -> Point {
        if n == 0 {
            return point;
        }
        self.wrap(match dir {
            W => ((point.0 + self.width - (n%self.width))%self.width,point.1),
            N => (point.0, (point.1 + self.height - (n%self.height))%self.height),
            E => ((point.0 + n)%self.width,point.1),
            S => (point.0, (point.1 + n)%self.height)
        })
    }

    fn free(&self, point: Point, time: Time) -> bool {
        if point == self.start || point == self.end {
            return true;
        }
        DIRECTIONS.iter().map(|dir| (dir, self.take_n(*dir, point, time)))
            .map(|(dir, pt)| (dir, self.blizzards_new.get(&pt)))
            .all(|(dir, opt)|
                match opt {
                    Some(b) => b.dir.opposite() != *dir,
                    _ => true
                }
            )
    }
}

impl Display for Basin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        for y in 0..=(self.height+1) {
            for x in 0..=(self.width+1) {
                if self.start == (x,y) || self.end == (x,y) {
                    ret.push('.');
                    continue;
                } else if x==0 || x==self.width+1 || y==0 || y==self.height+1 {
                    ret.push('#');
                    continue;
                }
                let bs = self.blizzards_at((x,y));
                if bs.len() > 1 {
                    ret.push_str(&format!("{}", bs.len()));
                } else if bs.is_empty() {
                    ret.push('.');
                } else {
                    ret.push(bs[0].dir.into());
                }
            }
            ret.push('\n');
        }
        write!(f, "{ret}")
    }
}

impl From<&str> for Basin {
    fn from(value: &str) -> Self {
        let mut basin = Self::default();
        let lines: Vec<(usize, &str)> = value.lines().enumerate().collect();
        basin.height = lines.len() - 2;
        basin.width = lines[0].1.len() - 2;
        basin.start = (lines[0].1.find('.').unwrap(), 0);
        basin.end = (lines[basin.height+1].1.find('.').unwrap(), basin.height+1);
        basin.points.insert(basin.start, 0);
        basin.points.insert(basin.end, 0);
        for (y, line) in &lines[1..=basin.height] {
            for x in 1..=basin.width {
                let c = line.chars().nth(x).unwrap();
                basin.points.insert((x,*y), 0);
                if let Ok(dir) = Direction::try_from(c) {
                    basin.blizzards.insert(Blizzard{
                        point: (x,*y), dir
                    }, (x, *y));
                    basin.blizzards_new.insert((x, *y), Blizzard { point: (x, *y), dir });
                    *basin.points.get_mut(&(x,*y)).unwrap() = 1;
                }
            }
        }
        basin
    }
}

#[derive(Debug)]
struct BasinStates {
    map: HashMap<(Point, Time), bool>,
    basin: Basin
}

impl BasinStates {
    fn new(basin: &Basin) -> Self {
        let map = HashMap::new();
        Self{
            map, basin: basin.clone()
        }
    }

    fn free(&mut self, point: Point, time: Time) -> bool {
        if let Some(x) = self.map.get(&(point, time)) {
            return *x;
        }
        let ret = self.basin.free(point, time);
        self.map.insert((point, time), ret);
        ret
    }
}

struct DirectedStateGraph {
    graph: DiGraph<(), usize>,
    index_map: HashMap<(Point, Time), NodeIndex>,
    start: NodeIndex,
    end: NodeIndex,
}

impl DirectedStateGraph {
    fn shortest_path_lengths(&self) -> HashMap<NodeIndex, usize> {
        dijkstra(&self.graph, self.end, None, |e| *e.weight())
    }

    /// reconnect the start and end nodes to the rest of the graph,
    /// swapping their 0-cost edge directions, but keeping each
    /// with the same neighbours.
    fn reverse(&mut self) {
        let new_end_neighbours = self.graph.neighbors_directed(self.start, Incoming)
            .collect::<Vec<NodeIndex>>();
        let new_start_neighbours = self.graph.neighbors_directed(self.end, Outgoing)
            .collect::<Vec<NodeIndex>>();
        std::mem::swap(&mut self.start, &mut self.end);
        for nb in new_end_neighbours {
            let eix = self.graph.find_edge(nb, self.end).unwrap();
            self.graph.remove_edge(eix);
            self.graph.add_edge(self.end, nb, 0);
        }
        for nb in new_start_neighbours {
            let eix = self.graph.find_edge(self.start, nb).unwrap();
            self.graph.remove_edge(eix);
            self.graph.add_edge(nb, self.start, 0);
        }
    }
}

impl From<&mut BasinStates> for DirectedStateGraph {
    fn from(states: &mut BasinStates) -> Self {
        let mut graph = DiGraph::new();
        let mut index_map = HashMap::new();
        let start = graph.add_node(());
        let end = graph.add_node(());
        for time in 0..states.basin.lcm() {
            for point in &states.basin.points() {
                if !states.free(*point, time) {
                    continue;
                }
                index_map.entry((*point, time)).or_insert_with(|| {
                    graph.add_node(())
                });
                let index = *index_map.get(&(*point, time)).unwrap();
                let time2 = (time+1)%states.basin.lcm();
                for neighbour in states.basin.neighbours_and_self(*point) {
                    if !states.free(neighbour, time2) {
                        continue;
                    }
                    if let Some(n_index) = index_map.get(&(neighbour, time2)) {
                        graph.add_edge(*n_index, index, 1);
                    } else {
                        let n_index = graph.add_node(());
                        index_map.insert((neighbour, time2), n_index);
                        graph.add_edge(n_index, index, 1);
                    }
                }
            }
        }
        for t in 0..states.basin.lcm() {
            let start2 = *index_map.get(&(states.basin.start, t)).unwrap();
            graph.add_edge(start2, start, 0);
            let end2 = *index_map.get(&(states.basin.end, t)).unwrap();
            graph.add_edge(end, end2, 0);
        }
        Self { graph, index_map, start, end }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    impl Basin {
        fn new(start: Point, end: Point, width: usize, height: usize, blizzards: HashMap<Blizzard, Point>) -> Self {
            let mut points = HashMap::new();
            for y in 0..height {
                for x in 0..width {
                    points.insert((x+1, y+1), 0);
                }
            }
            let mut blizzards_new = HashMap::new();
            for (b, pt) in &blizzards {
                *points.get_mut(pt).unwrap() += 1;
                blizzards_new.insert(*pt, b.clone());
            }
            points.insert(start, 0);
            points.insert(end, 0);
            Self { start, end, width, height, blizzards, points, blizzards_new }
        }
    }

    fn basin1() -> Basin {
        Basin::new(
            (1,0),
            (3,8),
            5,
            7,
            HashMap::from([
                (Blizzard{point:(3,3), dir:N}, (3,3)),
                (Blizzard{point:(3,3), dir:S}, (3,3)),
                (Blizzard{point:(4,4), dir:W}, (4,4)),
            ])
        )
    }

    fn basin2() -> Basin {
        Basin::new(
            (1,0),
            (4,8),
            5,
            7,
            HashMap::from([
                (Blizzard{point:(3,3), dir:N}, (3,3)),
                (Blizzard{point:(3,3), dir:S}, (3,3)),
                (Blizzard{point:(4,4), dir:W}, (4,4)),
            ]),
        )
    }

    // #.##
    // #<.#
    // #.>#
    // ##.#
    fn tiny_basin() -> Basin {
        Basin::new(
            (1,0),
            (2,3),
            2,
            2,
            HashMap::from([
                (Blizzard{point:(1,1), dir:W}, (1,1)),
                (Blizzard{point:(2,2), dir:E}, (2,2)),
            ]),
        )
    }

    #[test]
    fn display_basin() {
        let basin = basin1();
        println!("{basin}");
        assert_eq!(
            format!("{basin}"),
            "
#.#####
#.....#
#.....#
#..2..#
#...<.#
#.....#
#.....#
#.....#
###.###
".trim_start()
        );
    }

    #[test]
    fn test_lcm() {
        assert_eq!(num::integer::lcm(4, 6), 12);
        let basin = basin1();
        assert_eq!(basin.lcm(), 35);
    }

    #[test]
    fn points_size() {
        let basin = basin1();
        assert_eq!(basin.points().len(), 37);
    }

    #[test]
    fn points() {
        let basin = tiny_basin();
        let expected = HashMap::from([
            ((1,0), 0),
            ((1,1), 1),
            ((1,2), 0),
            ((2,1), 0),
            ((2,2), 1),
            ((2,3), 0),
        ]);
        for (k, v) in &expected {
            dbg!(k);
            assert!(basin.points.contains_key(k));
            assert_eq!(basin.points.get(k), Some(v));
        }
        assert_eq!(basin.points, expected);
    }

    #[test]
    fn neighbours_example() {
        let basin = basin2();
        assert_eq!(basin.neighbours((4,7)), vec![
            (4,6),(4,8),(5,7),(3,7)
        ]);
        assert_eq!(basin.neighbours((5,6)), vec![
            (5,5),(5,7),(4,6),
        ]);
        let basin = Basin::new(
            (1,0),
            (5,6),
            5,
            5,
            HashMap::from([
                (Blizzard{point:(1,2), dir:N}, (1,2)),
                (Blizzard{point:(4,4), dir:S}, (4,4)),
            ]),
        );
        assert_eq!(basin.neighbours((5,5)), vec![
            (5,4),(5,6),(4,5)
        ]);
    }

    #[test]
    fn try_free() {
        let basin = tiny_basin();
        let free_world: Vec<(Point, Time)> = [
            ((1,1),0), ((1,2),0), ((2,1),0), ((2,2),0),
            ((1,1),1), ((1,2),1), ((2,1),1), ((2,2),1),
            ((1,1),2), ((1,2),2), ((2,1),2), ((2,2),2),
            ].into_iter()
            .filter(|(pt,t)| basin.free(*pt, *t))
            .collect();
        assert_eq!(free_world, vec![
            ((1,2),0), ((2,1),0),
            ((1,1),1), ((2,2),1),
            ((1,2),2), ((2,1),2),
        ]);
    }
}
