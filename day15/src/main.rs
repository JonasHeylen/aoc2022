use std::{
    collections::{hash_map::RandomState, HashSet},
    time::Instant,
};

use parse_display::{Display, FromStr};
use range_map::{Range, RangeSet};

const INPUT: &str = include_str!("../input.txt");

#[derive(Display, Clone, Copy, Hash, FromStr, PartialEq, Eq, Debug)]
#[display("x={x}, y={y}")]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> u32 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }
}

#[derive(Display, Eq, Hash, FromStr, PartialEq, Debug)]
#[display("Sensor at {location}: closest beacon is at {nearest_beacon}")]
struct Sensor {
    location: Point,
    nearest_beacon: Point,
}

impl Sensor {
    fn covered_interval(&self, y: i32) -> Option<(i32, i32)> {
        let dy = (self.location.y - y).abs();
        let dist = self.location.distance(&self.nearest_beacon) as i32;
        if dy > dist {
            return None;
        }
        let from = self.location.x - dist + dy;
        let to = self.location.x + dist - dy;
        Some((from, to))
    }
}

fn run_part1(input: &str, y: i32) -> usize {
    let sensors: Vec<_> = input
        .lines()
        .flat_map(|l| l.parse::<Sensor>().ok())
        .collect();

    let max_sensor_beacon_distance = sensors
        .iter()
        .map(|s| s.location.distance(&s.nearest_beacon))
        .max()
        .unwrap();

    println!("Max sensor-beacon distance {}", max_sensor_beacon_distance);

    let min_x = sensors.iter().map(|s| s.nearest_beacon.x).min().unwrap()
        - max_sensor_beacon_distance as i32;
    let max_x = sensors.iter().map(|s| s.nearest_beacon.x).max().unwrap()
        + max_sensor_beacon_distance as i32;

    println!("Min {} - max {}", min_x, max_x);

    // let sensor_locations: HashSet<Point, RandomState> =
    //     HashSet::from_iter(sensors.iter().map(|s| (s.location)));
    let beacon_locations: HashSet<Point, RandomState> =
        HashSet::from_iter(sensors.iter().map(|s| (s.nearest_beacon)));

    (min_x..=max_x)
        .map(|x| Point { x, y })
        .map(|p| {
            let covered = sensors.iter().any(|sensor| {
                p.distance(&sensor.location) <= sensor.location.distance(&sensor.nearest_beacon)
            });
            (p, covered)
        })
        .filter(|(p, c)| *c && !beacon_locations.contains(p))
        .count()
}

fn run_part2(input: &str, max_loc: usize) -> usize {
    let sensors: Vec<_> = input
        .lines()
        .flat_map(|l| l.parse::<Sensor>().ok())
        .collect();

    for y in 0..=max_loc {
        if y % 1000000 == 0 {
            println!("y = {}", y);
        }
        let range_iter = sensors
            .iter()
            .filter_map(|s| s.covered_interval(y as i32))
            .map(|(a, b)| {
                let min = a.max(0) as usize;
                let max = b.min(max_loc as i32) as usize;
                Range::new(min, max)
            });
        let union = RangeSet::from_iter(range_iter);

        if union.ranges().count() > 1 {
            // I'm sure there's a better way
            let x = union
                .negated()
                .ranges()
                .take(1)
                .collect::<Vec<_>>()
                .first()
                .unwrap()
                .start;
            return x * 4000000 + y;
        }
    }

    todo!();
}

fn main() {
    let start_part1 = Instant::now();
    let result_part1 = run_part1(INPUT, 2000000);
    let elapsed_time_part1 = start_part1.elapsed().as_millis();
    println!("Part 1: {result_part1} in {elapsed_time_part1} ms");

    let start_part2 = Instant::now();
    let result_part2 = run_part2(INPUT, 4000000);
    let elapsed_time_part2 = start_part2.elapsed().as_millis();
    println!("Part 2: {result_part2} in {elapsed_time_part2} ms")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../input_test.txt");

    #[test]
    fn test_input_part1() {
        assert_eq!(run_part1(INPUT_TEST, 10), 26);
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(run_part2(INPUT_TEST, 20), 56000011);
    }
}
