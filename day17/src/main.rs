use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn max_height(cols: &Vec<Vec<bool>>) -> i64 {
    let first_trues: Vec<_> = cols
        .iter()
        .map(|col| col.iter().rev().position(|p| *p).map_or(0, |y| col.len() - y))
        .collect();

    first_trues.into_iter().max().unwrap() as i64
}

fn run_part1<const N: usize>(input: &str) -> Vec<Vec<bool>> {
    let blocks: Vec<Vec<(i64, i64)>> = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],         // ----
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], // +
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], // _|
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],         // |
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],         // []
    ];

    let mut blocks_fallen = 0;
    let mut jet_ix = 0;
    let mut pos: (i64, i64) = (2, 3);

    let mut cols: Vec<Vec<bool>> = vec![vec![false; N * 4 + 3]; 7]; // worst case height is N * 4

    loop {
        if blocks_fallen == N {
            break;
        }

        let block = &blocks[blocks_fallen % blocks.len()];

        // pushed by jet
        let pos_before_jet = pos;
        let jet = input.as_bytes()[jet_ix % input.len()];
        println!("{}", jet as char);
        let dx = if jet == b'<' { -1 } else { 1 };
        jet_ix += 1;
        pos = (pos.0 + dx, pos.1);

        fn collision(pos: (i64, i64), block: &Vec<(i64, i64)>, cols: &Vec<Vec<bool>>) -> bool {
            pos.0 < 0
                || block.iter().map(|(x, _)| x + pos.0).max().unwrap() > 6
                || block
                    .iter()
                    .map(|(x, y)| (x + pos.0, y + pos.1))
                    .any(|(x, y)| cols[x as usize][y as usize])
        }

        if collision(pos, &block, &cols) {
            println!("Collision");
            pos = pos_before_jet;
        }

        // fall down
        pos = (pos.0, pos.1 - 1);

        fn rock_landed(pos: (i64, i64), block: &Vec<(i64, i64)>, cols: &Vec<Vec<bool>>) -> bool {
            block
                .iter()
                .map(|(x, y)| (x + pos.0, y + pos.1))
                .any(|(x, y)| y < 0 || cols[x as usize][y as usize])
        }

        fn update_columns(pos: (i64, i64), block: &Vec<(i64, i64)>, cols: &mut Vec<Vec<bool>>) {
            for (x, y) in block {
                cols[(x + pos.0) as usize][(y + pos.1 + 1) as usize] = true;
            }
        }

        if rock_landed(pos, &block, &cols) {
            update_columns(pos, block, &mut cols);
            blocks_fallen += 1;
            pos = (2, 3 + max_height(&cols));
            println!("Block {blocks_fallen} fallen");
        }
    }

    cols
}

fn run_part2(_input: &str) -> usize {
    todo!();
}

fn main() {
    let start_part1 = Instant::now();
    let columns_part_1 = run_part1::<2022>(INPUT);
    let result_part1 = max_height(&columns_part_1);
    let elapsed_time_part1 = start_part1.elapsed().as_millis();
    println!("Part 1: {:?} in {elapsed_time_part1} ms", result_part1);

    let start_part2 = Instant::now();
    let result_part2 = run_part2(INPUT);
    let elapsed_time_part2 = start_part2.elapsed().as_millis();
    println!("Part 2: {result_part2} in {elapsed_time_part2} ms")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../input_test.txt");

    #[test]
    fn test_input_part1() {
        assert_eq!(max_height(&run_part1::<2022>(INPUT_TEST)), 3068);
    }

    #[test]
    fn test_input_part1_real() {
        assert!(max_height(&run_part1::<2022>(INPUT)) > 3214);
    }

    #[test]
    fn test_input_part1_2_blocks() {
        let cols = run_part1::<2>(INPUT_TEST);
        print_first_rows(&cols, 8);
        assert_eq!(max_height(&cols), 4);
    }

    #[test]
    fn test_input_part1_3_blocks() {
        let cols = run_part1::<3>(INPUT_TEST);
        print_first_rows(&cols, 8);
        assert_eq!(max_height(&cols), 6);
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(run_part2(INPUT_TEST), 0);
    }

    fn print_first_rows(cols: &Vec<Vec<bool>>, n_rows: usize) {
        for i in 0..n_rows {
            let y = n_rows - 1 - i;
            for col in cols {
                print!("{}", if col[y] { '#' } else { '.' });
            }
            println!();
        }
    }
}
