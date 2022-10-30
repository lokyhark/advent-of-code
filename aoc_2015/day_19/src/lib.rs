use std::collections::HashSet;

use aoc::Result;

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 19;

pub fn part_one(input: &str) -> Result<usize> {
    let mut iter = input.lines().map(|line| line.trim());
    let mut replacements = Vec::new();
    loop {
        let line = iter.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut split = line.split(" => ");
        let pattern = split.next().expect("invalid pattern {line}");
        let replace = split.next().expect("invalid replace {line}");
        replacements.push((pattern, replace));
    }
    let initial = iter.next().expect("invalid initial molecule");
    let mut molecules = HashSet::new();
    for (pattern, replace) in replacements {
        let mut start = 0;
        while let Some(index) = initial[start..].find(pattern) {
            start += index;
            let stop = start + pattern.len();
            let mut molecule = initial.to_string();
            molecule.replace_range(start..stop, replace);
            molecules.insert(molecule);
            start = stop;
        }
    }
    Ok(molecules.len())
}

pub fn part_two(input: &str) -> Result<usize> {
    let mut iter = input.lines().map(|line| line.trim());
    let mut replacements = Vec::new();
    loop {
        let line = iter.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut split = line.split(" => ");
        let pattern = split.next().expect("invalid pattern {line}");
        let replace = split.next().expect("invalid replace {line}");
        replacements.push((pattern, replace));
    }
    let mut molecule = iter.next().expect("invalid initial molecule").to_string();
    let mut count = 0;
    while molecule != "e" {
        for (pattern, replace) in &replacements {
            if molecule.contains(replace) {
                molecule = molecule.replacen(replace, pattern, 1);
                count += 1;
            }
        }
    }
    Ok(count)
}

#[test]
fn part_one_example1() -> Result<()> {
    let input = include_str!("../input/example1.txt");
    assert_eq!(part_one(input)?, 4);
    Ok(())
}

#[test]
fn part_one_example2() -> Result<()> {
    let input = include_str!("../input/example2.txt");
    assert_eq!(part_one(input)?, 7);
    Ok(())
}
