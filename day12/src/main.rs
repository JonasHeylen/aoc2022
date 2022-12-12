use std::time::Instant;

const INPUT: &[u8] = include_bytes!("../input.txt");

type Pos = (usize, usize);

#[derive(Debug)]
struct Grid {
    elevations: Vec<u8>,
    distances: Vec<Vec<usize>>,
    cols: usize,
    rows: usize,
    start: Pos,
    end: Pos,
}

impl Grid {
    fn from_bytes(elevations: &[u8]) -> Self {
        let cols = elevations.iter().position(|&b| b == b'\n').unwrap();
        let rows = elevations.len() / cols;
        let start = Grid::index_to_pos(cols, elevations.iter().position(|&b| b == b'S').unwrap());
        let end = Grid::index_to_pos(cols, elevations.iter().position(|&b| b == b'E').unwrap());
        Self {
            elevations: elevations.iter().cloned().collect(),
            distances: vec![vec![usize::MAX; rows]; cols],
            cols,
            rows,
            start,
            end,
        }
    }

    fn set_start_and_end_elevation(&mut self) {
        let start_index = self.pos_to_index(self.start);
        let end_index = self.pos_to_index(self.end);
        self.elevations[start_index] = b'a';
        self.elevations[end_index] = b'z';
    }

    fn index_to_pos(cols: usize, index: usize) -> Pos {
        let row = index / (cols + 1);
        let col = index % (cols + 1);
        (col, row)
    }

    fn pos_to_index(&self, pos: Pos) -> usize {
        pos.0 as usize + pos.1 as usize * (self.cols + 1)
    }

    fn elevation(&self, pos: Pos) -> u8 {
        self.elevations[self.pos_to_index(pos)]
    }

    fn distance(&self, pos: Pos) -> usize {
        self.distances[pos.0][pos.1]
    }

    fn set_distance(&mut self, pos: Pos, new_value: usize) {
        self.distances[pos.0][pos.1] = new_value;
    }
}

fn main() {
    let start_part1 = Instant::now();
    let result_part1 = run_part1(INPUT);
    let elapsed_time_part1 = start_part1.elapsed().as_millis();
    println!("Part 1: {result_part1} in {elapsed_time_part1} ms");

    let start_part2 = Instant::now();
    let result_part2 = run_part2(INPUT);
    let elapsed_time_part2 = start_part2.elapsed().as_millis();
    println!("Part 2: {result_part2} in {elapsed_time_part2} ms")
}

fn run_part1(input: &[u8]) -> usize {
    let mut grid = Grid::from_bytes(input);
    grid.set_start_and_end_elevation();
    let start_pos = grid.start;
    update_distance(&mut grid, start_pos, 0, b'a');
    grid.distance(grid.end) - 1
}

fn run_part2(input: &[u8]) -> usize {
    let mut grid = Grid::from_bytes(input);
    grid.set_start_and_end_elevation();
    let start_pos = grid.end;
    update_distance(&mut grid, start_pos, 0, b'z');
    0
}

fn update_distance(grid: &mut Grid, pos: Pos, prev_distance: usize, prev_elevation: u8) {
    let elevation = grid.elevation(pos);
    let distance = prev_distance + 1;
    #[cfg(test)]
    {
        println!("update distance for {:?}", pos);
        println!(
            "prev elevation: {}, current elevation {}",
            prev_elevation, elevation
        );
        println!("prev dist: {}, current dist {}", prev_distance, distance);
    }
    if grid.distance(pos) > distance && elevation <= prev_elevation + 1 {
        grid.set_distance(pos, distance);

        let steps = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for step in steps {
            let new_pos = (pos.0 as i32 + step.0, pos.1 as i32 + step.1);
            if !(new_pos.0 < 0
                || new_pos.1 < 0
                || new_pos.0 >= grid.cols as i32
                || new_pos.1 >= grid.rows as i32)
            {
                let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
                update_distance(grid, new_pos, distance, elevation);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &[u8] = include_bytes!("../input_test.txt");

    #[test]
    fn test_input_part1() {
        assert_eq!(run_part1(INPUT_TEST), 31);
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(run_part2(INPUT_TEST), 0);
    }
}
