use std::{fmt::{Display}, cmp::Ordering};
use std::io;
use std::io::Read;

const DIVIDERS: [&str; 2] = ["[[2]]", "[[6]]"];

#[derive(Debug, PartialEq, Eq, Ord)]
enum Packet {
    List(Vec<Box<Packet>>),
    Int(usize),
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::List(v) => f.write_fmt(
                format_args!(
                    "[{}]",
                    v.iter().map(
                        |x| format!("{}", x)
                    ).collect::<Vec<String>>()
                    .join(",")
                )
            ),
            Packet::Int(n) => f.write_fmt(format_args!("{}", n)),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Int(m), Packet::Int(n)) => m.partial_cmp(n),
            (Packet::List(v1), Packet::List(v2)) => {
                for (i, m) in v1.iter().enumerate() {
                    if v2.len() <= i {
                        // RHS ran out of items
                        return Some(Ordering::Greater);
                    }
                    match m.partial_cmp(&v2[i]) {
                        Some(Ordering::Less) => {
                            return Some(Ordering::Less);
                        },
                        Some(Ordering::Greater) => {
                            return Some(Ordering::Greater);
                        },
                        _ => {},
                    }
                }
                if v1.len() == v2.len() {
                    return Some(Ordering::Equal);
                }
                // LHS ran out of items
                return Some(Ordering::Less);
            },
            (Packet::Int(m), Packet::List(_)) => {
                return Packet::List(vec![Box::new(Packet::Int(*m))]).partial_cmp(other);
            },
            (Packet::List(_), Packet::Int(n)) => {
                return self.partial_cmp(&Packet::List(vec![Box::new(Packet::Int(*n))]));
            }
        }
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("Error reading stdin.");
    let mut pairs = buf.lines()
    .collect::<Vec<&str>>()
    .chunks(3)
    .map(|pair| (parse(&mut pair[0].chars().skip(1)), parse(&mut pair[1].chars().skip(1))))
    .map(|(s1, s2)| { let c = s1.partial_cmp(&s2); (s1, s2, c) })
    .collect::<Vec<(Packet, Packet, Option<Ordering>)>>();
    let indices = pairs.iter().enumerate().filter(|(_, (_, _, b))| b.unwrap().is_le())
    .map(|(i, _)| i + 1)
    .collect::<Vec<usize>>();
    let indices_sum = indices.iter().sum::<usize>();
    println!("{:?}", indices_sum);
    let mut packets = pairs.iter_mut()
    .flat_map(|(p1, p2, _)| vec![p1, p2]).collect::<Vec<&mut Packet>>();
    let mut dividers = DIVIDERS.iter().map(|d| parse(&mut d.chars().skip(1))).collect::<Vec<Packet>>();
    packets.append(&mut dividers.iter_mut().collect::<Vec<&mut Packet>>());
    packets.sort_unstable();
    let decoder_key = packets.iter().enumerate()
        .filter(|(_, p)| DIVIDERS.map(|s| s.to_string()).contains(&format!("{}",p)))
        .map(|(i, _)| i + 1)
        .product::<usize>();
    println!("Decoder key: {}", decoder_key);
}

fn parse(input: &mut dyn Iterator<Item = char>) -> Packet {
    let mut ret = vec![];
    let mut current = String::new();
    while let Some(c) = input.next() {
        match c {
            '[' => {
                ret.push(Box::new(parse(input)));
            },
            '0'..='9' => {
                current.push(c);
            },
            ',' => {
                if current.chars().count() > 0 {
                    ret.push(Box::new(Packet::Int(current.parse::<usize>().unwrap())));
                    current = String::new();
                }
            },
            ']' => {
                if current.chars().count() > 0 {
                    ret.push(Box::new(Packet::Int(current.parse::<usize>().unwrap())));
                }
                return Packet::List(ret);
            },
            _ => unreachable!(),
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let input = Packet::List(vec![
            Box::new(Packet::Int(1)),
            Box::new(Packet::List(vec![
                Box::new(Packet::Int(2)),
                Box::new(Packet::Int(3)),
            ])),
        ]);
        assert_eq!(format!("{}", input), "[1,[2,3]]");
    }

    #[test]
    fn parse_test() {
        let input = "[1,[2,3]]";
        let packet = parse(&mut input.chars().skip(1));
        let expected = Packet::List(vec![
            Box::new(Packet::Int(1)),
            Box::new(Packet::List(vec![
                Box::new(Packet::Int(2)),
                Box::new(Packet::Int(3)),
            ])),
        ]);
        assert_eq!(packet, expected);
    }

    #[test]
    fn pair1() {
        let one = "[1,1,3,1,1]";
        let two = "[1,1,5,1,1]";
        assert!(parse(&mut one.chars().skip(1)) < parse(&mut two.chars().skip(1)));
    }

    #[test]
    fn pair2() {
        let one = "[[1],[2,3,4]]";
        let two = "[[1],4]";
        assert!(parse(&mut one.chars().skip(1)) < parse(&mut two.chars().skip(1)));
    }

    #[test]
    fn pair3() {
        let one = "[9]";
        let two = "[[8,7,6]]";
        assert!(!(parse(&mut one.chars().skip(1)) < parse(&mut two.chars().skip(1))));
    }

    #[test]
    fn pair4() {
        let one = "[[4,4],4,4]";
        let two = "[[4,4],4,4,4]";
        assert!(parse(&mut one.chars().skip(1)) < parse(&mut two.chars().skip(1)));
    }

    #[test]
    fn pair5() {
        let one = "[7,7,7,7]";
        let two = "[7,7,7]";
        assert!(!(parse(&mut one.chars().skip(1)) < parse(&mut two.chars().skip(1))));
    }

    #[test]
    fn pair6() {
        let one = "[]";
        let two = "[3]";
        assert!(parse(&mut one.chars().skip(1)) < parse(&mut two.chars().skip(1)));
    }

    #[test]
    fn pair7() {
        let one = "[[[]]]";
        let two = "[[]]";
        assert!(!(parse(&mut one.chars().skip(1)) < parse(&mut two.chars().skip(1))));
    }

    #[test]
    fn pair8() {
        let one = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
        let two = "[1,[2,[3,[4,[5,6,0]]]],8,9]";
        assert!(!(parse(&mut one.chars().skip(1)) < parse(&mut two.chars().skip(1))));
    }

    #[test]
    fn equality() {
        let one = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
        let two = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
        assert!(!(parse(&mut one.chars().skip(1)) < parse(&mut two.chars().skip(1))));
    }

    #[test]
    fn faux_equality() {
        let one = "[1,[2],3]";
        let two = "[1,2,1]";
        assert!(!(parse(&mut one.chars().skip(1)) < parse(&mut two.chars().skip(1))));
    }
}