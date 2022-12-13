use std::cmp::Ordering;
use std::time::Instant;

use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair, terminated};
use nom::IResult;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Debug, Eq)]
enum Elem {
    Int(i32),
    List(Vec<Elem>),
}

impl PartialEq for Elem {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Elem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Elem::Int(a), Elem::Int(b)) => a.cmp(b),
            (Elem::List(aa), Elem::List(bb)) => match (aa.first(), bb.first()) {
                (Some(a), Some(b)) => {
                    if a == b {
                        Elem::List(aa[1..].to_vec()).cmp(&Elem::List(bb[1..].to_vec()))
                    } else {
                        a.cmp(b)
                    }
                }
                (None, Some(_)) => Ordering::Less,
                (Some(_), None) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            },
            (aa @ Elem::List(_), b @ Elem::Int(_)) => aa.cmp(&Elem::List(vec![b.clone()])),
            (a @ Elem::Int(_), bb @ Elem::List(_)) => Elem::List(vec![a.clone()]).cmp(bb),
        }
    }
}

fn main() {
    let start_part1 = Instant::now();
    let result_part1 = run_part1(INPUT);
    let elapsed_time_part1 = start_part1.elapsed().as_millis();
    println!("Part 1: {result_part1} in {elapsed_time_part1} ms");

    let start_part2 = Instant::now();
    let result_part2 = run_part2(INPUT);
    let elapsed_time_part2 = start_part2.elapsed().as_millis();
    println!("Part 2: {result_part2} in {elapsed_time_part2} ms")
}

fn run_part1(input: &str) -> usize {
    let packet_pairs = parse(input).unwrap().1;
    packet_pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (a, b))| (a < b).then_some(i + 1))
        .sum()
}

fn run_part2(input: &str) -> usize {
    let packet_pairs = parse(input).unwrap().1;
    let divider_packets = vec![
        Elem::List(vec![Elem::List(vec![Elem::Int(2)])]),
        Elem::List(vec![Elem::List(vec![Elem::Int(6)])]),
    ];
    let mut all_packets: Vec<_> = packet_pairs
        .into_iter()
        .flat_map(|(a, b)| vec![a, b])
        .chain(divider_packets.iter().cloned())
        .collect();
    all_packets.sort();
    all_packets
        .iter()
        .enumerate()
        .filter_map(|(i, p)| divider_packets.contains(p).then_some(i + 1))
        .product()
}

fn parse(input: &str) -> IResult<&str, Vec<(Elem, Elem)>> {
    separated_list0(char('\n'), packet_pair)(input)
}

fn packet_pair(input: &str) -> IResult<&str, (Elem, Elem)> {
    pair(packet, packet)(input)
}

fn packet(input: &str) -> IResult<&str, Elem> {
    terminated(elem, opt(char('\n')))(input)
}

fn elem(input: &str) -> IResult<&str, Elem> {
    alt((
        map(
            delimited(char('['), separated_list0(char(','), elem), char(']')),
            Elem::List,
        ),
        map(nom::character::complete::i32, Elem::Int),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../input_test.txt");

    #[test]
    fn test_elem() {
        let e1 = Elem::Int(123);
        let e2 = Elem::List(vec![Elem::Int(1), Elem::Int(2)]);
        let e3 = Elem::List(vec![Elem::Int(1), Elem::List(vec![Elem::Int(2)])]);

        println!("{:#?}", e1);
        println!("{:#?}", e2);
        println!("{:#?}", e3);
    }

    #[test]
    fn test_parse() {
        let result = parse(INPUT_TEST).unwrap();
        assert_eq!(result.1.len(), 8)
    }

    #[test]
    fn test_input_part1() {
        assert_eq!(run_part1(INPUT_TEST), 13);
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(run_part2(INPUT_TEST), 140);
    }
}
