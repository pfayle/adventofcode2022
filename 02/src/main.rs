use std::io;
use std::io::Read;

#[derive(Debug, PartialEq)]
enum Result {
    Win,
    Draw,
    Loss,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

use crate::Result::*;
use crate::Shape::*;

fn main() {
    // read the file into a string
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    // print out string to test
    // println!("{}", buf);
    // dbg!(&buf);
    // calculate points for example
    let part1_points = run_strategy(&buf);
    println!("Part1 strategy gave {} points", part1_points);
    let part2_points = run_strategy_part2(&buf);
    println!("Part2 strategy gave {} points", part2_points);
}

fn calculate_result_and_points(choice1: Shape, choice2: Shape) -> (Result, i8) {
    let result = calculate_result(&choice1, &choice2);
    let points = calculate_points(&choice2, &result);
    (result, points)
}

fn calculate_points(your_choice: &Shape, result: &Result) -> i8 {
    let mut points = 0;
    // score for your chosen shape
    points += match your_choice {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };
    // score for winning or losing
    points += match *result {
        Win => 6,
        Draw => 3,
        Loss => 0,
    };
    points
}

fn calculate_result(op_choice: &Shape, your_choice: &Shape) -> Result {
    let op_score = match &op_choice {
        Rock => 0,
        Paper => 1,
        Scissors => 2,
    };
    let your_score = match &your_choice {
        Rock => 0,
        Paper => 1,
        Scissors => 2,
    };
    let result_score = (3 + your_score - op_score) % 3;
    match result_score {
        0 => Draw,
        1 => Win,
        2 => Loss,
        _ => unreachable!(),
    }
}

fn calculate_shape(opponent_shape: &Shape, result: &Result) -> (Shape, i8) {
    for your_shape in &[Rock, Paper, Scissors] {
        if calculate_result(opponent_shape, your_shape) == *result {
            return (*your_shape, calculate_points(your_shape, result));
        }
    }
    unreachable!()
}

fn run_strategy(strategy: &str) -> i32 {
    let mut points = 0;
    for line in strategy.lines() {
        // "A Z" -> 'A', 'Z' -> calculate('A', 'Z')
        let mut chars = line.chars();
        let char1 = chars.next().unwrap();
        chars.next().unwrap();
        let char2 = chars.next().unwrap();
        // dbg!(char1, char2);
        let choice1 = match char1 {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissors,
            _ => unimplemented!(),
        };
        let choice2 = match char2 {
            'X' => Rock,
            'Y' => Paper,
            'Z' => Scissors,
            _ => unimplemented!(),
        };
        let (_, pts) = calculate_result_and_points(choice1, choice2);
        points += i32::from(pts);
    }
    points
}

fn run_strategy_part2(strategy: &str) -> i32 {
    let mut points = 0;
    for line in strategy.lines() {
        // "A Z" -> 'A', 'Z' -> calculate_shape('A', 'Z')
        let mut chars = line.chars();
        let char1 = chars.next().unwrap();
        chars.next().unwrap();
        let char2 = chars.next().unwrap();
        // dbg!(char1, char2);
        let choice1 = match char1 {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissors,
            _ => unimplemented!(),
        };
        let result = match char2 {
            'X' => Loss,
            'Y' => Draw,
            'Z' => Win,
            _ => unimplemented!(),
        };
        let (_, pts) = calculate_shape(&choice1, &result);
        points += i32::from(pts);
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rock_and_paper_gives_win() {
        let (result, _) = calculate_result_and_points(Rock, Paper);
        assert_eq!(result, Win);
    }

    #[test]
    fn rock_and_paper_gives_8pts() {
        let (_, points) = calculate_result_and_points(Rock, Paper);
        assert_eq!(points, 8);
    }

    #[test]
    fn paper_and_rock_gives_loss() {
        let (result, _) = calculate_result_and_points(Paper, Rock);
        assert_eq!(result, Loss);
    }

    #[test]
    fn paper_and_rock_gives_1pt() {
        let (_, points) = calculate_result_and_points(Paper, Rock);
        assert_eq!(points, 1);
    }

    #[test]
    fn double_scissors_gives_draw() {
        let (result, _) = calculate_result_and_points(Scissors, Scissors);
        assert_eq!(result, Draw);
    }

    #[test]
    fn double_scissors_gives_6pts() {
        let (_, points) = calculate_result_and_points(Scissors, Scissors);
        assert_eq!(points, 6);
    }

    #[test]
    fn example_strategy_gives_15pts() {
        let strategy = "A Y\nB X\nC Z\n";
        let points = run_strategy(strategy);
        assert_eq!(points, 15);
    }

    // Part II
    #[test]
    fn rock_and_draw_gives_rock() {
        let (shape, _) = calculate_shape(&Rock, &Draw);
        assert_eq!(shape, Rock);
    }

    #[test]
    fn rock_and_draw_gives_4pts() {
        let (_, points) = calculate_shape(&Rock, &Draw);
        assert_eq!(points, 4);
    }

    #[test]
    fn paper_and_loss_gives_rock() {
        let (shape, _) = calculate_shape(&Paper, &Loss);
        assert_eq!(shape, Rock);
    }

    #[test]
    fn paper_and_loss_gives_1pt() {
        let (_, points) = calculate_shape(&Paper, &Loss);
        assert_eq!(points, 1);
    }

    #[test]
    fn scissors_and_win_gives_rock() {
        let (shape, _) = calculate_shape(&Scissors, &Win);
        assert_eq!(shape, Rock);
    }

    #[test]
    fn scissors_and_win_gives_7pts() {
        let (_, points) = calculate_shape(&Scissors, &Win);
        assert_eq!(points, 7);
    }

    #[test]
    fn example_strategy_part2_gives_12pts() {
        let strategy = "A Y\nB X\nC Z\n";
        let points = run_strategy_part2(strategy);
        assert_eq!(points, 12);
    }


}
