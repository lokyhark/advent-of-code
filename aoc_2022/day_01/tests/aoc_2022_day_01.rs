use aoc::Result;

use aoc_2022_day_01::*;

#[test]
fn part_one_answer() -> Result<()> {
    let input = include_str!("../input/input.txt");
    assert_eq!(part_one(input)?, 67658);
    Ok(())
}

#[test]
fn part_two_answer() -> Result<()> {
    let input = include_str!("../input/input.txt");
    assert_eq!(part_two(input)?, 200158);
    Ok(())
}
