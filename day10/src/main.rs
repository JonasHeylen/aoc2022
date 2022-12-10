const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

#[derive(Clone, Copy, Debug)]
struct State {
    x: i32,
}

impl State {
    fn init() -> Self {
        Self { x: 1 }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> Vec<State> {
        match instruction {
            Instruction::Noop => vec![*self],
            Instruction::AddX(v) => {
                let out = vec![*self; 2];
                self.x += v;
                out
            }
        }
    }
}

fn main() {
    let instructions = parse_instructions(INPUT);
    let result_part1 = sum_signal_strengths(&instructions);
    println!("Part 1: {result_part1}");
    let result_part2 = draw_display(&instructions);
    println!("Part 2: {result_part2}")
}

fn sum_signal_strengths(instructions: &[Instruction]) -> i32 {
    let states = run_instructions(instructions);

    states
        .iter()
        .map(|s| s.x)
        .enumerate()
        .skip(19)
        .step_by(40)
        .take(6)
        .map(|(i, x)| (i as i32 + 1) * x)
        .sum()
}

fn draw_display(instructions: &[Instruction]) -> i32 {
    let states = run_instructions(instructions);
    println!("{}", states.len());

    let display_columns = 40usize;

    let display: String = states
        .iter()
        .map(|s| s.x)
        .enumerate()
        .map(|(cycle, x)| {
            if ((cycle % display_columns) as i32 - x).abs() <= 1 {
                '#'
            } else {
                ' '
            }
        })
        .collect();

    display
        .as_bytes()
        .chunks(40)
        .for_each(|row| println!("{}", String::from_utf8(row.to_vec()).unwrap()));

    0
}

fn run_instructions(instructions: &[Instruction]) -> Vec<State> {
    instructions
        .iter()
        .scan(State::init(), |state, instruction| {
            Some(state.execute_instruction(instruction))
        })
        .flatten()
        .collect()
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.lines().map(parse_moves_line).collect()
}

fn parse_moves_line(line: &str) -> Instruction {
    let mut iter = line.split(' ');
    let opcode = iter.next();
    let operand = iter.next();
    match opcode {
        Some("noop") => Instruction::Noop,
        Some("addx") => {
            if let Some(v) = operand {
                Instruction::AddX(v.parse().expect("AddX: Invalid number"))
            } else {
                panic!("AddX: Missing operand")
            }
        }
        _ => panic!("Invalid instruction"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../input_test.txt");

    #[test]
    fn test_input_part1() {
        let instructions = parse_instructions(INPUT_TEST);
        assert_eq!(sum_signal_strengths(&instructions), 13140);
    }
}
