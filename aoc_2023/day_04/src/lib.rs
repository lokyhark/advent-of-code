use std::collections::HashSet;

use aoc::*;

pub const YEAR: u32 = 2023;
pub const DAY: u32 = 4;

pub fn part_one(input: &str) -> Result<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let (_, numbers) = line.split_once(':').expect("invalid card");
        let (wins, draw) = numbers.split_once('|').expect("invalid card");
        let wins: HashSet<_> = wins.trim().split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
        let draw: HashSet<_> = draw.trim().split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
        let count = wins.intersection(&draw).count() as u32;
        if count > 0 {
            let points = 2u32.pow(count - 1);
            sum += points;
        }
    }
    Ok(sum)
}

pub fn part_two(input: &str) -> Result<usize> {
    let mut counts = Vec::new();
    for line in input.lines() {
        let (_, numbers) = line.split_once(':').expect("invalid card");
        let (wins, draw) = numbers.split_once('|').expect("invalid card");
        let wins: HashSet<_> = wins.trim().split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
        let draw: HashSet<_> = draw.trim().split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
        let count = wins.intersection(&draw).count();
        counts.push(count)
    }
    let mut copies = vec![1; counts.len()];
    for (idx, count) in counts.iter().enumerate() {
        if *count == 0 {
            continue;
        }
        let times = copies[idx];
        let start = idx + 1;
        let stop = idx + count;
        for copy in &mut copies[start..=stop] {
            *copy += times;
        }
    }
    let total = copies.iter().sum();

    Ok(total)
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_one(input)?, 13);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_two(input)?, 30);
    Ok(())
}
