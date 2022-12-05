use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Instruction {
    num_crates: usize,
    from: usize,
    to: usize,
}

type Stack = Vec<char>;
type Stacks = Vec<Stack>;

fn main() {
    let result_part1 = run_part1("input.txt").unwrap();
    let result_part2 = run_part2("input.txt").unwrap();
    println!("Part 1: {result_part1} - Part 2: {result_part2}");
}

fn run_part1(filename: &str) -> io::Result<String> {
    run(filename, execute_instruction)
}

fn run_part2(filename: &str) -> io::Result<String> {
    run(filename, execute_instruction_part2)
}

fn run(filename: &str, f: impl Fn(Instruction, &mut Stacks)) -> io::Result<String> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let lines = reader.lines().flatten();

    let initial_stacks_t: Stacks = lines
        .take_while(|s| !s.is_empty())
        .map(|s| {
            s.chars()
                .skip(1)
                .enumerate()
                .filter_map(|(i, c)| if i % 4 == 0 { Some(c) } else { None })
                .collect::<Stack>()
        })
        .collect();

    let mut stacks = transpose_stacks(initial_stacks_t);
    println!("Initial stacks: {:?}", stacks);

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let instructions: Vec<Instruction> = reader
        .lines()
        .flatten()
        .flat_map(|s| parse_instruction(&s))
        .collect();
    println!("Instructions: {:?}", instructions);

    execute(instructions, &mut stacks, f);

    let result = stacks
        .iter_mut()
        .map(|s| s.pop().unwrap())
        .collect::<String>();

    Ok(result)
}

fn execute(
    instructions: Vec<Instruction>,
    stacks: &mut Stacks,
    f: impl Fn(Instruction, &mut Stacks),
) {
    for instruction in instructions {
        f(instruction, stacks);
    }
}

fn execute_instruction(instruction: Instruction, stacks: &mut Stacks) {
    for _ in 0..instruction.num_crates {
        let item = stacks[instruction.from].pop().unwrap();
        stacks[instruction.to].push(item);
    }
}

fn execute_instruction_part2(instruction: Instruction, stacks: &mut Stacks) {
    let from_stack = &mut stacks[instruction.from];
    let items_to_move = from_stack.split_off(from_stack.len() - instruction.num_crates);
    println!("{:?}", instruction);
    println!("Items to move: {:?}", items_to_move);
    stacks[instruction.to].extend(items_to_move);
    println!("{:?}", stacks);
}

fn parse_instruction(instruction: &str) -> Option<Instruction> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }
    RE.captures_iter(instruction)
        .next()
        .map(|captures| Instruction {
            num_crates: captures[1].parse().unwrap(),
            from: captures[2].parse::<usize>().unwrap() - 1,
            to: captures[3].parse::<usize>().unwrap() - 1,
        })
}

fn transpose_stacks(stacks_t: Stacks) -> Stacks {
    let num_stacks = stacks_t[0].len();
    (0..num_stacks)
        .map(|i| {
            stacks_t
                .iter()
                .rev()
                .skip(1)
                .filter_map(|inner| {
                    let c = inner[i];
                    if c != ' ' {
                        Some(c)
                    } else {
                        None
                    }
                })
                .collect::<Stack>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() {
        assert_eq!(run_part1("input_test.txt").unwrap(), "CMZ");
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(run_part2("input_test.txt").unwrap(), "MCD");
    }
}
