use std::io;
use std::io::Read;
use std::ops::RangeInclusive;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct UnitCube{
    x: isize,
    y: isize,
    z: isize,
}

impl UnitCube {
    fn distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x)
        + self.y.abs_diff(other.y)
        + self.z.abs_diff(other.z)
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    let cubes = parse_input(&buf);
    let mut adjacencies = 0;
    for n in 0..(cubes.len() - 1) {
        for m in (n+1)..cubes.len() {
            if cubes[n].distance(&cubes[m]) == 1 {
                adjacencies += 1;
            }
        }
    }
    let exposed_sides = 6 * cubes.len() - 2 * adjacencies;
    println!("Surface area: {exposed_sides}");
    let boundary = boundary(&cubes);
    let faces = external_faces(&boundary, &cubes, boundary_faces(&boundary), vec![]);
    println!("External surface area: {faces}");
}

fn parse_input(input: &str) -> Vec<UnitCube> {
    input.lines().map(
        |s| s.split(',').map(|num| num.parse::<isize>().unwrap()).collect::<Vec<isize>>()
    ).map(
        |v| UnitCube{
            x: v[0],
            y: v[1],
            z: v[2],
        }
    ).collect::<Vec<UnitCube>>()
}

fn boundary(cubes: &Vec<UnitCube>) -> [RangeInclusive<isize>; 3] {
    let (a, (b, c)): (Vec<isize>, (Vec<isize>, Vec<isize>)) = cubes.iter().map(|c| (c.x, (c.y, c.z))).unzip();
    [
        (*a.iter().min().unwrap()-1)..=(*a.iter().max().unwrap()+1),
        (*b.iter().min().unwrap()-1)..=(*b.iter().max().unwrap()+1),
        (*c.iter().min().unwrap()-1)..=(*c.iter().max().unwrap()+1),
    ]
}

fn boundary_faces(boundary: &[RangeInclusive<isize>; 3]) -> Vec<UnitCube> {
    let mut ret = vec![];
    for x in [boundary[0].clone().min().unwrap(), boundary[0].clone().max().unwrap()] {
        for y in boundary[1].clone() {
            for z in boundary[2].clone() {
                ret.push(UnitCube{x,y,z});
            }
        }
    }
    for x in boundary[0].clone() {
        for y in [boundary[1].clone().min().unwrap(), boundary[1].clone().max().unwrap()] {
            for z in boundary[2].clone() {
                ret.push(UnitCube{x,y,z});
            }
        }
    }
    for x in boundary[0].clone() {
        for y in boundary[1].clone() {
            for z in [boundary[2].clone().min().unwrap(), boundary[2].clone().max().unwrap()] {
                ret.push(UnitCube{x,y,z});
            }
        }
    }
    ret
}

fn external_faces(boundary: &[RangeInclusive<isize>; 3], cubes: &Vec<UnitCube>, to_visit: Vec<UnitCube>, mut visited: Vec<UnitCube>) -> usize {
    let mut ret = 0;
    let mut next_to_visit = vec![];
    for cube in &to_visit {
        visited.push(*cube);
        for index in [0, 1, 2] {
            for length in [-1, 1] {
                let mut new_pos = [cube.x, cube.y, cube.z];
                new_pos[index] += length;
                let adjacency = UnitCube{
                    x: new_pos[0],
                    y: new_pos[1],
                    z: new_pos[2],
                };
                if !boundary[0].contains(&adjacency.x) || !boundary[1].contains(&adjacency.y) || !boundary[2].contains(&adjacency.z) {
                    continue;
                }
                if cubes.contains(&adjacency) {
                    ret += 1;
                } else if (!to_visit.contains(&adjacency)) && !next_to_visit.contains(&adjacency) && !visited.contains(&adjacency) {
                    next_to_visit.push(adjacency);
                }
            }
        }
    }
    if next_to_visit.len() > 0 {
        ret += external_faces(boundary, cubes, next_to_visit, visited);
    }
    ret
}
