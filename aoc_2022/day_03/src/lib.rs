use std::collections::HashSet;

use aoc::*;

pub const YEAR: u32 = 2022;
pub const DAY: u32 = 3;

pub fn part_one(input: &str) -> Result<u32> {
    let mut sum = 0;
    for line in input.lines() {
        assert!(line.is_ascii());
        let bytes = line.trim().as_bytes().to_vec();
        let (first, second) = bytes.split_at(bytes.len() / 2);
        let mut left: HashSet<u8> = HashSet::new();
        left.extend(first);
        let mut right = HashSet::new();
        right.extend(second);
        if let [&item] = left.intersection(&right).collect::<Vec<_>>().as_slice() {
            sum += priority(item)?;
        } else {
            return err!("multiple items appear in the two compartment");
        }
    }
    Ok(sum)
}

pub fn part_two(input: &str) -> Result<u32> {
    let mut sum = 0;
    let lines: Vec<_> = input.lines().collect();
    for chunks in lines.chunks_exact(3) {
        let mut set = HashSet::new();
        set.extend(b'a'..=b'z');
        set.extend(b'A'..=b'Z');
        for line in chunks {
            let rucksack: HashSet<u8> = line.trim().bytes().collect();
            set = &set & &rucksack;
        }
        if let [item] = set.into_iter().collect::<Vec<_>>().as_slice() {
            sum += priority(*item)?;
        } else {
            return err!("multiple items appear in the two compartment");
        }
    }
    Ok(sum)
}

fn priority(byte: u8) -> Result<u32> {
    match byte {
        b @ b'a'..=b'z' => Ok((b - b'a' + 1) as u32),
        b @ b'A'..=b'Z' => Ok((b - b'A' + 27) as u32),
        _ => err!("invalid priority: {}", byte.escape_ascii()),
    }
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_one(input)?, 157);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_two(input)?, 70);
    Ok(())
}
