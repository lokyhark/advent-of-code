use aoc::*;
use md5::{digest::generic_array::GenericArray, Digest, Md5};
use std::fmt::Write;

pub const YEAR: u32 = 2016;
pub const DAY: u32 = 5;

pub fn part_one(input: &str) -> Result<String> {
    let input = input.trim();
    let mut hasher = Md5::new();
    let mut array = GenericArray::default();
    let mut value = String::new();
    let mut password = String::new();
    let mut int = 0;
    loop {
        write!(&mut value, "{}", int)?;
        hasher.update(input);
        hasher.update(&value);
        hasher.finalize_into_reset(&mut array);
        if array[..2] == [0, 0] && array[2] < 16 {
            write!(&mut password, "{:x}", array[2])?;
        }
        if password.len() == 8 {
            break Ok(password);
        }
        value.clear();
        int += 1;
    }
}

pub fn part_two(input: &str) -> Result<String> {
    let input = input.trim();
    let mut hasher = Md5::new();
    let mut array = GenericArray::default();
    let mut value = String::new();
    let mut password = [char::default(); 8];
    let mut int = 0;
    while password.contains(&'\0') {
        write!(&mut value, "{}", int)?;
        hasher.update(input);
        hasher.update(&value);
        hasher.finalize_into_reset(&mut array);
        if array[..2] == [0, 0] && array[2] <= 7 {
            let index = array[2] as usize;
            if password[index] == '\0' {
                let char = char::from_digit((array[3] / 16).into(), 16).unwrap();
                password[index] = char;
            }
        }
        value.clear();
        int += 1;
    }
    Ok(password.iter().collect())
}

#[test]
fn part_one_example() -> Result<()> {
    assert_eq!(part_one("abc")?, "18f47a30");
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    assert_eq!(part_two("abc")?, "05ace8e3");
    Ok(())
}
