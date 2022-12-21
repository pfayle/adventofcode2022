use std::io::{self, Read};
// use std::collections::VecDeque;

const INDICES: [usize; 3] = [1000, 2000, 3000];
const DECRYPTION_KEY: isize = 811589153;
const MIXING_CYCLES: usize = 10;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    let file = parse_input(&buf);
    let mixed_file = file.mix();
    let grove_coordinates = mixed_file.grove_coordinates();
    println!("Grove coordinates {:?}; sum: {}", grove_coordinates, grove_coordinates.iter().sum::<isize>());
    let decrypted_file = file.decrypt();
    let mixed_file = decrypted_file.mix_times(MIXING_CYCLES);
    let grove_coordinates = mixed_file.grove_coordinates();
    println!("Decrypted grove coordinates {:?}; sum: {}", grove_coordinates, grove_coordinates.iter().sum::<isize>());
}

// type List = VecDeque<isize>;
type List = Vec<isize>;

#[derive(Debug)]
struct EncryptedFile {
    list: List,
}

impl EncryptedFile {
    fn index_map(size: usize, source: usize, target: usize) -> Vec<usize> {
        let mut ret = vec![];
        if target > source {
            for ix in 0..source {
                ret.push(ix);
            }
            for ix in source..target {
                ret.push(ix+1);
            }
            ret.push(source);
            for ix in (target+1)..size {
                ret.push(ix);
            }
        }
        if target < source {
            for ix in 0..target {
                ret.push(ix);
            }
            ret.push(source);
            for ix in (target+1)..=source {
                ret.push(ix-1);
            }
            for ix in (source+1)..size {
                ret.push(ix);
            }
        }
        if target == source {
            ret = (0..size).collect();
        }
        ret
    }

    fn mix_step(&self, source_index: usize, number: isize) -> Vec<usize> {
        let target_index = index(source_index as isize + number, self.list.len());
        Self::index_map(self.list.len(), source_index, target_index)
    }

    fn mix(&self) -> Self {
        self.mix_times(1)
    }

    fn mix_times(&self, times: usize) -> Self {
        let mut ret = EncryptedFile{
            list: self.list.clone()
        };
        let mut map: Vec<usize> = (0..self.list.len()).collect();
        let iter = self.list.iter().enumerate().cycle().take(times * self.list.len());
        for (ix, number) in iter {
            let index = map.iter().position(|n| *n == ix).unwrap();
            let new_map = ret.mix_step(index, *number);
            map = new_map.iter().map(|ix| map[*ix]).collect::<Vec<usize>>();
            ret.list = map.iter().map(|ix| self.list[*ix]).collect::<List>();
        }
        ret
    }

    fn grove_coordinates(&self) -> Vec<isize> {
        let zero_position = self.list.iter().position(|n| *n == 0).unwrap();
        INDICES.iter()
            .map(|ix| self.list[(*ix + zero_position) % self.list.len()])
            .collect::<Vec<isize>>()
    }

    fn decrypt(&self) -> Self {
        Self {
            list: self.list.iter().map(
                |n| *n * DECRYPTION_KEY
            ).collect::<List>()
        }
    }
}

fn index(n: isize, m: usize) -> usize {
    if n > 0 {
        n as usize % (m - 1)
    } else {
        let ret = ((n % (m - 1) as isize) + m as isize - 1) as usize;
        if ret == 0 {
            m
        } else {
            ret
        }
    }
}

