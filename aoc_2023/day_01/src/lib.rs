use aoc::*;

pub const YEAR: u32 = 2023;
pub const DAY: u32 = 1;

pub fn part_one(input: &str) -> Result<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let left = line.chars().find(|x| x.is_ascii_digit()).map(|x| x.to_digit(10).unwrap()).expect("no digit found");
        let right = line.chars().rev().find(|x| x.is_ascii_digit()).map(|x| x.to_digit(10).unwrap()).expect("no digit found");
        let val = left * 10 + right;
        sum += val;
    }
    Ok(sum)
}

#[rustfmt::skip]
const PATTERNS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", 
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
];

pub fn part_two(input: &str) -> Result<u32> {
    let mut sum = 0;
    for line in input.lines() {
        // Left
        let mut digit = None;
        let mut pos = line.len();
        for pattern in PATTERNS {
            match line.find(pattern) {
                Some(idx) => {
                    if idx < pos {
                        digit = Some(pattern);
                        pos = idx;
                    }
                }
                _ => continue,
            }
        }
        let left = digit.expect("digit not found");

        // Right
        let mut digit = None;
        let mut pos = 0;
        for pattern in PATTERNS {
            match line.rfind(pattern) {
                Some(idx) => {
                    if idx > pos {
                        digit = Some(pattern);
                        pos = idx;
                    }
                }
                _ => continue,
            }
        }
        let right = digit.unwrap_or(left);

        // Parse
        let left = parse_digit(left)?;
        let right = parse_digit(right)?;

        // Sum
        let val = left * 10 + right;
        sum += val;
    }
    Ok(sum)
}

pub fn parse_digit(digit: &str) -> Result<u32> {
    let digit = match digit.as_bytes() {
        b"1" | b"one" => 1,
        b"2" | b"two" => 2,
        b"3" | b"three" => 3,
        b"4" | b"four" => 4,
        b"5" | b"five" => 5,
        b"6" | b"six" => 6,
        b"7" | b"seven" => 7,
        b"8" | b"eight" => 8,
        b"9" | b"nine" => 9,
        _ => return err!("invalid digit"),
    };
    Ok(digit)
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example1.txt");
    assert_eq!(part_one(input)?, 142);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let input = include_str!("../input/example2.txt");
    assert_eq!(part_two(input)?, 281);
    Ok(())
}
