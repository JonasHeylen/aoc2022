const INPUT: &[u8] = include_bytes!("../input.txt");

fn main() {
    let grid = Grid::from_bytes(INPUT);
    let result_part1 = run_part1(&grid);
    let result_part2 = run_part2(&grid);
    println!("Part 1: {result_part1} - Part 2: {result_part2}")
}

fn run_part1(grid: &Grid) -> usize {
    let visible_trees = grid.visible_trees();
    count_trues(visible_trees.iter().flatten())
}

fn run_part2(grid: &Grid) -> usize {
    let scenic_scores = grid.scenic_score();
    *scenic_scores.iter().flatten().max().unwrap()
}

struct Grid<'a> {
    grid: &'a [u8],
    num_rows: usize,
    num_cols: usize,
}

impl<'a> Grid<'a> {
    fn from_bytes(input: &'a [u8]) -> Self {
        let num_cols = input.iter().position(|&b| b == b'\n').unwrap();
        let num_rows = input.len() / num_cols;
        Self {
            grid: input,
            num_rows,
            num_cols,
        }
    }

    fn iterate_row(&self, row_ix: usize) -> impl DoubleEndedIterator<Item = &u8> {
        self.grid
            .iter()
            .skip(row_ix * (self.num_cols + 1))
            .step_by(1)
            .take(self.num_cols)
    }

    fn iterate_row_reverse(&self, row_ix: usize) -> impl Iterator<Item = &u8> {
        self.iterate_row(row_ix).rev()
    }

    fn iterate_col(&self, col_ix: usize) -> impl DoubleEndedIterator<Item = &u8> {
        self.grid.iter().skip(col_ix).step_by(self.num_cols + 1)
    }

    fn iterate_col_reverse(&self, col_ix: usize) -> impl Iterator<Item = &u8> {
        self.iterate_col(col_ix).rev()
    }

    fn visible_trees_for_row(&self, row_ix: usize) -> Vec<bool> {
        let visible_from_left = visible_trees_from_start(self.iterate_row(row_ix));
        let mut visible_from_right: Vec<_> =
            visible_trees_from_start(self.iterate_row_reverse(row_ix));
        visible_from_right.reverse();
        vec_or(visible_from_left, visible_from_right)
    }

    fn visible_trees_for_col(&self, col_ix: usize) -> Vec<bool> {
        let visible_from_top = visible_trees_from_start(self.iterate_col(col_ix));
        let mut visible_from_bottom: Vec<_> =
            visible_trees_from_start(self.iterate_col_reverse(col_ix));
        visible_from_bottom.reverse();
        vec_or(visible_from_top, visible_from_bottom)
    }

    fn visible_trees_all_rows(&self) -> Vec<Vec<bool>> {
        let mut result = Vec::with_capacity(self.num_rows);
        for row in 0..self.num_rows {
            result.push(self.visible_trees_for_row(row));
        }
        result
    }

    fn visible_trees_all_cols(&self) -> Vec<Vec<bool>> {
        let mut result = Vec::with_capacity(self.num_cols);
        for col in 0..self.num_cols {
            result.push(self.visible_trees_for_col(col));
        }
        result
    }

    fn visible_trees(&self) -> Vec<Vec<bool>> {
        let visible_by_row = self.visible_trees_all_rows();
        let visible_by_col = self.visible_trees_all_cols();
        let mut result = Vec::with_capacity(self.num_rows);
        for row in 0..self.num_rows {
            let mut result_row = Vec::with_capacity(self.num_cols);
            for col in 0..self.num_cols {
                result_row.push(visible_by_row[row][col] || visible_by_col[col][row]);
            }
            result.push(result_row);
        }
        result
    }

    fn scenic_score(&self) -> Vec<Vec<usize>> {
        let mut result = Vec::with_capacity(self.num_rows);
        for row in 1..self.num_rows - 1 {
            let mut result_row = Vec::with_capacity(self.num_cols);
            for col in 1..self.num_cols - 1 {
                result_row.push(self.scenic_score_for_tree(row, col));
            }
            result.push(result_row);
        }
        result
    }

    fn height(&self, row_ix: usize, col_ix: usize) -> u8 {
        self.grid[row_ix * (self.num_cols + 1) + col_ix]
    }

    fn scenic_score_for_tree(&self, row_ix: usize, col_ix: usize) -> usize {
        self.scenic_score_left(row_ix, col_ix)
            * self.scenic_score_right(row_ix, col_ix)
            * self.scenic_score_top(row_ix, col_ix)
            * self.scenic_score_bottom(row_ix, col_ix)
    }

    fn scenic_score_left(&self, row_ix: usize, col_ix: usize) -> usize {
        let height = self.height(row_ix, col_ix);
        self.iterate_row_reverse(row_ix)
            .skip(self.num_cols - col_ix)
            .position(|&x| x >= height)
            .map(|x| x + 1)
            .unwrap_or(col_ix)
    }

    fn scenic_score_right(&self, row_ix: usize, col_ix: usize) -> usize {
        let height = self.height(row_ix, col_ix);
        self.iterate_row(row_ix)
            .skip(col_ix + 1)
            .position(|&x| x >= height)
            .map(|x| x + 1)
            .unwrap_or(self.num_cols - col_ix - 1)
    }

