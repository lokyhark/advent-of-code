use std::collections::BTreeMap;

use itertools::Itertools;

use aoc::Result;

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 17;

pub fn part_one(input: &str) -> Result<usize> {
    let containers = parse_input(input)?;
    let count = combinations(&containers, 150);
    Ok(count)
}

pub fn part_two(input: &str) -> Result<usize> {
    let containers = parse_input(input)?;
    let count = minimal_combinations(&containers, 150);
    Ok(count)
}

fn parse_input(input: &str) -> Result<Vec<u32>> {
    let mut containers = Vec::new();
    for line in input.trim().lines() {
        match line.parse() {
            Ok(value) => containers.push(value),
            Err(_) => return Err("invalid input".into()),
        }
    }
    if containers.is_empty() {
        return Err("invalid input".into());
    }
    Ok(containers)
}

fn combinations(containers: &[u32], eggnog: u32) -> usize {
    containers.iter().powerset().filter(|set| set.iter().copied().sum::<u32>() == eggnog).count()
}

fn minimal_combinations(containers: &[u32], eggnog: u32) -> usize {
    let powerset = containers.iter().powerset().filter(|set| set.iter().copied().sum::<u32>() == eggnog);
    let mut map = BTreeMap::new();
    for combination in powerset {
        *map.entry(combination.len()).or_insert(0_usize) += 1;
    }
    *map.values().next().unwrap()
}

#[test]
fn part_one_example() -> Result<()> {
    let containers = vec![20, 15, 10, 5, 5];
    assert_eq!(combinations(&containers, 25), 4);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let containers = vec![20, 15, 10, 5, 5];
    assert_eq!(minimal_combinations(&containers, 25), 3);
    Ok(())
}
