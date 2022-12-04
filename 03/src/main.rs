use std::io;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    let mut duplicate_priorities: u32 = 0;
    let mut badge_priorities: u32 = 0;
    for group in buf.lines().collect::<Vec<&str>>().chunks(3) {
        for line in group {
            duplicate_priorities += priority(find_overlap(&compartments(line))) as u32;
        }
        badge_priorities += priority(badge(group)) as u32;
    }
    println!("The sum of the duplicate item priorities is {}", duplicate_priorities);
    println!("The sum of the badge priorities is {}", badge_priorities);
}

fn compartments(input: &str) -> (String, String) {
    let length = input.len() / 2;
    let mut comp1 = String::new();
    let mut comp2 = String::new();
    for i in 0..length {
        comp1.push(input.chars().nth(i).unwrap());
    }
    for i in length..(2*length) {
        comp2.push(input.chars().nth(i).unwrap());
    }
    (comp1, comp2)
}

fn find_overlap(comps: &(String, String)) -> char {
    comps.0.chars().find(|c| comps.1.contains(*c)).unwrap()
}

fn priority(c: char) -> u8 {
    match c {
        'a'..='z' => c as u8 - 96,
        'A'..='Z' => c as u8 - 65 + 27,
        _ => unimplemented!(),
    }
}

fn badge(group: &[&str]) -> char {
    group[0].chars().filter(|c| group[1].contains(*c))
        .find(|c| group[2].contains(*c)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_one_rucksack() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let result = compartments(input);
        assert_eq!(result, (String::from("vJrwpWtwJgWr"), String::from("hcsFMMfFFhFp")))
    }

    #[test]
    fn single_overlap() {
        let comps = (String::from("vJrwpWtwJgWr"), String::from("hcsFMMfFFhFp"));
        let result = find_overlap(&comps);
        assert_eq!(result, 'p');
    }

    #[test]
    fn priority_lowercase() {
        let c = 'a';
        assert_eq!(priority(c), 1);
    }

    #[test]
    fn priority_uppercase() {
        let c = 'A';
        assert_eq!(priority(c), 27);
    }

    // Part II
    #[test]
    fn find_badge() {
        let group = ["vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg"];
        let result = badge(&group);
        assert_eq!(result, 'r');
    }
}