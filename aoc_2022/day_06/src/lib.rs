use std::collections::HashSet;

use aoc::Result;

pub const YEAR: u32 = 2022;
pub const DAY: u32 = 6;

pub fn part_one(input: &str) -> Result<usize> {
    marker_position(input, 4)
}

pub fn part_two(input: &str) -> Result<usize> {
    marker_position(input, 14)
}

fn marker_position(input: &str, size: usize) -> Result<usize> {
    let stream: Vec<_> = input.bytes().collect();
    for (idx, window) in stream.windows(size).enumerate() {
        if window.iter().collect::<HashSet<_>>().len() == size {
            return Ok(size + idx);
        }
    }
    Err("packet marker not found".into())
}

#[test]
fn part_one_example1() -> Result<()> {
    assert_eq!(part_one("mjqjpqmgbljsphdztnvjfqwrcgsmlb")?, 7);
    Ok(())
}

#[test]
fn part_one_example2() -> Result<()> {
    assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz")?, 5);
    Ok(())
}

#[test]
fn part_one_example3() -> Result<()> {
    assert_eq!(part_one("nppdvjthqldpwncqszvftbrmjlhg")?, 6);
    Ok(())
}

#[test]
fn part_one_example4() -> Result<()> {
    assert_eq!(part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")?, 10);
    Ok(())
}

#[test]
fn part_one_example5() -> Result<()> {
    assert_eq!(part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")?, 11);
    Ok(())
}

#[test]
fn part_two_example1() -> Result<()> {
    assert_eq!(part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb")?, 19);
    Ok(())
}

#[test]
fn part_two_example2() -> Result<()> {
    assert_eq!(part_two("bvwbjplbgvbhsrlpgdmjqwftvncz")?, 23);
    Ok(())
}

#[test]
fn part_two_example3() -> Result<()> {
    assert_eq!(part_two("nppdvjthqldpwncqszvftbrmjlhg")?, 23);
    Ok(())
}

#[test]
fn part_two_example4() -> Result<()> {
    assert_eq!(part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")?, 29);
    Ok(())
}

#[test]
fn part_two_example5() -> Result<()> {
    assert_eq!(part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")?, 26);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    Ok(())
}
