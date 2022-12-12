const INPUT: &[u8] = include_bytes!("../input.txt");

type Pos = (usize, usize);

struct Grid<'a> {
    elevations: &'a [u8],
    distances: Vec<Vec<usize>>,
    cols: usize,
    rows: usize,
    start: Pos,
    end: Pos,
}

impl<'a> Grid<'a> {
    fn from_bytes(elevations: &'a [u8]) -> Self {
        let cols = elevations.iter().position(|&b| b == b'\n').unwrap();
        let rows = elevations.len() / cols;
        let start = Grid::index_to_pos(cols, elevations.iter().position(|&b| b == b'S').unwrap());
        let end = Grid::index_to_pos(cols, elevations.iter().position(|&b| b == b'E').unwrap());
        Self{
            elevations,
            distances: vec![vec![0; rows]; cols],
            cols,
            rows,
            start,
            end,
        }
    }

    fn index_to_pos(cols: usize, index: usize) -> Pos {
        let row = index / (cols + 1);
        let col = index % (cols + 1);
        (col, row)
    }

    fn pos_to_index(&self, pos: Pos) -> usize {
        pos.0 + pos.1 * (self.cols + 1)
    }

    fn elevation(&self, pos: Pos) -> u8 {
        self.elevations[self.pos_to_index(pos)]
    }

    fn distance(&self, pos: Pos) -> usize {
        self.distances[pos.0][pos.1]
    }
}

fn main() {
    let result_part1 = run_part1(INPUT);
    println!("Part 1: {result_part1}");
    let result_part2 = run_part2(INPUT);
    println!("Part 2: {result_part2}")
}

fn run_part1(input: &[u8]) -> i32 {
    let grid = Grid::from_bytes(input);
    println!("Start: {:?}", grid.start);
    println!("End: {:?}", grid.end);

    todo!();
}

fn run_part2(_input: &[u8]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &[u8] = include_bytes!("../input_test.txt");

    #[test]
    fn test_input_part1() {
        assert_eq!(run_part1(INPUT_TEST), 0);
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(run_part2(INPUT_TEST), 0);
    }
}
