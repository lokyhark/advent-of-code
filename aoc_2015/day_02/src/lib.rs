use aoc::*;

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 2;

pub fn part_one(input: &str) -> Result<u32> {
    let mut paper = 0;
    for line in input.trim().lines() {
        let present: Present = parse_present(line)?;
        paper += present.paper();
    }
    Ok(paper)
}

pub fn part_two(input: &str) -> Result<u32> {
    let mut ribbon = 0;
    for line in input.trim().lines() {
        let present: Present = parse_present(line)?;
        ribbon += present.ribbon();
    }
    Ok(ribbon)
}

fn parse_present(line: &str) -> Result<Present> {
    let mut split = line.trim().split('x');
    let length = match split.next().map(|x| x.parse()) {
        Some(Ok(length)) => length,
        _ => return err!("invalid present dimensions '{}'", line.trim()),
    };
    let width = match split.next().map(|x| x.parse()) {
        Some(Ok(width)) => width,
        _ => return err!("invalid present dimensions '{}'", line.trim()),
    };
    let height = match split.next().map(|x| x.parse()) {
        Some(Ok(height)) => height,
        _ => return err!("invalid present dimensions '{}'", line.trim()),
    };
    Ok(Present { length, width, height })
}

struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn paper(&self) -> u32 {
        let surfaces = [self.length * self.width, self.width * self.height, self.height * self.length];
        surfaces.iter().sum::<u32>() * 2 + surfaces.iter().min().unwrap()
    }

    fn ribbon(&self) -> u32 {
        let perimeters = [2 * (self.length + self.width), 2 * (self.width + self.height), 2 * (self.height + self.length)];
        perimeters.iter().min().unwrap() + self.length * self.width * self.height
    }
}

#[test]
fn part_one_example1() -> Result<()> {
    assert_eq!(parse_present("2x3x4")?.paper(), 58);
    Ok(())
}

#[test]
fn part_one_example2() -> Result<()> {
    assert_eq!(parse_present("1x1x10")?.paper(), 43);
    Ok(())
}

#[test]
fn part_two_example1() -> Result<()> {
    assert_eq!(parse_present("2x3x4")?.ribbon(), 34);
    Ok(())
}

#[test]
fn part_two_example2() -> Result<()> {
    assert_eq!(parse_present("1x1x10")?.ribbon(), 14);
    Ok(())
}
