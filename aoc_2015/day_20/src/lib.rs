use std::collections::BTreeMap;

use aoc::*;

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 20;

pub fn part_one(input: &str) -> Result<usize> {
    let target: usize = input.trim().parse()?;
    let mut houses = BTreeMap::new();
    for i in 1usize..target / 10 {
        for j in (i..target / 10).step_by(i) {
            *houses.entry(j).or_insert(0) += i * 10;
        }
    }
    match houses.into_iter().skip_while(|(_, x)| *x < target).map(|(x, _)| x).next() {
        Some(house) => Ok(house),
        None => err!("no house found"),
    }
}

pub fn part_two(input: &str) -> Result<usize> {
    let target: usize = input.trim().parse()?;
    let mut houses = BTreeMap::new();
    for i in 1usize..target / 11 {
        for j in (i..target / 11).step_by(i).take(50) {
            *houses.entry(j).or_insert(0) += i * 11;
        }
    }
    match houses.into_iter().skip_while(|(_, x)| *x < target).map(|(x, _)| x).next() {
        Some(house) => Ok(house),
        None => err!("no house found"),
    }
}