fn parse_input(input: &str) -> EncryptedFile {
    let list = input.lines().map(
        |num| num.parse::<isize>().unwrap()
    ).collect::<List>();
    EncryptedFile { list }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_one_to_the_right() {
        let input = vec![1, 2, 3];
        let result = EncryptedFile::index_map(input.len(), 0, 1);
        assert_eq!(result, vec![1, 0, 2]);
        let ret = result.iter().map(|ix| input[*ix]).collect::<List>();
        assert_eq!(ret, vec![2, 1, 3]);
    }

    #[test]
    fn move_one_in_the_middle_to_the_right() {
        let input = vec![2, 1, 3];
        let result = EncryptedFile::index_map(input.len(), 1, 0);
        assert_eq!(result, vec![1, 0, 2]);
        let ret = result.iter().map(|ix| input[*ix]).collect::<List>();
        assert_eq!(ret, vec![1, 2, 3]);
    }

    #[test]
    fn move_one_at_the_right_to_the_right() {
        let input = vec![2, 3, 1];
        let result = EncryptedFile::index_map(input.len(), 2, 1);
        assert_eq!(result, vec![0, 2, 1]);
        let ret = result.iter().map(|ix| input[*ix]).collect::<List>();
        assert_eq!(ret, vec![2, 1, 3]);
    }

    #[test]
    fn move_two_to_the_right() {
        let input = vec![2, 3, 4];
        let result = EncryptedFile::index_map(input.len(), 1, 1);
        assert_eq!(result, vec![0, 1, 2]);
        let ret = result.iter().map(|ix| input[*ix]).collect::<List>();
        assert_eq!(ret, vec![2, 3, 4]);
    }

    #[test]
    fn move_three_to_the_right() {
        // let input: EncryptedFile = EncryptedFile { list: vec![3, 4, 5].into() };
        // let result = input.mix_step_deprecated(3);
        // assert_eq!(result.list, vec![4, 3, 5]);
        let input = vec![3, 4, 5];
        let result = EncryptedFile::index_map(input.len(), 0, 1);
        assert_eq!(result, vec![1, 0, 2]);
        let ret = result.iter().map(|ix| input[*ix]).collect::<List>();
        assert_eq!(ret, vec![4, 3, 5]);
    }

    #[test]
    fn move_five_to_the_right() {
        // let input: EncryptedFile = EncryptedFile { list: vec![5, 6, 7].into() };
        // let result = input.mix_step_deprecated(5);
        // assert_eq!(result.list, vec![6, 5, 7]);
        let input = vec![5, 6, 7];
        let result = EncryptedFile::index_map(input.len(), 0, 1);
        assert_eq!(result, vec![1, 0, 2]);
        let ret = result.iter().map(|ix| input[*ix]).collect::<List>();
        assert_eq!(ret, vec![6, 5, 7]);
    }

    #[test]
    fn move_one_to_the_left() {
        let input = vec![-1, 0, 1];
        let result = EncryptedFile::index_map(input.len(), 0, 1);
        assert_eq!(result, vec![1, 0, 2]);
        let ret = result.iter().map(|ix| input[*ix]).collect::<List>();
        assert_eq!(ret, vec![0, -1, 1]);
    }

    #[test]
    fn move_one_in_the_middle_to_the_left() {
        let input = vec![0, -1, 1];
        let result = EncryptedFile::index_map(input.len(), 1, 2);
        assert_eq!(result, vec![0, 2, 1]);
        let ret = result.iter().map(|ix| input[*ix]).collect::<List>();
        assert_eq!(ret, vec![0, 1, -1]);
    }

    #[test]
    fn move_one_at_the_right_to_the_left() {
        let input = vec![0, 1, -1];
        let result = EncryptedFile::index_map(input.len(), 2, 1);
        assert_eq!(result, vec![0, 2, 1]);
        let ret = result.iter().map(|ix| input[*ix]).collect::<List>();
        assert_eq!(ret, vec![0, -1, 1]);
    }

    #[test]
    fn move_two_to_the_left() {
        let input = vec![-2, -1, 0];
        let result = EncryptedFile::index_map(input.len(), 0, 2);
        assert_eq!(result, vec![1, 2, 0]);
        let ret = result.iter().map(|ix| input[*ix]).collect::<List>();
        assert_eq!(ret, vec![-1, 0, -2]);
    }

    #[test]
    fn combine_two_moves() {
        let input = vec![-2, -1, 0];
        let result1 = EncryptedFile::index_map(input.len(), 0, 2);
        assert_eq!(result1, vec![1, 2, 0]);
        let result2 = EncryptedFile::index_map(input.len(), 1, 0);
        assert_eq!(result2, vec![1, 0, 2]);
        let combined_result = result2.iter().map(|ix| result1[*ix]).collect::<Vec<usize>>();
        assert_eq!(combined_result, vec![2, 1, 0]);
        let ret1 = result1.iter().map(|ix| input[*ix]).collect::<List>();
        assert_eq!(ret1, vec![-1, 0, -2]);
        let ret2 = result2.iter().map(|ix| ret1[*ix]).collect::<List>();
        assert_eq!(ret2, vec![0, -1, -2]);
        let combined_ret = result2.iter()
            .map(|ix| input[result1[*ix]])
            .collect::<List>();
        assert_eq!(combined_ret, vec![0, -1, -2]);
        let combined_ret2 = (0..input.len()).map(|ix| input[result1[result2[ix]]]).collect::<List>();
        assert_eq!(combined_ret2, vec![0, -1, -2]);
        assert_eq!(combined_ret2, vec![input[combined_result[0]], input[combined_result[1]], input[combined_result[2]]]);
    }

    #[test]
    fn move_all_numbers() {
        let input: EncryptedFile = EncryptedFile { list: vec![1, 2, -3, 3, -2, 0, 4].into() };
        let result = input.mix();
        assert_eq!(result.list, vec![1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn example_case_1() {
        let input = vec![4, 5, 6, 1, 7, 8, 9];
        let result = EncryptedFile::index_map(input.len(), 3, 4);
        assert_eq!(result, vec![0, 1, 2, 4, 3, 5, 6]);
        let ret = result.iter().map(|ix| input[*ix]).collect::<List>();
        assert_eq!(ret, vec![4, 5, 6, 7, 1, 8, 9]);
    }

    #[test]
    fn reverse_example_case_1() {
        let input = vec![4, 5, 6, -5, 7, 8, 9];
        let result = EncryptedFile::index_map(input.len(), 3, 4);
        assert_eq!(result, vec![0, 1, 2, 4, 3, 5, 6]);
        let ret = result.iter().map(|ix| input[*ix]).collect::<List>();
        assert_eq!(ret, vec![4, 5, 6, 7, -5, 8, 9]);
    }

    #[test]
    fn example_case_2() {
        let input = vec![4, -2, 5, 6, 7, 8, 9];
        let result = EncryptedFile::index_map(input.len(), 1, 5);
        assert_eq!(result, vec![0, 2, 3, 4, 5, 1, 6]);
        let ret = result.iter().map(|ix| input[*ix]).collect::<List>();
        assert_eq!(ret, vec![4, 5, 6, 7, 8, -2, 9]);
    }

    #[test]
    fn backwards_example_case_2() {
        let input = vec![4, 8, 5, 6, 7, -2, 9];
        let result = EncryptedFile::index_map(input.len(), 5, 3);
        assert_eq!(result, vec![0, 1, 2, 5, 3, 4, 6]);
        let ret = result.iter().map(|ix| input[*ix]).collect::<List>();
        assert_eq!(ret, vec![4, 8, 5, -2, 6, 7, 9]);
    }

}
