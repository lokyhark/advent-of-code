use aoc::Result;

use aoc_2015_day_11::*;

#[test]
fn part_one_answer() -> Result<()> {
    let input = include_str!("../input/input.txt");
    assert_eq!(part_one(input)?, "hxbxxyzz");
    Ok(())
}

#[test]
fn part_two_answer() -> Result<()> {
    let input = include_str!("../input/input.txt");
    assert_eq!(part_two(input)?, "hxcaabcc");
    Ok(())
}
