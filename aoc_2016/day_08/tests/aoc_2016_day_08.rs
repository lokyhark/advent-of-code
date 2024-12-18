use aoc::*;

use aoc_2016_day_08::*;

#[test]
fn part_one_answer() -> Result<()> {
    let input = include_str!("../input/input.txt");
    assert_eq!(part_one(input)?, 115);
    Ok(())
}
