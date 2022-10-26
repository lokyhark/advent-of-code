use aoc::Result;

use aoc_2016_day_02::*;

#[test]
fn part_one_answer() -> Result<()> {
    let input = include_str!("../input/input.txt");
    assert_eq!(part_one(input)?, "82958");
    Ok(())
}

#[test]
fn part_two_answer() -> Result<()> {
    let input = include_str!("../input/input.txt");
    assert_eq!(part_two(input)?, "B3DB8");
    Ok(())
}
