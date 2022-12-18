use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

const INPUT: &str = include_str!("../input.txt");

const STEPS: [(i32, i32, i32); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

fn build_cubes_set(input: &str) -> HashSet<(i32, i32, i32)> {
    input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|c| {
                c.parse::<i32>()
                    .expect(&format!("Invalid coordinate: {}", c))
            });
            (
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect()
}

fn build_external_set(cubes: &HashSet<(i32, i32, i32)>) -> HashSet<(i32, i32, i32)> {
    let mut external: HashSet<(i32, i32, i32)> = HashSet::new();
    let (bb_min, bb_max) = bounding_box(cubes);
    let bb_min = (bb_min.0 - 1, bb_min.1 - 1, bb_min.2 - 1);
    let bb_max = (bb_max.0 + 1, bb_max.1 + 1, bb_max.2 + 1);
    let start = bb_min;
    assert!(!cubes.contains(&bb_min)); // starting point must be outside of the droplet
    let mut q = VecDeque::from([start]);
    while let Some(p @ (x, y, z)) = q.pop_front() {
        if !cubes.contains(&p) && !external.contains(&p) {
            external.insert(p);
            for (dx, dy, dz) in STEPS {
                let p @ (x, y, z) = (x + dx, y + dy, z + dz);
                if x >= bb_min.0
                    && x <= bb_max.0
                    && y >= bb_min.1
                    && y <= bb_max.1
                    && z >= bb_min.2
                    && z <= bb_max.2
                {
                    q.push_back(p);
                }
            }
        }
    }
    external
}

fn bounding_box(cubes: &HashSet<(i32, i32, i32)>) -> ((i32, i32, i32), (i32, i32, i32)) {
    cubes.iter().fold(
        (
            (i32::MAX, i32::MAX, i32::MAX),
            (i32::MIN, i32::MIN, i32::MIN),
        ),
        |((x_min, y_min, z_min), (x_max, y_max, z_max)), (x, y, z)| {
            (
                (x_min.min(*x), y_min.min(*y), z_min.min(*z)),
                (x_max.max(*x), y_max.max(*y), z_max.max(*z)),
            )
        },
    )
}

fn run_part1(input: &str) -> usize {
    let cubes = build_cubes_set(input);

    let mut surface_area = 0;
    for (x, y, z) in &cubes {
        for (dx, dy, dz) in STEPS {
            if !cubes.contains(&(x + dx, y + dy, z + dz)) {
                surface_area += 1;
            }
        }
    }

    surface_area
}

fn run_part2(input: &str) -> usize {
    let cubes = build_cubes_set(input);
    let external = build_external_set(&cubes);

    let mut surface_area = 0;
    for (x, y, z) in &cubes {
        for (dx, dy, dz) in STEPS {
            let p = (x + dx, y + dy, z + dz);
            if !cubes.contains(&p) && external.contains(&p) {
                surface_area += 1;
            }
        }
    }

    surface_area
}

fn main() {
    let start_part1 = Instant::now();
    let result_part1 = run_part1(INPUT);
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
        assert_eq!(run_part1(INPUT_TEST), 64);
    }

    #[test]
    fn test_part1_two_cubes() {
        let input = "1,1,1\n2,1,1";
        assert_eq!(run_part1(input), 10);
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(run_part2(INPUT_TEST), 58);
    }
}
