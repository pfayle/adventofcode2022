use std::{io};
use std::io::Read;

const START: char = 'S';
const END: char = 'E';

struct Square {
    elevation: char,
}

impl Square {
    fn height(&self) -> u8 {
        match self.elevation {
            START => 0,
            END => 25,
            _ => self.elevation as u8 - 97,
        }
    }
}

struct SquareVisitor {
    square: Square,
    visited: bool,
}

fn main() {
    let mut hill: Vec<Vec<SquareVisitor>> = vec![];
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    let (start, possible_starts) = initialise_map(&buf, &mut hill);
    let first_route = shortest_path(start, &mut hill, None);
    println!("Shortest route: {}", first_route);
    let mut min = first_route;
    for st in possible_starts {
        reset_visitors(&mut hill);
        min = shortest_path(st, &mut hill, Some(min));
    }
    println!("Shortest of all routes: {}", min);
}

fn initialise_map(input: &String, map: &mut Vec<Vec<SquareVisitor>>) -> ((usize, usize), Vec<(usize, usize)>) {
    let mut start = (0, 0);
    let mut starts = vec![];
    for (i, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (j, c) in line.char_indices() {
            let sv = SquareVisitor{
                square: Square{
                    elevation: c,
                },
                visited: false,
            };
            if c == START {
                start = (i, j);
            }
            if sv.square.height() == 0 { 
                starts.push((i, j));
            }
            row.push(sv);
        }
        map.push(row);
    }
    (start, starts)
}

fn neighbour_coords((i, j): (usize, usize), map: &Vec<Vec<SquareVisitor>>) -> Vec<(usize, usize)> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().filter(|inc| {
        (0..map.len() as isize).contains(&(inc.0 + i as isize))
        && (0..map[0].len() as isize).contains(&(inc.1 + j as isize))
    }).map(|inc| ((inc.0 + i as isize) as usize, (inc.1 + j as isize) as usize)).collect()
}

fn unvisited_neighbours((i, j): (usize, usize), map: &Vec<Vec<SquareVisitor>>) -> Vec<(usize, usize)> {
    let h = map[i][j].square.height();
    neighbour_coords((i, j), map).iter().map(|(ix, jx)| (*ix, *jx)).filter(|(ix, jx)| {
            let sv = &map[*ix][*jx];
            !sv.visited && sv.square.height() <= h + 1
        })
        .collect()
}

fn shortest_path(start: (usize, usize), hill: &mut Vec<Vec<SquareVisitor>>, min: Option<usize>) -> usize {
    let mut recent = vec![start];
    let mut n = 0;
    loop {
        recent = recent.iter().flat_map(|pt| unvisited_neighbours(*pt, &hill)).collect();
        recent.sort_unstable();
        recent.dedup();
        for pt in &recent {
            hill[pt.0][pt.1].visited = true;
        }
        n += 1;
        if recent.iter().find(|(i, j)| hill[*i][*j].square.elevation == END).is_some() {
            break;
        }
        if min.is_some() && min.unwrap() <= n {
            return n;
        }
    }
    n
}

fn reset_visitors(hill: &mut Vec<Vec<SquareVisitor>>) {
    for sv in hill.iter_mut().flatten() {
        sv.visited = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heights() {
        let sq = Square{
            elevation: 'a'
        };
        assert_eq!(sq.height(), 0);
        let sq = Square{
            elevation: 'S'
        };
        assert_eq!(sq.height(), 0);
        let sq = Square{
            elevation: 'E'
        };
        assert_eq!(sq.height(), 25);
    }

    #[test]
    fn neighbour_coord() {
        let mut map = vec![];
        for _ in 0..=2 {
            let mut row = vec![];
            for _ in 0..=3 {
                row.push(SquareVisitor{
                    square: Square{
                        elevation: 'b',
                    },
                    visited: false,
                });
            }
            map.push(row);
        }
        assert_eq!(neighbour_coords((0, 0), &map), vec![(1, 0), (0, 1)]);
        assert_eq!(neighbour_coords((1, 0), &map), vec![(0, 0), (2, 0), (1, 1)]);
        assert_eq!(neighbour_coords((0, 1), &map), vec![(1, 1), (0, 0), (0, 2)]);
        assert_eq!(neighbour_coords((1, 1), &map), vec![(0, 1), (2, 1), (1, 0), (1, 2)]);
        assert_eq!(neighbour_coords((1, 3), &map), vec![(0, 3), (2, 3), (1, 2)]);
        assert_eq!(neighbour_coords((2, 1), &map), vec![(1, 1), (2, 0), (2, 2)]);
    }
}