use std::collections::HashSet;

use aoc::*;

pub const YEAR: u32 = 2023;
pub const DAY: u32 = 3;

pub fn part_one(input: &str) -> Result<u32> {
    // Retrieve patterns of symbols
    let mut patterns = Vec::new();
    for line in input.lines() {
        let mut set = HashSet::new();
        for (idx, byte) in line.bytes().enumerate() {
            match byte {
                b'0'..=b'9' => continue,
                b'.' => continue,
                _ => {
                    set.extend([idx - 1, idx, idx + 1]);
                }
            }
        }
        patterns.push(set);
    }
    // Combine patterns over 3 lines
    let mut sets = Vec::new();
    sets.push(&patterns[0] | &patterns[1]);
    for slice in patterns.windows(3) {
        let mut set = HashSet::new();
        for pattern in slice {
            set.extend(pattern);
        }
        sets.push(set);
    }
    sets.push(&patterns[patterns.len() - 2] | &patterns[patterns.len() - 1]);
    // Detect numbers near symbol
    let mut sum = 0;
    for (idx, line) in input.lines().enumerate() {
        let mut iter = line.char_indices().peekable();
        while let Some((pos, char)) = iter.next() {
            if !char.is_ascii_digit() {
                continue;
            }
            let mut number = String::from(char);
            let mut adjacent = false;
            adjacent |= sets[idx].contains(&pos);
            while let Some((pos, digit @ '0'..='9')) = iter.peek() {
                number.push(*digit);
                adjacent |= sets[idx].contains(pos);
                iter.next();
            }
            if adjacent {
                sum += number.parse::<u32>().unwrap();
            }
        }
    }
    Ok(sum)
}

pub fn part_two(input: &str) -> Result<u32> {
    // Retrieve parts
    let mut parts = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for (col, byte) in line.bytes().enumerate() {
            match byte {
                b'*' => parts.push(Part::new(row, col)),
                _ => continue,
            }
        }
    }
    // Retrieve numbers
    let mut numbers = Vec::new();
    for (row, line) in input.lines().enumerate() {
        let mut iter = line.char_indices().peekable();
        while let Some((pos, char)) = iter.next() {
            if !char.is_ascii_digit() {
                continue;
            }
            let mut number = String::from(char);
            let start = pos;
            let mut end = start;
            while let Some((_, digit @ '0'..='9')) = iter.peek() {
                number.push(*digit);
                end += 1;
                iter.next();
            }
            let number = number.parse().unwrap();
            let number = Number::new(number, row, start, end);
            numbers.push(number);
        }
    }
    // Find gears and compute rati
    let mut sum = 0;
    for part in parts {
        let adjacents: Vec<_> = numbers.iter().filter(|number| number.adjacent(part.row, part.col)).collect();
        if adjacents.len() == 2 {
            sum += adjacents.into_iter().map(|n| n.value).product::<u32>();
        }
    }
    Ok(sum)
}

#[derive(Debug)]
struct Number {
    value: u32,
    row: usize,
    start: usize,
    end: usize,
}

impl Number {
    pub fn new(value: u32, row: usize, start: usize, end: usize) -> Self {
        assert!(start <= end);
        Self { value, row, start, end }
    }

    pub fn adjacent(&self, row: usize, col: usize) -> bool {
        let start = match self.row {
            0 => 0,
            row => row - 1,
        };
        let end = self.row + 1;
        if !(start..=end).contains(&row) {
            return false;
        }
        let start = match self.start {
            0 => 0,
            start => start - 1,
        };
        let end = self.end + 1;
        if !(start..=end).contains(&col) {
            return false;
        }
        true
    }
}

#[derive(Debug)]
struct Part {
    row: usize,
    col: usize,
}

impl Part {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_one(input)?, 4361);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_two(input)?, 467835);
    Ok(())
}
