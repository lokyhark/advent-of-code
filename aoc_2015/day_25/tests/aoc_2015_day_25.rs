use aoc::*;

use aoc_2015_day_25::*;

#[test]
fn part_one_answer() -> Result<()> {
    let input = include_str!("../input/input.txt");
    assert_eq!(part_one(input)?, 9132360);
    Ok(())
}
