use std::io;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    let result = find_elf_with_most_calories(&buf);
    println!("The biggest elf has {} calories.", &result);
    let result2 = find_elves_with_most_calories(&buf);
    println!("The biggest elves have {} calories.", &result2);
}

fn process_calories(text: &String) -> Vec<i32> {
    let mut calories: Vec<i32> = Vec::new();
    for elfs_calories in text.split("\n\n") {
        let single_elf_calories = process_elfs_calories(&elfs_calories.to_string());
        calories.push(single_elf_calories);
    }
    calories
}

fn process_elfs_calories(text: &String) -> i32 {
    let mut calories = 0;
    for line in text.lines() {
        calories += match line.parse::<i32>(){
            Ok(integer) => integer,
            Err(_) => panic!("failed to parse string as integer")
        };
    }
    calories
}

fn find_elf_with_most_calories(text: &String) -> i32 {
    let calories = process_calories(text);
    *calories.iter().max().unwrap()
}

fn find_elves_with_most_calories(text: &String) -> i32 { 
    let mut calories = process_calories(text);
    calories.sort_unstable();
    let mut result = 0;
    calories.reverse();
    let mut iter = calories.iter();
    result += iter.next().unwrap();
    result += iter.next().unwrap(); 
    result += iter.next().unwrap();
    result 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elf_with_single_line_returns_that_value() {
        let text = "1000\n".to_string();
        let result = process_elfs_calories(&text);
        assert_eq!(result, 1000);
    }

    #[test]
    fn elf_with_two_line_returns_the_sum() {
        let text = "1000\n2000\n".to_string();
        let result = process_elfs_calories(&text);
        assert_eq!(result, 3000);
    }

    #[test]
    fn two_elves_are_separated() {
        let text = "1000\n\n2000\n".to_string();
        let result = process_calories(&text);
        assert_eq!(result, vec![1000, 2000]);
    }
    // process_elfs_calories -> 1000
    // process_calories -> [1000, 2000]

    #[test]
    fn return_the_higher_of_two_elves() {
        let text = "1000\n\n2000\n".to_string();
        let result = find_elf_with_most_calories(&text);
        assert_eq!(result, 2000);
    }
    
    // #[test]
    // fn aoc_example() {
        // let text = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"
        // let result = find_elf_with_most_calories(text);
        // assert_eq!(result, 24000);
    // }
}