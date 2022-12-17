use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn run_part1<const N: usize>(input: &str) -> Vec<i64> {
    let blocks: Vec<Vec<(i64, i64)>> = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],         // ----
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], // +
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], // _|
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],         // |
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],         // []
    ];

    let mut blocks_fallen = 0;
    let mut jet_ix = 0;
    let mut rel_pos: (i64, i64) = (2, 4); // y is relative to max height of columns

    let mut cols: Vec<i64> = vec![0; 7];

    loop {
        if blocks_fallen == N {
            break;
        }

        let block = &blocks[blocks_fallen % blocks.len()];
        let max_height = *cols.iter().max().unwrap();

        // pushed by jet
        let pos_before_jet = rel_pos;
        let jet = input.as_bytes()[jet_ix % input.len()];
        println!("{}", jet as char);
        let dx = if jet == b'<' { -1 } else { 1 };
        jet_ix += 1;
        rel_pos = (rel_pos.0 + dx, rel_pos.1);

        fn collision(
            pos: (i64, i64),
            block: &Vec<(i64, i64)>,
            cols: &Vec<i64>,
            max_height: i64,
        ) -> bool {
            pos.0 < 0
                || block.iter().map(|(x, _)| x + pos.0).max().unwrap() > 6
                || block
                    .iter()
                    .map(|(x, y)| (x + pos.0, y + pos.1 + max_height))
                    .any(|(x, y)| *cols.get(x as usize).unwrap() >= y)
        }

        if collision(rel_pos, &block, &cols, max_height) {
            println!("Collision");
            rel_pos = pos_before_jet;
        }

        // fall down
        rel_pos = (rel_pos.0, rel_pos.1 - 1);

        fn rock_landed(
            pos: (i64, i64),
            block: &Vec<(i64, i64)>,
            cols: &Vec<i64>,
            max_height: i64,
        ) -> bool {
            block
                .iter()
                .map(|(x, y)| (x + pos.0, y + pos.1 + max_height))
                .any(|(x, y)| *cols.get(x as usize).unwrap() >= y)
        }

        fn update_columns(
            pos: (i64, i64),
            block: &Vec<(i64, i64)>,
            cols: &mut Vec<i64>,
            max_height: i64,
        ) {
            for (x, y) in block {
                if let Some(max_y) = cols.get_mut((x + pos.0) as usize) {
                    let y = y + pos.1 + 1 + max_height;
                    if y > *max_y {
                        *max_y = y;
                    }
                }
            }
        }

        if rock_landed(rel_pos, &block, &cols, max_height) {
            update_columns(rel_pos, block, &mut cols, max_height);
            blocks_fallen += 1;
            rel_pos = (2, 4);
            println!("Block {blocks_fallen} fallen\nCols: {:?}", cols);
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
    let result_part1 = columns_part_1.iter().max().unwrap();
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
        assert_eq!(*run_part1::<2022>(INPUT_TEST).iter().max().unwrap(), 3068);
    }

    #[test]
    fn test_input_part1_real() {
        assert!(*run_part1::<2022>(INPUT).iter().max().unwrap() > 3214);
    }

    #[test]
    fn test_input_part1_2_blocks() {
        assert_eq!(run_part1::<2>(INPUT_TEST), vec![0, 0, 3, 4, 3, 1, 0]);
    }

    #[test]
    fn test_input_part1_3_blocks() {
        assert_eq!(run_part1::<3>(INPUT_TEST), vec![4, 4, 6, 4, 3, 1, 0]);
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(run_part2(INPUT_TEST), 0);
    }
}
