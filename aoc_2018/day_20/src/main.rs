use std::io::{stdout, Write};

use aoc::Result;

use aoc_2018_day_20::*;

fn main() -> Result<()> {
    writeln!(stdout(), "Advent of Code {}-{:02}", YEAR, DAY)?;
    let input = aoc::input_from_stdin()?;
    let answer = part_one(&input)?;
    writeln!(stdout(), "--> part one:")?;
    writeln!(stdout(), "{}", answer)?;
    let answer = part_two(&input)?;
    writeln!(stdout(), "--> part two:")?;
    writeln!(stdout(), "{}", answer)?;
    Ok(())
}
