use std::{collections::HashMap, iter::zip};

use aoc::*;

pub const YEAR: u32 = 2024;
pub const DAY: u32 = 1;

pub fn part_one(input: &str) -> Result<u32> {
    let (mut left, mut right) = parse(input)?;
    left.sort_unstable();
    right.sort_unstable();
    let mut diff = 0;
    for (x, y) in zip(left, right) {
        diff += x.abs_diff(y);
    }
    Ok(diff)
}

pub fn part_two(input: &str) -> Result<u32> {
    let (left, right) = parse(input)?;
    let mut map = HashMap::new();
    for x in right {
        *map.entry(x).or_default() += 1
    }
    let mut score = 0;
    for x in left {
        score += x * map.get(&x).copied().unwrap_or(0u32);
    }
    Ok(score)
}

fn parse(input: &str) -> Result<(Vec<u32>, Vec<u32>)> {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let mut split = line.split_ascii_whitespace();
        match split.next() {
            Some(first) => match first.parse() {
                Ok(value) => left.push(value),
                Err(_) => return err!("invalid input"),
            },
            None => return err!("invalid input"),
        }
        match split.next() {
            Some(second) => match second.parse() {
                Ok(value) => right.push(value),
                Err(_) => return err!("invalid input"),
            },
            None => return err!("invalid input"),
        }
    }
    Ok((left, right))
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_one(input)?, 11);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_two(input)?, 31);
    Ok(())
}
