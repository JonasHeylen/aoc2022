use std::{collections::HashMap, time::Instant};

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
enum Expr {
    Const(i64),
    Add(String, String),
    Mul(String, String),
    Sub(String, String),
    Div(String, String),
}

impl Expr {
    fn eval(&self, context: &HashMap<String, Expr>) -> i64 {
        match self {
            Expr::Const(c) => *c,
            Expr::Add(a, b) => context[a].eval(context) + context[b].eval(context),
            Expr::Mul(a, b) => context[a].eval(context) * context[b].eval(context),
            Expr::Sub(a, b) => context[a].eval(context) - context[b].eval(context),
            Expr::Div(a, b) => context[a].eval(context) / context[b].eval(context),
        }
    }

    fn contains(&self, name: &str, context: &HashMap<String, Expr>) -> bool {
        fn contains_inner(name: &str, context: &HashMap<String, Expr>, a: &str, b: &str) -> bool {
            a == name
                || b == name
                || context[a].contains(name, context)
                || context[b].contains(name, context)
        }
        match self {
            Expr::Const(_) => false,
            Expr::Add(a, b) => contains_inner(name, context, a, b),
            Expr::Mul(a, b) => contains_inner(name, context, a, b),
            Expr::Sub(a, b) => contains_inner(name, context, a, b),
            Expr::Div(a, b) => contains_inner(name, context, a, b),
        }
    }

    fn solve(&self, name: &str, val: i64, context: &HashMap<String, Expr>) -> i64 {
        match self {
            Expr::Const(_) => val,
            Expr::Add(a, b) => {
                if a == name || context[a].contains(name, context) {
                    context[a].solve(name, val - context[b].eval(context), context)
                } else if b == name || context[b].contains(name, context) {
                    context[b].solve(name, val - context[a].eval(context), context)
                } else {
                    panic!("Expression does not contain var to solve for");
                }
            }
            Expr::Mul(a, b) => {
                if a == name || context[a].contains(name, context) {
                    context[a].solve(name, val / context[b].eval(context), context)
                } else if b == name || context[b].contains(name, context) {
                    context[b].solve(name, val / context[a].eval(context), context)
                } else {
                    panic!("Expression does not contain var to solve for");
                }
            }
            Expr::Sub(a, b) => {
                if a == name || context[a].contains(name, context) {
                    context[a].solve(name, val + context[b].eval(context), context)
                } else if b == name || context[b].contains(name, context) {
                    context[b].solve(name, -val + context[a].eval(context), context)
                } else {
                    panic!("Expression does not contain var to solve for");
                }
            }
            Expr::Div(a, b) => {
                if a == name || context[a].contains(name, context) {
                    context[a].solve(name, val * context[b].eval(context), context)
                } else if b == name || context[b].contains(name, context) {
                    context[b].solve(name, context[a].eval(context) / val, context)
                } else {
                    panic!("Expression does not contain var to solve for");
                }
            }
        }
    }
}

fn parse(input: &str) -> HashMap<String, Expr> {
    input
        .lines()
        .flat_map(|l| {
            let (name, expr) = l.split_once(':')?;
            Some((name.to_string(), parse_expr(&expr[1..])))
        })
        .collect()
}

fn parse_expr(expr: &str) -> Expr {
    expr.parse::<i64>().map_or_else(
        |_| {
            let mut split = expr.split(' ');
            let operand1 = split.next().unwrap();
            let operator = split.next().unwrap();
            let operand2 = split.next().unwrap();
            match operator {
                "+" => Expr::Add(operand1.to_string(), operand2.to_string()),
                "-" => Expr::Sub(operand1.to_string(), operand2.to_string()),
                "*" => Expr::Mul(operand1.to_string(), operand2.to_string()),
                "/" => Expr::Div(operand1.to_string(), operand2.to_string()),
                _ => panic!("Invalid operator"),
            }
        },
        Expr::Const,
    )
}

fn run_part1(input: &str) -> i64 {
    let monkeys: HashMap<String, Expr> = parse(input);
    monkeys["root"].eval(&monkeys)
}

fn run_part2(input: &str) -> i64 {
    let monkeys: HashMap<String, Expr> = parse(input);
    if let Expr::Add(a, b) = &monkeys["root"] {
        if monkeys[a].contains("humn", &monkeys) {
            monkeys[a].solve("humn", monkeys[b].eval(&monkeys), &monkeys)
        } else if monkeys[b].contains("humn", &monkeys) {
            monkeys[b].solve("humn", monkeys[a].eval(&monkeys), &monkeys)
        } else {
            panic!("no humn!?")
        }
    } else {
        panic!("root is no Add")
    }
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
    fn test_solve() {
        let x = Expr::Const(123);
        let y = Expr::Const(2);
        let sum = Expr::Add("x".to_string(), "y".to_string());
        let context = HashMap::from([
            ("x".to_string(), x),
            ("y".to_string(), y),
            ("sum".to_string(), sum),
        ]);
        assert_eq!(context["sum"].solve("x", 5, &context), 3);
    }

    #[test]
    fn test_input_part1() {
        assert_eq!(run_part1(INPUT_TEST), 152);
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(run_part2(INPUT_TEST), 301);
    }
}
