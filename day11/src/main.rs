use std::{collections::VecDeque, str::FromStr};

use itertools::Itertools;
use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut monkeys = parse_input(INPUT);
    let modulus = monkey_modulus(&monkeys);
    println!("{:#?}", monkeys);
    let result_part1 = run_part1(&mut monkeys, modulus);
    println!("Part 1: {result_part1}");
    let result_part2 = run_part2(&mut monkeys, modulus);
    println!("Part 2: {result_part2}")
}

fn monkey_modulus(monkeys: &Vec<Monkey>) -> Item {
    monkeys.iter().map(|m| m.test_divisible_by).reduce(|acc, x| acc * x).unwrap()
}

fn run_part1(monkeys: &mut Vec<Monkey>, modulus: Item) -> u64 {
    run(monkeys, 20, true, modulus)
}

fn run_part2(monkeys: &mut Vec<Monkey>, modulus: Item) -> u64 {
    run(monkeys, 10000, false, modulus)
}

fn run(monkeys: &mut Vec<Monkey>, turns: usize, divide_by_three: bool, modulus: Item) -> u64 {
    for _turn in 0..turns {
        for i in 0..monkeys.len() {
            // iterate using index to avoid mutable borrowing
            let throws = monkeys[i].inspect_and_throw_items(divide_by_three, modulus);
            for (monkey_id, item) in throws {
                monkeys[monkey_id].catch_item(item);
            }
        }
    }

    calculate_monkey_business_level(monkeys)
}

fn calculate_monkey_business_level(monkeys: &[Monkey]) -> u64 {
    let mut inspection_counts: Vec<_> = monkeys
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect();
    inspection_counts.sort_by(|a, b| b.cmp(a));
    inspection_counts
        .into_iter()
        .take(2)
        .reduce(|acc, cnt| acc * cnt)
        .unwrap()
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
    fn execute(&self, x: Item, modulus: Item) -> Item {
        match self {
            Self::Add(y) => x + y,
            Self::Multiply(y) => mul_mod(x, *y, modulus),
            Self::Square => mul_mod(x, x, modulus),
        }
    }
}

fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    let (a, b, m) = (a as u128, b as u128, m as u128);
    ((a * b) % m) as u64
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
    fn inspect_and_throw_items(&mut self, divide_by_three: bool, modulus: Item) -> Vec<(MonkeyId, Item)> {
        let mut throws = vec![];
        while let Some(item) = self.queue.pop_front() {
            self.inspection_count += 1;
            let item_after_inspection = self.operation.execute(item, modulus);
            let item_after_adjustment = if divide_by_three {
                item_after_inspection / 3
            } else {
                item_after_inspection
            };
            if item_after_adjustment % self.test_divisible_by == 0
            {
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
        .flat_map(|monkey_lines| parse_monkey(&monkey_lines.collect_vec()))
        .collect()
}

// ugly monkey parser
fn parse_monkey(input: &[&str]) -> Option<Monkey> {
    let initial_items = parse_numbers(input[1]);
    let line2 = parse_numbers(input[2]);
    let operation = line2.get(0).map_or(Operation::Square, |&operand| {
        if input[2].contains("+") {
            Operation::Add(operand)
        } else {
            Operation::Multiply(operand)
        }
    });
    let line3 = parse_numbers(input[3]);
    let test = line3.get(0).expect("Division test not found");
    let line4 = parse_numbers(input[4]);
    let if_true = line4.get(0).expect("If true not found");
    let line5 = parse_numbers(input[5]);
    let if_false = line5.get(0).expect("If false not found");

    Some(Monkey {
        queue: VecDeque::from(initial_items),
        inspection_count: 0,
        operation: operation,
        test_divisible_by: *test,
        if_true_throw_to: *if_true,
        if_false_throw_to: *if_false,
    })
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
        let mut monkeys = parse_input(INPUT_TEST);
        let modulus = monkey_modulus(&monkeys);
        eprintln!("{:#?}", monkeys);
        assert_eq!(run_part1(&mut monkeys, modulus), 10605);
    }

    #[test]
    fn test_input_part2() {
        let mut monkeys = parse_input(INPUT_TEST);
        let modulus = monkey_modulus(&monkeys);
        eprintln!("{:#?}", monkeys);
        assert_eq!(run_part2(&mut monkeys, modulus), 2713310158);
    }
}
