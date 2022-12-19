use std::{cmp::Ordering, collections::HashSet, ops::Sub, time::Instant};

use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Resources {
    ore: u8,
    clay: u8,
    obsidian: u8,
    cracked_geodes: u8,
}

impl Resources {
    fn new(ore: u8, clay: u8, obsidian: u8, cracked_geodes: u8) -> Self {
        Self {
            ore,
            clay,
            obsidian,
            cracked_geodes,
        }
    }
}

impl PartialOrd for Resources {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let field_orderings = [
            self.ore.partial_cmp(&other.ore).unwrap(),
            self.clay.partial_cmp(&other.clay).unwrap(),
            self.obsidian.partial_cmp(&other.obsidian).unwrap(),
            self.cracked_geodes
                .partial_cmp(&other.cracked_geodes)
                .unwrap(),
        ];

        let has_less = field_orderings.contains(&Ordering::Less);
        let has_greater = field_orderings.contains(&Ordering::Greater);

        if has_less && has_greater {
            None
        } else if has_greater {
            Some(Ordering::Greater)
        } else if has_less {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Sub for Resources {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            cracked_geodes: self.cracked_geodes - rhs.cracked_geodes,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Blueprint {
    cost_ore_robot: Resources,
    cost_clay_robot: Resources,
    cost_obsidian_robot: Resources,
    cost_geode_cracking_robot: Resources,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    ore_robots: u8,
    clay_robots: u8,
    obsidian_robots: u8,
    geode_cracking_robots: u8,
    resources: Resources,
}

impl State {
    fn initial() -> Self {
        Self {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_cracking_robots: 0,
            resources: Resources::default(),
        }
    }

    fn build_ore_robot(&self, cost: &Resources) -> Self {
        Self {
            ore_robots: self.ore_robots + 1,
            resources: self.resources - *cost,
            ..*self
        }
    }

    fn build_clay_robot(&self, cost: &Resources) -> Self {
        Self {
            clay_robots: self.clay_robots + 1,
            resources: self.resources - *cost,
            ..*self
        }
    }

    fn build_obsidian_robot(&self, cost: &Resources) -> Self {
        Self {
            obsidian_robots: self.obsidian_robots + 1,
            resources: self.resources - *cost,
            ..*self
        }
    }

    fn build_geode_cracking_robot(&self, cost: &Resources) -> Self {
        Self {
            geode_cracking_robots: self.geode_cracking_robots + 1,
            resources: self.resources - *cost,
            ..*self
        }
    }
}

fn parse(input: &str) -> Vec<Blueprint> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Blueprint {
    let re = Regex::new(r"^Blueprint \d+: Each ore robot costs (.+)\. Each clay robot costs (.+)\. Each obsidian robot costs (.+)\. Each geode robot costs (.+)\.$").unwrap();
    let captures = re.captures(line).unwrap();
    Blueprint {
        cost_ore_robot: parse_cost(captures.get(1).unwrap().as_str()),
        cost_clay_robot: parse_cost(captures.get(2).unwrap().as_str()),
        cost_obsidian_robot: parse_cost(captures.get(3).unwrap().as_str()),
        cost_geode_cracking_robot: parse_cost(captures.get(4).unwrap().as_str()),
    }
}

fn parse_cost(cost: &str) -> Resources {
    let re = Regex::new(r"^(?:(\d+) ore)?(?: and )?(?:(\d+) clay)?(?: and )?(?:(\d+) obsidian)?$")
        .unwrap();
    let captures = re.captures(cost).unwrap();
    Resources {
        ore: captures.get(1).map_or_else(
            || panic!("Invalid ore cost"),
            |c| c.as_str().parse().unwrap(),
        ),
        clay: captures.get(2).map_or(0, |c| c.as_str().parse().unwrap()),
        obsidian: captures.get(3).map_or(0, |c| c.as_str().parse().unwrap()),
        cracked_geodes: 0,
    }
}

fn find_maximum_open_geodes(blueprint: &Blueprint, minutes: usize) -> u8 {
    let mut states = Vec::with_capacity(134_217_728);
    states.push(State::initial());
    let mut new_states = HashSet::with_capacity(134_217_728);

    let max_ore_per_min = blueprint
        .cost_ore_robot
        .ore
        .max(blueprint.cost_clay_robot.ore)
        .max(blueprint.cost_obsidian_robot.ore)
        .max(blueprint.cost_geode_cracking_robot.ore);

    for min in 0..minutes {
        for state in states.drain(..) {
            let mut next_states = Vec::with_capacity(5);
            // build robots
            next_states.push(state.clone()); // or build nothing
            if state.ore_robots < max_ore_per_min
                && state.resources >= blueprint.cost_ore_robot
            {
                next_states.push(state.build_ore_robot(&blueprint.cost_ore_robot));
            }
            if state.clay_robots < blueprint.cost_obsidian_robot.clay
                && state.resources >= blueprint.cost_clay_robot
            {
                next_states.push(state.build_clay_robot(&blueprint.cost_clay_robot));
            }
            if state.obsidian_robots < blueprint.cost_geode_cracking_robot.obsidian
                && state.resources >= blueprint.cost_obsidian_robot
            {
                next_states.push(state.build_obsidian_robot(&blueprint.cost_obsidian_robot));
            }
            if state.resources >= blueprint.cost_geode_cracking_robot {
                next_states
                    .push(state.build_geode_cracking_robot(&blueprint.cost_geode_cracking_robot));
            }
            // collect resources
            for next_state in &mut next_states {
                next_state.resources.ore = next_state.resources.ore + state.ore_robots;
                next_state.resources.clay = next_state.resources.clay + state.clay_robots;
                next_state.resources.obsidian =
                    next_state.resources.obsidian + state.obsidian_robots;
                next_state.resources.cracked_geodes =
                    next_state.resources.cracked_geodes + state.geode_cracking_robots;
            }
            new_states.extend(next_states);
        }
        states = new_states.drain().collect();
        println!(
            "After minute {} there are {} possible states.",
            min + 1,
            states.len()
        );
    }
    states
        .iter()
        .map(|s| s.resources.cracked_geodes)
        .max()
        .unwrap()
}

fn run_part1(blueprints: &[Blueprint]) -> usize {
    blueprints
        .iter()
        .map(|blueprint| find_maximum_open_geodes(blueprint, 24))
        .enumerate()
        .map(|(i, open_geodes)| {
            println!(
                "Blueprint {} results in {} open geodes.",
                i + 1,
                open_geodes
            );
            (i + 1) * open_geodes as usize
        })
        .sum()
}

fn run_part2(blueprints: &[Blueprint]) -> usize {
    blueprints
        .iter()
        .take(3)
        .map(|blueprint| find_maximum_open_geodes(blueprint, 32) as usize)
        .product()
}

fn main() {
    let start_partsing = Instant::now();
    let blueprints = parse(INPUT);
    let elapsed_time_parsing = start_partsing.elapsed().as_micros();
    println!("Parsed in {elapsed_time_parsing} µs");

    let start_part1 = Instant::now();
    let result_part1 = run_part1(&blueprints);
    let elapsed_time_part1 = start_part1.elapsed().as_micros();
    println!("Part 1: {:?} in {elapsed_time_part1} µs", result_part1);

    let start_part2 = Instant::now();
    let result_part2 = run_part2(&blueprints);
    let elapsed_time_part2 = start_part2.elapsed().as_micros();
    println!("Part 2: {result_part2} in {elapsed_time_part2} µs")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../input_test.txt");

    #[test]
    fn resources_partial_ord() {
        let r1 = Resources::new(1, 1, 0, 0);
        let r2 = Resources::new(0, 0, 0, 0);
        let r3 = Resources::new(2, 0, 0, 0);
        assert!(r1 >= r2);
        assert!(!(r3 >= r1));
        assert!(!(r3 < r1));
    }

    #[test]
    fn test_parse() {
        let blueprints = parse(INPUT_TEST);
        assert_eq!(blueprints.len(), 2);
        assert_eq!(
            blueprints[0],
            Blueprint {
                cost_ore_robot: Resources::new(4, 0, 0, 0),
                cost_clay_robot: Resources::new(2, 0, 0, 0),
                cost_obsidian_robot: Resources::new(3, 14, 0, 0),
                cost_geode_cracking_robot: Resources::new(2, 0, 7, 0),
            }
        );
        assert_eq!(
            blueprints[1],
            Blueprint {
                cost_ore_robot: Resources::new(2, 0, 0, 0),
                cost_clay_robot: Resources::new(3, 0, 0, 0),
                cost_obsidian_robot: Resources::new(3, 8, 0, 0),
                cost_geode_cracking_robot: Resources::new(3, 0, 12, 0),
            }
        );
    }

    #[test]
    fn test_input_part1() {
        assert_eq!(run_part1(&parse(INPUT_TEST)), 33);
    }

    #[test]
    fn test_input_part2_blueprint1() {
        assert_eq!(find_maximum_open_geodes(&parse(INPUT_TEST)[0], 32), 56);
    }

    #[test]
    fn test_input_part2_blueprint2() {
        assert_eq!(find_maximum_open_geodes(&parse(INPUT_TEST)[1], 32), 62);
    }
}
