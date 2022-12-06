use std::{
    collections::{hash_map::RandomState, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() {
    let result_part1 = run_part1("input.txt").unwrap();
    let result_part2 = run_part2("input.txt").unwrap();
    println!("Part 1: {result_part1} - Part 2: {result_part2}");
}

fn run_part1(filename: &str) -> io::Result<usize> {
    run(filename, 4)
}

fn run_part2(filename: &str) -> io::Result<usize> {
    run(filename, 14)
}

fn run(filename: &str, marker_length: usize) -> io::Result<usize> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let line = reader
        .lines()
        .flatten()
        .next()
        .expect("No line found in input file.");

    let offset = line
        .as_bytes()
        .windows(marker_length)
        .enumerate()
        .filter_map(|(i, window)| {
            let set: HashSet<&u8, RandomState> = HashSet::from_iter(window);
            if set.len() == marker_length {
                Some(i + marker_length)
            } else {
                None
            }
        })
        .next()
        .expect("No marker found.");

    Ok(offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1_part1() {
        assert_eq!(run_part1("input_test_1.txt").unwrap(), 7);
    }

    #[test]
    fn test_input_2_part1() {
        assert_eq!(run_part1("input_test_2.txt").unwrap(), 5);
    }

    #[test]
    fn test_input_3_part1() {
        assert_eq!(run_part1("input_test_3.txt").unwrap(), 6);
    }

    #[test]
    fn test_input_4_part1() {
        assert_eq!(run_part1("input_test_4.txt").unwrap(), 10);
    }

    #[test]
    fn test_input_5_part1() {
        assert_eq!(run_part1("input_test_5.txt").unwrap(), 11);
    }

    #[test]
    fn test_input_1_part2() {
        assert_eq!(run_part2("input_test_1.txt").unwrap(), 19);
    }

    #[test]
    fn test_input_2_part2() {
        assert_eq!(run_part2("input_test_2.txt").unwrap(), 23);
    }

    #[test]
    fn test_input_3_part2() {
        assert_eq!(run_part2("input_test_3.txt").unwrap(), 23);
    }

    #[test]
    fn test_input_4_part2() {
        assert_eq!(run_part2("input_test_4.txt").unwrap(), 29);
    }

    #[test]
    fn test_input_5_part2() {
        assert_eq!(run_part2("input_test_5.txt").unwrap(), 26);
    }
}
