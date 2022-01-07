use std::collections::HashSet;

use aoc::Result;

pub const YEAR: u32 = 2020;
pub const DAY: u32 = 1;

pub fn part_one(input: &str) -> Result<i32> {
    let mut report = HashSet::new();
    for expense in input.trim().lines() {
        let expense = match expense.parse::<i32>() {
            Ok(expense) => expense,
            Err(_) => return Err(format!("invalid expense: '{}", expense).into()),
        };
        report.insert(expense);
    }
    for x in report.iter() {
        if let Some(y) = report.get(&(2020 - x)) {
            return Ok(x * y);
        }
    }
    Err("complement expenses not found".into())
}

pub fn part_two(input: &str) -> Result<i32> {
    let mut report = HashSet::new();
    for expense in input.trim().lines() {
        let expense = match expense.parse::<i32>() {
            Ok(expense) => expense,
            Err(_) => return Err(format!("invalid expense: '{}", expense).into()),
        };
        report.insert(expense);
    }
    for x in report.iter() {
        for y in report.iter() {
            if let Some(z) = report.get(&(2020 - x - y)) {
                return Ok(x * y * z);
            }
        }
    }
    Err("complement expenses not found".into())
}

#[test]
fn part_one_example() -> Result<()> {
    assert_eq!(part_one("1721\n979\n366\n299\n675\n1456\n")?, 514579);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    assert_eq!(part_two("1721\n979\n366\n299\n675\n1456\n")?, 241861950);
    Ok(())
}
