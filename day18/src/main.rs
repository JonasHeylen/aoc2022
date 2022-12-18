use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

const INPUT: &str = include_str!("../input.txt");

fn build_cubes_set(input: &str) -> HashSet<(i32, i32, i32)> {
    input.lines().map(|line| {
        let mut coords = line.split(',').map(|c| {
            c.parse::<i32>()
                .expect(&format!("Invalid coordinate: {}", c))
        });
        (
            coords.next().unwrap(),
            coords.next().unwrap(),
            coords.next().unwrap(),
        )
    }).collect()
}

fn run_part1(input: &str) -> usize {
    let cubes = build_cubes_set(input);

    let mut surface_area = 0;
    for (x, y, z) in &cubes {
        for (dx, dy, dz) in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ] {
            if !cubes.contains(&(x + dx, y + dy, z + dz)) {
                surface_area += 1;
            }
        }
    }

    surface_area
}

fn run_part2(input: &str) -> usize {
    let cubes = build_cubes_set(input);

    let mut external: HashSet<(i32, i32, i32)> = HashSet::new();
    let start = (0, 0, 0);
    let limit = 30;
    assert!(!cubes.contains(&start)); // starting point must be outside of the droplet
    let mut q = VecDeque::from([start]);
    while let Some(p @ (x, y, z)) = q.pop_front() {
        if !cubes.contains(&p) && !external.contains(&p) {
            external.insert(p);
            for (dx, dy, dz) in [
                (-1, 0, 0),
                (1, 0, 0),
                (0, -1, 0),
                (0, 1, 0),
                (0, 0, -1),
                (0, 0, 1),
            ] {
                let p @ (x, y, z) = (x + dx, y + dy, z + dz);
                if x >= -limit && x < limit && y >= -limit && y < limit && z >= -limit && z < limit
                {
                    q.push_back(p);
                }
            }
        }
    }

    let mut surface_area = 0;
    for (x, y, z) in &cubes {
        for (dx, dy, dz) in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ] {
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
