use std::fmt::{Display, Formatter, Result};
use std::time::Instant;

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;

const INPUT: &str = include_str!("../input.txt");

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Point(usize, usize);

impl Point {
    fn down(&self) -> Self {
        Self(self.0, self.1 + 1)
    }
    fn down_left(&self) -> Self {
        Self(self.0 - 1, self.1 + 1)
    }
    fn down_right(&self) -> Self {
        Self(self.0 + 1, self.1 + 1)
    }
}

#[derive(Debug)]
struct Path {
    points: Vec<Point>,
}

impl Path {
    fn x_min(&self) -> usize {
        self.points.iter().map(|p| p.0).min().unwrap()
    }

    fn x_max(&self) -> usize {
        self.points.iter().map(|p| p.0).max().unwrap()
    }

    fn y_max(&self) -> usize {
        self.points.iter().map(|p| p.1).max().unwrap()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Elem {
    Air,
    Rock,
    Sand,
}

impl Display for Elem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Elem::Air => write!(f, "."),
            Elem::Rock => write!(f, "#"),
            Elem::Sand => write!(f, "o"),
        }
    }
}

#[derive(Debug)]
struct Grid {
    origin: Point,
    cells: Vec<Vec<Elem>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "   {}", self.origin.0).unwrap();
        for row in 0..self.rows() {
            write!(f, "{}  ", self.origin.1 + row).unwrap();
            for col in 0..self.cols() {
                write!(f, "{} ", self.cells[col][row]).unwrap();
            }
            writeln!(f).unwrap();
        }
        write!(f, "")
    }
}

impl Grid {
    fn fit_paths(paths: &[Path]) -> Self {
        let x_min = paths.iter().map(Path::x_min).min().unwrap();
        let x_max = paths.iter().map(Path::x_max).max().unwrap();
        let y_max = paths.iter().map(Path::y_max).max().unwrap();
        Self::with_origin_and_size(Point(x_min, 0), (x_max - x_min) + 1, y_max + 1)
    }

    fn fit_paths_plus_margin(paths: &[Path]) -> Self {
        let x_min = paths.iter().map(Path::x_min).min().unwrap();
        let x_max = paths.iter().map(Path::x_max).max().unwrap();
        let y_max = paths.iter().map(Path::y_max).max().unwrap();
        let height = y_max + 3;
        Self::with_origin_and_size(
            Point(x_min - height, 0),
            (x_max - x_min) + 1 + 2 * height,
            height,
        )
    }

    fn with_origin_and_size(origin: Point, size_x: usize, size_y: usize) -> Self {
        Self {
            origin,
            cells: vec![vec![Elem::Air; size_y]; size_x],
        }
    }

    fn draw_path(&mut self, path: &Path, elem: Elem) {
        path.points
            .windows(2)
            .for_each(|line| self.draw_line(line[0], line[1], elem));
    }

    fn draw_line(&mut self, from: Point, to: Point, elem: Elem) {
        if from.0 == to.0 {
            for y in from.1.min(to.1)..=from.1.max(to.1) {
                self.set_cell(Point(from.0, y), elem);
            }
        } else {
            for x in from.0.min(to.0)..=from.0.max(to.0) {
                self.set_cell(Point(x, from.1), elem);
            }
        }
    }

    fn set_cell(&mut self, point: Point, elem: Elem) {
        let x = point.0 - self.origin.0;
        let y = point.1 - self.origin.1;
        self.cells[x][y] = elem;
    }

    fn get_cell(&mut self, point: Point) -> Elem {
        let x = point.0 - self.origin.0;
        let y = point.1 - self.origin.1;
        self.cells[x][y]
    }

    fn is_in_grid(&self, point: Point) -> bool {
        !(point.0 < self.origin.0
            || point.0 > self.origin.0 + self.cells.len() - 1
            || point.1 < self.origin.1
            || point.1 > self.origin.1 + self.cells.first().unwrap().len() - 1)
    }

    fn cols(&self) -> usize {
        self.cells.len()
    }

    fn rows(&self) -> usize {
        self.cells.first().unwrap().len()
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

fn run_part1(input: &str) -> usize {
    let paths = parse_paths(input);
    let mut grid = Grid::fit_paths(&paths);
    for path in paths {
        grid.draw_path(&path, Elem::Rock);
    }

    let sand_origin = Point(500, 0);
    let mut sand_counter = 0;
    while let Some(_final_pos) = drop_sand(&mut grid, sand_origin) {
        sand_counter += 1;
    }
    // println!("{}", grid);

    sand_counter
}

fn run_part2(input: &str) -> usize {
    let paths = parse_paths(input);
    let mut grid = Grid::fit_paths_plus_margin(&paths);
    let floor_p1 = Point(grid.origin.0, grid.origin.1 + grid.rows() - 1);
    let floor_p2 = Point(
        grid.origin.0 + grid.cols() - 1,
        grid.origin.1 + grid.rows() - 1,
    );
    grid.draw_line(floor_p1, floor_p2, Elem::Rock);
    for path in paths {
        grid.draw_path(&path, Elem::Rock);
    }

    let sand_origin = Point(500, 0);
    let mut sand_counter = 0;
    while let Some(final_pos) = drop_sand(&mut grid, sand_origin) {
        sand_counter += 1;
        if final_pos == sand_origin {
            break;
        }
    }
    // println!("{}", grid);

    sand_counter
}

fn drop_sand(grid: &mut Grid, pos: Point) -> Option<Point> {
    if !grid.is_in_grid(pos.down()) {
        None
    } else if grid.get_cell(pos.down()) == Elem::Air {
        drop_sand(grid, pos.down())
    } else if !grid.is_in_grid(pos.down_left()) {
        None
    } else if grid.get_cell(pos.down_left()) == Elem::Air {
        drop_sand(grid, pos.down_left())
    } else if !grid.is_in_grid(pos.down_right()) {
        None
    } else if grid.get_cell(pos.down_right()) == Elem::Air {
        drop_sand(grid, pos.down_right())
    } else {
        grid.set_cell(pos, Elem::Sand);
        Some(pos)
    }
}

fn parse_paths(input: &str) -> Vec<Path> {
    input
        .lines()
        .flat_map(|line| path(line).map(|(_, path)| path))
        .collect()
}

fn path(input: &str) -> IResult<&str, Path> {
    map(separated_list0(tag(" -> "), point), |points| Path {
        points,
    })(input)
}

fn point(input: &str) -> IResult<&str, Point> {
    map(
        tuple((
            nom::character::complete::u32,
            char(','),
            nom::character::complete::u32,
        )),
        |(x, _, y)| Point(x as usize, y as usize),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../input_test.txt");

    #[test]
    fn test_parse_paths() {
        let paths = parse_paths(INPUT_TEST);
        assert_eq!(paths.len(), 2);

        let first_path = paths.first().unwrap();
        assert_eq!(
            first_path.points,
            vec![Point(498, 4), Point(498, 6), Point(496, 6)]
        );

        assert_eq!(paths.get(1).unwrap().points.len(), 4);
    }

    #[test]
    fn test_input_part1() {
        assert_eq!(run_part1(INPUT_TEST), 24);
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(run_part2(INPUT_TEST), 93);
    }
}
