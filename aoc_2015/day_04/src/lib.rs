use std::fmt::Write;

use md5::{digest::generic_array::GenericArray, Digest, Md5};

use aoc::Result;

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 4;

pub fn part_one(input: &str) -> Result<u32> {
    let input = input.trim();
    let mut hasher = Md5::new();
    let mut array = GenericArray::default();
    let mut value = String::new();
    for int in 1.. {
        write!(&mut value, "{}", int)?;
        hasher.update(input);
        hasher.update(&value);
        hasher.finalize_into_reset(&mut array);
        if array[..2] == [0, 0] && array[2] < 16 {
            return Ok(int);
        }
        value.clear();
    }
    unreachable!()
}

pub fn part_two(input: &str) -> Result<u32> {
    let input = input.trim();
    let mut hasher = Md5::new();
    let mut array = GenericArray::default();
    let mut value = String::new();
    for int in 1.. {
        write!(&mut value, "{}", int)?;
        hasher.update(input);
        hasher.update(&value);
        hasher.finalize_into_reset(&mut array);
        if array[..3] == [0, 0, 0] {
            return Ok(int);
        }
        value.clear();
    }
    unreachable!()
}

#[test]
fn part_one_example1() -> Result<()> {
    assert_eq!(part_one("abcdef")?, 609043);
    Ok(())
}

#[test]
fn part_one_example2() -> Result<()> {
    assert_eq!(part_one("pqrstuv")?, 1048970);
    Ok(())
}
