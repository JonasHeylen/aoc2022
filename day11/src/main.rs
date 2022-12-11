use std::{collections::VecDeque, str::FromStr};

use itertools::Itertools;
use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let result_part1 = run_part1(INPUT);
    println!("Part 1: {result_part1}");
    let result_part2 = run_part2(INPUT);
    println!("Part 2: {result_part2}")
}

fn monkey_modulus(monkeys: &[Monkey]) -> Item {
    monkeys.iter().map(|m| m.test_divisible_by).product()
}

fn run_part1(input: &str) -> u64 {
    run(input, 20, true)
}

fn run_part2(input: &str) -> u64 {
    run(input, 10000, false)
}

fn run(input: &str, rounds: usize, divide_by_three: bool) -> u64 {
    let mut monkeys = parse_input(input);
    let modulus = monkey_modulus(&monkeys);
    for _round in 0..rounds {
        for i in 0..monkeys.len() {
            // iterate using index to avoid mutable borrowing
            let throws = monkeys[i].inspect_and_throw_items(divide_by_three, modulus);
            for (monkey_id, item) in throws {
                monkeys[monkey_id].catch_item(item);
            }
        }

        // if (_round + 1) % 1000 == 0 {
        //     eprintln!(
        //         "After round {}\n{}\n",
        //         _round + 1,
        //         monkeys
        //             .iter()
        //             .enumerate()
        //             .map(|(i, m)| format!(
        //                 "Monkey {} inspected {} items and holds {:?}",
        //                 i, m.inspection_count, m.queue
        //             ))
        //             .join("\n")
        //     )
        // }
    }

    // println!("{:#?}", monkeys);
    calculate_monkey_business_level(&monkeys)
}

fn calculate_monkey_business_level(monkeys: &[Monkey]) -> u64 {
    let mut inspection_counts: Vec<_> = monkeys
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect();
    inspection_counts.sort_by(|a, b| b.cmp(a));
    inspection_counts.into_iter().take(2).product()
}

type Item = u64;
type MonkeyId = usize;

#[derive(Clone, Debug)]
enum Operation {
    Add(Item),
    Multiply(Item),
    Square,
}

impl Operation {
    fn execute(&self, x: Item) -> Item {
        match self {
            Self::Add(y) => x + y,
            Self::Multiply(y) => x * y,
            Self::Square => x * x,
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    queue: VecDeque<Item>,
    inspection_count: u64,
    operation: Operation,
    test_divisible_by: Item,
    if_true_throw_to: MonkeyId,
    if_false_throw_to: MonkeyId,
}

impl Monkey {
    fn inspect_and_throw_items(
        &mut self,
        divide_by_three: bool,
        modulus: Item,
    ) -> Vec<(MonkeyId, Item)> {
        let mut throws = vec![];
        while let Some(item) = self.queue.pop_front() {
            self.inspection_count += 1;
            let item_after_inspection = self.operation.execute(item);
            let item_after_adjustment = if divide_by_three {
                item_after_inspection / 3
            } else {
                item_after_inspection % modulus
            };
            if item_after_adjustment % self.test_divisible_by == 0 {
                throws.push((self.if_true_throw_to, item_after_adjustment));
            } else {
                throws.push((self.if_false_throw_to, item_after_adjustment));
            }
        }
        throws
    }

    fn catch_item(&mut self, item: Item) {
        self.queue.push_back(item);
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .lines()
        .chunks(7)
        .into_iter()
        .map(|monkey_lines| parse_monkey(&monkey_lines.collect_vec()))
        .collect()
}

// ugly monkey parser
fn parse_monkey(input: &[&str]) -> Monkey {
    let initial_items = parse_numbers(input[1]);
    let line2 = parse_numbers(input[2]);
    let operation = line2.first().map_or(Operation::Square, |&operand| {
        if input[2].contains('+') {
            Operation::Add(operand)
        } else {
            Operation::Multiply(operand)
        }
    });
    let line3 = parse_numbers(input[3]);
    let test_divisible_by = *line3.first().expect("Division test not found");
    let line4 = parse_numbers(input[4]);
    let if_true_throw_to = *line4.first().expect("If true not found");
    let line5 = parse_numbers(input[5]);
    let if_false_throw_to = *line5.first().expect("If false not found");

    Monkey {
        queue: VecDeque::from(initial_items),
        inspection_count: 0,
        operation,
        test_divisible_by,
        if_true_throw_to,
        if_false_throw_to,
    }
}

fn parse_numbers<T: FromStr>(input: &str) -> Vec<T> {
    let numbers_re: Regex = Regex::new(r"\d+").unwrap();
    numbers_re
        .captures_iter(input)
        .flat_map(|capture| capture.get(0).and_then(|c| c.as_str().parse::<T>().ok()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../input_test.txt");

    #[test]
    fn test_parse_numers() {
        let input = "  Starting items: 79, 98";
        let numbers = parse_numbers::<i32>(input);
        assert_eq!(numbers, vec![79, 98])
    }

    #[test]
    fn test_input_part1() {
        assert_eq!(run_part1(INPUT_TEST), 10605);
    }

    #[test]
    fn test_real_input_part1() {
        assert_eq!(run_part1(INPUT), 98280);
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(run_part2(INPUT_TEST), 2713310158);
    }
}
