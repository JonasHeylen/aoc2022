use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Lines},
    str::FromStr,
};
use itertools::Itertools;

struct Rucksack {
    compartment1: HashSet<char>,
    compartment2: HashSet<char>,
}

impl Rucksack {
    fn new(compartment1: HashSet<char>, compartment2: HashSet<char>) -> Self {
        Self {
            compartment1,
            compartment2,
        }
    }
}

impl Rucksack {
    fn duplicate_item(&self) -> &char {
        self.compartment1
            .intersection(&self.compartment2)
            .next()
            .unwrap()
    }

    fn all_items(&self) -> HashSet<char> {
        self.compartment1.union(&self.compartment2).cloned().collect()
    }
}

impl FromStr for Rucksack {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mid = s.len() / 2;
        let c1 = &s[..mid];
        let c2 = &s[mid..];
        let c1_str = String::from(c1);
        let c2_str = String::from(c2);
        let c1_chars = c1_str.chars();
        let c2_chars = c2_str.chars();
        Ok(Rucksack::new(
            HashSet::from_iter(c1_chars),
            HashSet::from_iter(c2_chars),
        ))
    }
}

fn priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        (c as u8 - 96) as u32
    } else if c.is_ascii_uppercase() {
        (c as u8 - 38) as u32
    } else {
        panic!("Invalid character: {}", c)
    }
}

fn main() {
    let result = run("input.txt");
    let result2 = run_part2("input.txt");
    println!("Result: {result} - Result part 2: {result2}");
}

fn run(filename: &str) -> u32 {
    read_file(filename)
        .filter_map(|res| Some(res.ok()?))
        .map(|s| {
            priority(
                *Rucksack::from_str(&s)
                    .expect("Invalid rucksack")
                    .duplicate_item(),
            )
        })
        .sum()
}

fn run_part2(filename: &str) -> u32 {
    read_file(filename)
        .filter_map(|res| Some(res.ok()?))
        .tuples()
        .map(|(l1, l2, l3)| {
            let r1 = Rucksack::from_str(&l1).expect("Invalid rucksack").all_items();
            let r2 = Rucksack::from_str(&l2).expect("Invalid rucksack").all_items();
            let r3 = Rucksack::from_str(&l3).expect("Invalid rucksack").all_items();
            let intersection1and2 = r1
                .intersection(&r2)
                .cloned()
                .collect::<HashSet<_>>();
            let mut intersection = intersection1and2.intersection(&r3);
            let common_item = intersection.next().unwrap();
            let prio = priority(*common_item);
            println!("Common item: {}, priority: {}", common_item, prio);
            prio
        })
        .sum()
}

fn read_file(filename: &str) -> Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priorty() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('b'), 2);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }

    #[test]
    fn test_input() {
        assert_eq!(run("input_test.txt"), 157);
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(run_part2("input_test.txt"), 70);
    }
}
