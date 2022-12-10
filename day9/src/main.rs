use std::{cmp, collections::HashSet};

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Debug)]
enum Move {
    L,
    R,
    U,
    D,
}

fn main() {
    let moves = parse_moves(INPUT);
    let result_part1 = run_part1(&moves);
    let result_part2 = run_part2(&moves);
    println!("Part 1: {result_part1} - Part 2: {result_part2}")
}

fn run_part1(moves: &Vec<Move>) -> usize {
    run(moves, 2)
}

fn run_part2(moves: &Vec<Move>) -> usize {
    run(moves, 10)
}

fn run(moves: &Vec<Move>, rope_length: usize) -> usize {
    let mut knots: Vec<(i32, i32)> = vec![(0, 0); rope_length];
    let mut unique_positions = HashSet::new();
    unique_positions.insert(*knots.last().unwrap());
    for m in moves {
        match m {
            Move::L => knots[0].0 -= 1,
            Move::R => knots[0].0 += 1,
            Move::U => knots[0].1 -= 1,
            Move::D => knots[0].1 += 1,
        }
        for i in 1..knots.len() {
            knots[i] = move_tail(knots[i], knots[i-1]);
            unique_positions.insert(*knots.last().unwrap());
        }
    }
    unique_positions.len()
}

fn move_tail(pos_tail: (i32, i32), pos_head: (i32, i32)) -> (i32, i32) {
    if cmp::max(
        (pos_tail.0 - pos_head.0).abs(),
        (pos_tail.1 - pos_head.1).abs(),
    ) <= 1
    {
        // head is near tail, no need to move
        pos_tail
    } else {
        // move one step in the direction of head
        (pos_tail.0 + (pos_head.0 - pos_tail.0).signum(), pos_tail.1 + (pos_head.1 - pos_tail.1).signum())
    }
}

fn parse_moves(input: &str) -> Vec<Move> {
    input.lines().flat_map(parse_moves_line).collect()
}

fn parse_moves_line(line: &str) -> Vec<Move> {
    let mut split = line.split(' ');
    let direction = split.next().unwrap();
    let count_str = split.next().unwrap();
    let count = count_str.parse().unwrap();
    vec![parse_move(direction); count]
}

fn parse_move(direction: &str) -> Move {
    match direction {
        "L" => Move::L,
        "R" => Move::R,
        "U" => Move::U,
        "D" => Move::D,
        _ => panic!("Invalid direction"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../input_test.txt");
    const INPUT_TEST_2: &str = include_str!("../input_test_2.txt");

    #[test]
    fn test_input_part1() {
        let moves = parse_moves(INPUT_TEST);
        assert_eq!(run_part1(&moves), 13);
    }

    #[test]
    fn test_input_part2() {
        let moves = parse_moves(INPUT_TEST);
        assert_eq!(run_part2(&moves), 1);
    }

    #[test]
    fn test_input_part2_2() {
        let moves = parse_moves(INPUT_TEST_2);
        assert_eq!(run_part2(&moves), 36);
    }
}
