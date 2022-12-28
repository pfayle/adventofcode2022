use std::fmt::Display;
use std::io;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    let snafus = parse_input(&buf);
    let sum: isize = snafus.iter().cloned().map(isize::from).sum();
    println!("Snafu sum {sum}; encoded: {}", Snafu::from(sum));
}

fn parse_input(input: &str) -> Vec<Snafu> {
    input.lines().map(Snafu::from).collect()
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum SnafuDigit{
    DoubleMinus,
    Minus,
    Zero,
    One,
    Two,
}

use SnafuDigit::*;

impl From<char> for SnafuDigit {
    fn from(value: char) -> Self {
        match value {
            '=' => DoubleMinus,
            '-' => Minus,
            '0' => Zero,
            '1' => One,
            '2' => Two,
            _ => unimplemented!()
        }
    }
}

impl From<SnafuDigit> for char {
    fn from(value: SnafuDigit) -> Self {
        match value {
            DoubleMinus => '=',
            Minus => '-',
            Zero => '0',
            One => '1',
            Two => '2',
        }
    }
}

impl From<isize> for SnafuDigit {
    fn from(value: isize) -> Self {
        match value {
            -2 => DoubleMinus,
            -1 => Minus,
            0 => Zero,
            1 => One,
            2 => Two, 
            _ => unimplemented!(),
        }
    }
}

impl From<SnafuDigit> for isize {
    fn from(value: SnafuDigit) -> Self {
        match value {
            DoubleMinus => -2,
            Minus => -1,
            Zero => 0,
            One => 1,
            Two => 2,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Snafu {
    data: Vec<SnafuDigit>,
}

impl Snafu {
    fn power(ex: u32) -> usize {
        5_usize.pow(ex)
    }

    fn next_power(data: isize) -> (u32, usize) {
        let mut ret = (0, 1);
        while ret.1 == 1 || data / ((ret.1-1)/2) as isize != 0 {
            ret.0 += 1;
            ret.1 = Self::power(ret.0);
        }
        ret
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str: String = self.data.iter().map(|dig| char::from(*dig)).collect();
        write!(f, "{str}")
    }
}

impl From<&str> for Snafu {
    fn from(value: &str) -> Self {
        let data = value.chars().rev().map(SnafuDigit::from).collect::<Vec<SnafuDigit>>();
        Self { data }
    }
}

impl From<Snafu> for isize {
    fn from(value: Snafu) -> Self {
        let mut data = 0;
        for (i, sd) in value.data.into_iter().enumerate() {
            let v: isize = sd.into();
            data += 5_isize.pow(i as u32)*v;
        }
        data
    }
}

impl From<isize> for Snafu {
    fn from(value: isize) -> Self {
        let mut rem = value;
        let (exp, _) = Snafu::next_power(rem);
        let mut data = vec![];
        for n in (0..exp).rev() {
            let power = Snafu::power(n);
            let coeff = rem.signum() * (rem.abs() + (power as isize -1)/2)/ power as isize;
            rem -= coeff * power as isize;
            data.push(coeff.into());
        }
        Self{ data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_to_snafu() {
        let input = "=";
        let snafu = Snafu::from(input);
        assert_eq!(snafu.data, vec![DoubleMinus]);
    }

    #[test]
    fn first_example() {
        let input = "1=-0-2";
        let snafu = Snafu::from(input);
        assert_eq!(isize::from(snafu), 1747);
    }

    #[test]
    fn isize_to_snafu_simple() {
        let num = -1_isize;
        let result = Snafu::from(num);
        assert_eq!(result.data, vec![Minus]);
    }

    #[test]
    fn isize_to_snafu_complex() {
        let num = -176_isize;
        let result = Snafu::from(num);
        assert_eq!(result.data, vec![Minus, DoubleMinus, Zero, Minus]);
    }
}
