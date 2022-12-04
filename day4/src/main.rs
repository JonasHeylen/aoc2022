use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

type Assignment = (u32, u32);

fn main() {
    let result_part1 = run_part1("input.txt").unwrap();
    let result_part2 = run_part2("input.txt").unwrap();
    println!("Part 1: {result_part1} - Part 2: {result_part2}");
}

fn run_part1(filename: &str) -> io::Result<usize> {
    run(filename, is_contained_in_or_contains)
}

fn run_part2(filename: &str) -> io::Result<usize> {
    run(filename, overlaps)
}

fn run(filename: &str, check: impl Fn(Assignment, Assignment) -> bool) -> io::Result<usize> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let count = reader
        .lines()
        .flatten()
        .map(|line| parse_line(&line))
        .filter(|(a1, a2)| check(*a1, *a2))
        .count();

    Ok(count)
}

fn parse_line(s: &str) -> (Assignment, Assignment) {
    let mut split = s.split(',').map(parse_assignment);
    (split.next().unwrap(), split.next().unwrap())
}

fn parse_assignment(s: &str) -> Assignment {
    let mut split = s.split('-').flat_map(|s| s.parse::<u32>());
    (split.next().unwrap(), split.next().unwrap())
}

fn is_contained_in_or_contains(a: Assignment, b: Assignment) -> bool {
    is_contained_in(a, b) || is_contained_in(b, a)
}

fn is_contained_in(a: Assignment, b: Assignment) -> bool {
    a.0 >= b.0 && a.1 <= b.1
}

fn overlaps(a: Assignment, b: Assignment) -> bool {
    (a.0..=a.1).any(|i| (b.0..=b.1).contains(&i))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() {
        assert_eq!(run_part1("input_test.txt").unwrap(), 2);
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(run_part2("input_test.txt").unwrap(), 4);
    }
}
