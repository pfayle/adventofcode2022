use std::collections::VecDeque;
use std::io;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    let result = process_packet(&buf);
    println!("Start of packet marker: {}", result);
    let result2 = process_message(&buf);
    println!("Start of message marker: {}", result2);
}

fn process_marker(input: &str, chars: usize) -> u32 {
    let mut store = VecDeque::new();
    let mut i = 0;
    for c in input.chars() {
        i += 1;
        // if store is not big enough then continue
        if store.len() < chars {
            store.push_back(c);
            continue;
        }
        // if store has no duplicates
        // then return
        if !check_duplicates(&store) {
            return i - 1;
        }
        // pop queue
        // push char to queue
        store.pop_front();
        store.push_back(c);
        // otherwise:
        // next
    }
    unreachable!()
}

fn process_packet(input: &str) -> u32 {
    process_marker(input, 4)
}

fn process_message(input: &str) -> u32 {
    process_marker(input, 14)
}

fn check_duplicates(store: &VecDeque<char>) -> bool {
    let mut observed = vec![];
    for c in store {
        if observed.contains(c) {
            return true;
        } else {
            observed.push(*c);
        }
    }
    false
}
 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abcde_string_returns_4() {
        let input = "abcdefgh";
        let result = process_packet(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn aabcd_returns_5() {
        let input = "aabcdefg";
        let result = process_packet(input);
        assert_eq!(result, 5);
    }

    #[test]
    fn abcd_no_dups() {
        let mut input = VecDeque::new();
        input.push_back('a');
        input.push_back('b');
        input.push_back('c');
        input.push_back('d');
        let result = check_duplicates(&input);
        assert!(!result);
    }

    #[test]
    fn example_string_gives_7() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = process_packet(input);
        assert_eq!(result, 7);
    }

    #[test]
    fn example_string2_gives_11() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let result = process_packet(input);
        assert_eq!(result, 11);
    }

    #[test]
    fn example_string_gives_message_marker_of_19() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = process_message(input);
        assert_eq!(result, 19);
    }

}
