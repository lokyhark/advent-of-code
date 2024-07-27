use aoc::Result;

pub const YEAR: u32 = 2022;
pub const DAY: u32 = 1;

pub fn part_one(input: &str) -> Result<u32> {
    let vec = parse_list(input)?;
    match vec.iter().max() {
        Some(max) => Ok(*max),
        None => Err("invalid calories list".into()),
    }
}

pub fn part_two(input: &str) -> Result<u32> {
    let mut vec = parse_list(input)?;
    vec.sort_unstable();
    vec.reverse();
    let sum = vec.into_iter().take(3).sum::<u32>();
    Ok(sum)
}

pub fn parse_list(input: &str) -> Result<Vec<u32>> {
    let mut vec = Vec::new();
    let mut calories = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            vec.push(calories);
            calories = 0;
        } else {
            calories += match line.trim().parse::<u32>() {
                Ok(value) => value,
                Err(_) => return Err(format!("invalid calories number: {}", line).into()),
            }
        }
    }
    vec.push(calories);
    Ok(vec)
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_one(input)?, 24000);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_two(input)?, 45000);
    Ok(())
}
