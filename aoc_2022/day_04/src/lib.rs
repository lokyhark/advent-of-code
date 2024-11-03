use aoc::*;

pub const YEAR: u32 = 2022;
pub const DAY: u32 = 4;

pub fn part_one(input: &str) -> Result<u32> {
    let mut count = 0;
    for line in input.lines() {
        let section = parse_section(line)?;
        if section.contain() {
            count += 1;
        }
    }
    Ok(count)
}

pub fn part_two(input: &str) -> Result<usize> {
    let mut count = 0;
    for line in input.lines() {
        let section = parse_section(line)?;
        if section.overlap() {
            count += 1;
        }
    }
    Ok(count)
}

fn parse_section(slice: &str) -> Result<Section> {
    let mut split = slice.split(',');
    let left = match split.next() {
        Some(slice) => parse_range(slice)?,
        _ => return err!("invalid section"),
    };
    let right = match split.next() {
        Some(slice) => parse_range(slice)?,
        _ => return err!("invalid section"),
    };
    Ok(Section { left, right })
}

fn parse_range(slice: &str) -> Result<Range> {
    let mut split = slice.split('-');
    let start = match split.next().map(|x| x.parse::<u32>()) {
        Some(Ok(start)) => start,
        _ => return err!("invalid range"),
    };
    let end = match split.next().map(|x| x.parse::<u32>()) {
        Some(Ok(end)) => end,
        _ => return err!("invalid range"),
    };
    Ok(Range { start, end })
}

#[derive(Debug)]
struct Section {
    left: Range,
    right: Range,
}

impl Section {
    fn contain(&self) -> bool {
        (self.left.start >= self.right.start && self.left.end <= self.right.end) || (self.right.start >= self.left.start && self.right.end <= self.left.end)
    }

    fn overlap(&self) -> bool {
        self.left.end >= self.right.start && self.left.start <= self.right.end
    }
}

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_one(input)?, 2);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_two(input)?, 4);
    Ok(())
}
