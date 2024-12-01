use std::io::{stdout, Write};

use aoc::*;

use aoc_2015_day_25::*;

fn main() -> Result<()> {
    println!("Advent of Code {}-{:02}", YEAR, DAY);
    let input = aoc::input_from_stdin()?;
    let answer = part_one(&input)?;
    writeln!(stdout(), "--> part one:")?;
    writeln!(stdout(), "{}", answer)?;
    Ok(())
}
