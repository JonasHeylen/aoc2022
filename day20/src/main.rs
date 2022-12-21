use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn mix(numbers: &mut Vec<(usize, i64)>) -> &Vec<(usize, i64)> {
    for i in 0..numbers.len() {
        let ix = numbers
            .iter()
            .position(|(orig_ix, _)| *orig_ix == i)
            .unwrap();
        let elem = numbers.remove(ix);
        let len_i64 = numbers.len() as i64;
        numbers.insert(
            ((((ix as i64 + elem.1) % len_i64) + len_i64) % len_i64) as usize,
            elem,
        );
    }
    numbers
}

fn run_part1(input: &str) -> i64 {
    let mut numbers: Vec<_> = input
        .lines()
        .flat_map(|l| l.parse::<i64>().ok())
        .enumerate()
        .collect();

    mix(&mut numbers);

    let start_ix = numbers.iter().position(|(_, num)| *num == 0).unwrap();
    numbers[(start_ix + 1000) % numbers.len()].1
        + numbers[(start_ix + 2000) % numbers.len()].1
        + numbers[(start_ix + 3000) % numbers.len()].1
}

fn run_part2(input: &str) -> i64 {
    let mut numbers: Vec<_> = input
        .lines()
        .flat_map(|l| l.parse::<i64>().ok())
        .map(|n| n * 811589153)
        .enumerate()
        .collect();

    for _ in 0..10 {
        mix(&mut numbers);
    }

    let start_ix = numbers.iter().position(|(_, num)| *num == 0).unwrap();
    numbers[(start_ix + 1000) % numbers.len()].1
        + numbers[(start_ix + 2000) % numbers.len()].1
        + numbers[(start_ix + 3000) % numbers.len()].1
}

fn main() {
    let start_part1 = Instant::now();
    let result_part1 = run_part1(INPUT);
    let elapsed_time_part1 = start_part1.elapsed().as_micros();
    println!("Part 1: {:?} in {elapsed_time_part1} µs", result_part1);

    let start_part2 = Instant::now();
    let result_part2 = run_part2(INPUT);
    let elapsed_time_part2 = start_part2.elapsed().as_micros();
    println!("Part 2: {result_part2} in {elapsed_time_part2} µs")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../input_test.txt");

    #[test]
    fn test_input_part1() {
        assert_eq!(run_part1(INPUT_TEST), 3);
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(run_part2(INPUT_TEST), 1623178306);
    }
}
