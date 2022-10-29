use std::collections::HashMap;

use aoc::Result;

pub const YEAR: u32 = 2016;
pub const DAY: u32 = 6;

pub fn part_one(input: &str) -> Result<String> {
    let words: Vec<_> = input.trim().lines().map(|line| line.trim()).collect();
    let mut maps: Vec<HashMap<char, u32>> = Vec::new();
    for word in words {
        for (i, char) in word.char_indices() {
            match maps.get_mut(i) {
                Some(map) => {
                    *map.entry(char).or_default() += 1;
                }
                None => {
                    let mut map = HashMap::new();
                    map.insert(char, 1);
                    maps.push(map)
                }
            };
        }
    }
    let mut message = String::with_capacity(maps.len());
    for map in maps {
        match map.into_iter().max_by(|x, y| x.1.cmp(&y.1)) {
            Some((c, _)) => message.push(c),
            _ => unreachable!(),
        }
    }
    Ok(message)
}

pub fn part_two(input: &str) -> Result<String> {
    let words: Vec<_> = input.trim().lines().map(|line| line.trim()).collect();
    let mut maps: Vec<HashMap<char, u32>> = Vec::new();
    for word in words {
        for (i, char) in word.char_indices() {
            match maps.get_mut(i) {
                Some(map) => {
                    *map.entry(char).or_default() += 1;
                }
                None => {
                    let mut map = HashMap::new();
                    map.insert(char, 1);
                    maps.push(map)
                }
            };
        }
    }
    let mut message = String::with_capacity(maps.len());
    for map in maps {
        match map.into_iter().min_by(|x, y| x.1.cmp(&y.1)) {
            Some((c, _)) => message.push(c),
            _ => unreachable!(),
        }
    }
    Ok(message)
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_one(input)?, "easter");
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_two(input)?, "advent");
    Ok(())
}
