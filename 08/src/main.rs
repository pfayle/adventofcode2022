use std::io;
use std::io::Read;

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

const DIRECTIONS: [Dir; 4] = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    let pts = string_to_points(&buf);
    println!("{} trees are visible", visible(&pts));
    println!("Max scenic score: {}", max_scenic_score(&pts));
}

fn points_iter(pts: &Vec<Vec<u8>>) -> impl Iterator<Item = (usize, usize, u8)> {
    let mut flat = vec![];
    for (i, row) in pts.iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            flat.push((i, j, *v));
        }
    }
    flat.into_iter()
}

fn visible(pts: &Vec<Vec<u8>>) -> u32 {
    let mut visible = 0;
    for (i, j, _) in points_iter(pts) {
        for dir in DIRECTIONS {
            if visible_from(dir, &(i, j), pts) {
                visible += 1;
                break;
            }
        }
    }
    visible
}

fn max_scenic_score(pts: &Vec<Vec<u8>>) -> usize {
    let mut scores = vec![];
    for (i, j, _) in points_iter(pts) {
        scores.push(scenic_score((i, j), pts));
    }
    *scores.iter().max().unwrap()
}

fn scenic_score((i, j): (usize, usize), pts: &Vec<Vec<u8>>) -> usize {
    [Dir::Up, Dir::Down, Dir::Left, Dir::Right].iter().map(|&dir|
    viewing_distance(dir, (i, j), &pts)).product()
}

fn viewing_distance(dir: Dir, (i, j): (usize, usize), pts: &Vec<Vec<u8>>) -> usize {
    let height = pts[i][j];
    let mut distance = 0;
    match dir {
        Dir::Up => {
            for n in (0..i).rev() {
                distance += 1;
                if pts[n][j] >= height {
                    break;
                } 
            }
        },
        Dir::Down => {
            for n in (i+1)..pts.len() {
                distance += 1;
                if pts[n][j] >= height {
                    break;
                } 
            }
        },
        Dir::Left => {
            for m in (0..j).rev() {
                distance += 1;
                if pts[i][m] >= height {
                    break;
                } 
            }
        },
        Dir::Right => {
            for m in (j+1)..pts[i].len() {
                distance += 1;
                if pts[i][m] >= height {
                    break;
                } 
            }
        },
    }
    distance
}

fn visible_from(dir: Dir, pt: &(usize, usize), pts: &Vec<Vec<u8>>) -> bool {
    let i = pt.0;
    let j = pt.1;
    let height = pts[i][j];
    match dir {
        Dir::Up => {
            for n in 0..i {
                if pts[n][j] >= height {
                    return false;
                } 
            }
        },
        Dir::Down => {
            for n in (i+1)..pts.len() {
                if pts[n][j] >= height {
                    return false;
                } 
            }
        },
        Dir::Left => {
            for m in 0..j {
                if pts[i][m] >= height {
                    return false;
                } 
            }
        },
        Dir::Right => {
            for m in (j+1)..pts[i].len() {
                if pts[i][m] >= height {
                    return false;
                } 
            }
        },
    }
    true
}

fn string_to_points(input: &str) -> Vec<Vec<u8>> {
    let mut ret = vec![];
    for line in input.lines() {
        ret.push(line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect());
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_cell_is_visible() {
        let pts = vec![vec![1]];
        let result = visible(&pts);
        assert_eq!(result, 1);
    }

    #[test]
    fn single_row_is_visible() {
        let pts = vec![vec![1, 1, 1, 1, 1]];
        let result = visible(&pts);
        assert_eq!(result, 5);
    }

    #[test]
    fn small_square_is_visible() {
        let pts = vec![vec![1, 1], vec![1, 1]];
        let result = visible(&pts);
        assert_eq!(result, 4);
    }

    #[test]
    fn hidden_centre_is_invisible() {
        let pts = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let result = visible(&pts);
        assert_eq!(result, 8);
    }

    #[test]
    fn invisible_from_left() {
        let result = visible_from(Dir::Left, &(0, 1), &vec![vec![2, 1]]);
        assert!(!result);
    }

    #[test]
    fn invisible_from_right() {
        let result = visible_from(Dir::Right, &(0, 0), &vec![vec![1, 2]]);
        assert!(!result);
    }

    #[test]
    fn invisible_from_top() {
        let result = visible_from(Dir::Up, &(1, 0), &vec![vec![2], vec![1]]);
        assert!(!result);
    }

    #[test]
    fn invisible_from_bottom() {
        let result = visible_from(Dir::Down, &(0, 0), &vec![vec![1], vec![2]]);
        assert!(!result);
    }

    #[test]
    fn outside_visible() {
        let result = visible_from(Dir::Down, &(0, 0), &vec![vec![1, 1, 1], vec![1, 1, 1]]);
        assert!(!result);
    }

    #[test]
    fn scenic_scores() {
        let pts = vec![
            vec![3, 0, 3, 7, 3,],
            vec![2, 5, 5, 1, 2,],
            vec![6, 5, 3, 3, 2,],
            vec![3, 3, 5, 4, 9,],
            vec![3, 5, 3, 9, 0,],
        ];
        assert_eq!(viewing_distance(Dir::Up, (3, 2), &pts), 2);
        assert_eq!(viewing_distance(Dir::Left, (3, 2), &pts), 2);
        assert_eq!(viewing_distance(Dir::Down, (3, 2), &pts), 1);
        assert_eq!(viewing_distance(Dir::Right, (3, 2), &pts), 2);
    }
}