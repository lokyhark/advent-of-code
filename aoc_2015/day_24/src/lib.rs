use std::cmp::Ordering;

use itertools::Itertools;

use aoc::*;

pub const YEAR: u64 = 2015;
pub const DAY: u64 = 24;

pub fn part_one(input: &str) -> Result<u64> {
    let weights = parse(input)?;
    let quantum = search(weights, 3)?;
    Ok(quantum)
}

pub fn part_two(input: &str) -> Result<u64> {
    let weights = parse(input)?;
    let quantum = search(weights, 4)?;
    Ok(quantum)
}

fn parse(input: &str) -> Result<Vec<u64>> {
    let mut weights = Vec::new();
    for line in input.lines() {
        match line.trim().parse() {
            Ok(weight) => weights.push(weight),
            Err(_) => return err!("invalid weight: {}", line),
        }
    }
    weights.sort_unstable();
    Ok(weights)
}

fn search(weights: Vec<u64>, size: u64) -> Result<u64> {
    let sum: u64 = weights.iter().sum();
    let threshold = sum / size;
    let mut len = 1;
    let mut min = None;
    loop {
        if len > weights.len() {
            break;
        }
        for group in weights.iter().combinations(len) {
            let weight = group.iter().copied().sum::<u64>();
            match weight.cmp(&threshold) {
                Ordering::Less => {
                    continue;
                }
                Ordering::Equal => {
                    let quantum = group.iter().copied().product::<u64>();
                    match min {
                        Some(min) if min <= quantum => continue,
                        _ => min = Some(quantum),
                    }
                }
                Ordering::Greater => {
                    if let Some(min) = min {
                        return Ok(min);
                    } else {
                        break;
                    }
                }
            }
        }
        len += 1;
    }
    err!("packages combination not found")
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_one(input)?, 99);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_two(input)?, 44);
    Ok(())
}