    fn scenic_score_top(&self, row_ix: usize, col_ix: usize) -> usize {
        let height = self.height(row_ix, col_ix);
        self.iterate_col_reverse(col_ix)
            .skip(self.num_rows - row_ix)
            .position(|&x| x >= height)
            .map(|x| x + 1)
            .unwrap_or(row_ix)
    }

    fn scenic_score_bottom(&self, row_ix: usize, col_ix: usize) -> usize {
        let height = self.height(row_ix, col_ix);
        self.iterate_col(col_ix)
            .skip(row_ix + 1)
            .position(|&x| x >= height)
            .map(|x| x + 1)
            .unwrap_or(self.num_rows - row_ix - 1)
    }
}

fn visible_trees_from_start<'a, T>(r: T) -> Vec<bool>
where
    T: Iterator<Item = &'a u8>,
{
    let mut max_height = 0;
    let mut visible_trees = Vec::new();
    for &height in r {
        if height > max_height {
            visible_trees.push(true);
            max_height = height;
        } else {
            visible_trees.push(false);
        }
    }
    visible_trees
}

fn vec_or(a: Vec<bool>, b: Vec<bool>) -> Vec<bool> {
    a.iter().zip(b.iter()).map(|(&l, &r)| l || r).collect()
}

fn count_trues<'a>(bools: impl Iterator<Item = &'a bool>) -> usize {
    bools.filter(|&x| *x).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &[u8] = include_bytes!("../input_test.txt");

    #[test]
    fn test_visible_trees_from_start() {
        let visible_trees = visible_trees_from_start(vec![3, 0, 3, 7, 3].iter());
        println!("{:#?}", visible_trees);
        assert_eq!(visible_trees, vec![true, false, false, true, false]);
    }

    #[test]
    fn test_grid_from_bytes() {
        let grid = Grid::from_bytes(INPUT_TEST);
        assert_eq!(grid.num_cols, 5);
        assert_eq!(grid.num_rows, 5);
    }

    #[test]
    fn test_iterate_row() {
        let grid = Grid::from_bytes(INPUT_TEST);
        let row2: Vec<_> = grid.iterate_row(1).collect();
        assert_eq!(row2, vec![&b'2', &b'5', &b'5', &b'1', &b'2'])
    }

    #[test]
    fn test_iterate_row_reverse() {
        let grid = Grid::from_bytes(INPUT_TEST);
        let row3_reverse: Vec<_> = grid.iterate_row_reverse(2).collect();
        assert_eq!(row3_reverse, vec![&b'2', &b'3', &b'3', &b'5', &b'6'])
    }

    #[test]
    fn test_iterate_col() {
        let grid = Grid::from_bytes(INPUT_TEST);
        let col2: Vec<_> = grid.iterate_col(1).collect();
        assert_eq!(col2, vec![&b'0', &b'5', &b'5', &b'3', &b'5'])
    }

    #[test]
    fn test_iterate_col_reverse() {
        let grid = Grid::from_bytes(INPUT_TEST);
        let col3_reverse: Vec<_> = grid.iterate_col_reverse(2).collect();
        assert_eq!(col3_reverse, vec![&b'3', &b'5', &b'3', &b'5', &b'3'])
    }

    #[test]
    fn test_iterate_visible_trees_row() {
        let grid = Grid::from_bytes(INPUT_TEST);
        let row3_visible = grid.visible_trees_for_row(2);
        assert_eq!(row3_visible, vec![true, true, false, true, true])
    }

    #[test]
    fn test_iterate_visible_trees_col() {
        let grid = Grid::from_bytes(INPUT_TEST);
        let col5_visible = grid.visible_trees_for_col(4);
        assert_eq!(col5_visible, vec![true, false, false, true, true])
    }

    #[test]
    fn test_scenic_score_top() {
        let grid = Grid::from_bytes(INPUT_TEST);
        assert_eq!(grid.scenic_score_top(1, 2), 1);
        assert_eq!(grid.scenic_score_top(3, 2), 2);
    }

    #[test]
    fn test_scenic_score_left() {
        let grid = Grid::from_bytes(INPUT_TEST);
        assert_eq!(grid.scenic_score_left(1, 2), 1);
        assert_eq!(grid.scenic_score_left(3, 2), 2);
    }

    #[test]
    fn test_scenic_score_bottom() {
        let grid = Grid::from_bytes(INPUT_TEST);
        assert_eq!(grid.scenic_score_bottom(1, 2), 2);
        assert_eq!(grid.scenic_score_bottom(3, 2), 1);
    }

    #[test]
    fn test_scenic_score_right() {
        let grid = Grid::from_bytes(INPUT_TEST);
        assert_eq!(grid.scenic_score_right(1, 2), 2);
        assert_eq!(grid.scenic_score_right(3, 2), 2);
    }

    #[test]
    fn test_input_part1() {
        let grid = Grid::from_bytes(INPUT_TEST);
        assert_eq!(run_part1(&grid), 21);
    }

    #[test]
    fn test_input_part2() {
        let grid = Grid::from_bytes(INPUT_TEST);
        eprintln!("{:#?}", grid.scenic_score());
        assert_eq!(run_part2(&grid), 8);
    }
}
