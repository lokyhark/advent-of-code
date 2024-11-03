use std::collections::HashSet;

use aoc::*;

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 5;

pub fn part_one(input: &str) -> Result<u32> {
    let mut nice = 0;
    for line in input.trim().lines() {
        if is_nice1(line) {
            nice += 1;
        }
    }
    Ok(nice)
}

pub fn part_two(input: &str) -> Result<u32> {
    let mut nice = 0;
    for line in input.trim().lines() {
        if is_nice2(line) {
            nice += 1;
        }
    }
    Ok(nice)
}

fn is_nice1(string: &str) -> bool {
    if string.chars().filter(|c| "aeiou".contains(*c)).count() < 3 {
        return false;
    }
    let mut iter = string.chars().peekable();
    while let Some(char) = iter.next() {
        match iter.peek() {
            Some(peek) if *peek == char => break,
            Some(_) => continue,
            None => return false,
        }
    }
    if string.contains("ab") || string.contains("cd") || string.contains("pq") || string.contains("xy") {
        return false;
    }
    true
}

fn is_nice2(string: &str) -> bool {
    let mut set = HashSet::new();
    let mut iter = string.as_bytes().windows(2).peekable();
    while let Some(next) = iter.next() {
        if iter.peek() == Some(&next) {
            iter.next();
        }
        if !set.insert(next) {
            break;
        }
        if iter.peek().is_none() {
            return false;
        }
    }
    if !string.as_bytes().windows(3).any(|w| w[0] == w[2]) {
        return false;
    }
    true
}

#[test]
fn part_one_example1() {
    assert!(is_nice1("ugknbfddgicrmopn"));
}

#[test]
fn part_one_example2() {
    assert!(is_nice1("aaa"));
}

#[test]
fn part_one_example3() {
    assert!(!is_nice1("jchzalrnumimnmhp"));
}

#[test]
fn part_one_example4() {
    assert!(!is_nice1("haegwjzuvuyypxyu"));
}

#[test]
fn part_one_example5() {
    assert!(!is_nice1("dvszwmarrgswjxmb"));
}

#[test]
fn part_two_example1() {
    assert!(is_nice2("qjhvhtzxzqqjkmpb"));
}

#[test]
fn part_two_example2() {
    assert!(is_nice2("xxyxx"));
}

#[test]
fn part_two_example3() {
    assert!(!is_nice2("uurcxstgmygtbstg"));
}

#[test]
fn part_two_example4() {
    assert!(!is_nice2("ieodomkazucvgmuy"));
}

#[test]
fn part_two_example5() {
    assert!(!is_nice2("xxx"));
}

#[test]
fn part_two_example6() {
    assert!(is_nice2("xxxx"));
}
