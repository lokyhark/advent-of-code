use aoc::Result;

pub const YEAR: u32 = 2022;
pub const DAY: u32 = 2;

pub fn part_one(input: &str) -> Result<u32> {
    let mut score = 0;
    for line in input.lines() {
        let (left, right) = parse_round_one(line.trim())?;
        score += right.score(left);
    }
    Ok(score)
}

pub fn part_two(input: &str) -> Result<u32> {
    let mut score = 0;
    for line in input.lines() {
        let (left, right) = parse_round_two(line.trim())?;
        score += right.score(left);
    }
    Ok(score)
}

fn parse_round_one(line: &str) -> Result<(Shape, Shape)> {
    let mut split = line.split_ascii_whitespace();
    let left = match split.next() {
        Some("A") => Shape::Rock,
        Some("B") => Shape::Paper,
        Some("C") => Shape::Scissors,
        _ => return Err(format!("invalid shapes: {}", line).into()),
    };
    let right = match split.next() {
        Some("X") => Shape::Rock,
        Some("Y") => Shape::Paper,
        Some("Z") => Shape::Scissors,
        _ => return Err(format!("invalid shapes: {}", line).into()),
    };
    Ok((left, right))
}

fn parse_round_two(line: &str) -> Result<(Shape, Shape)> {
    let mut split = line.split_ascii_whitespace();
    let left = match split.next() {
        Some("A") => Shape::Rock,
        Some("B") => Shape::Paper,
        Some("C") => Shape::Scissors,
        _ => return Err(format!("invalid shapes: {}", line).into()),
    };
    let right = match split.next() {
        Some("X") => match left {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        },
        Some("Y") => left,
        Some("Z") => match left {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        },
        _ => return Err(format!("invalid shapes: {}", line).into()),
    };
    Ok((left, right))
}

#[derive(Clone, Copy, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn score(&self, other: Self) -> u32 {
        let mut score = 0;
        score += match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        };
        score += match (self, other) {
            (Shape::Rock, Shape::Rock) => 3,
            (Shape::Rock, Shape::Paper) => 0,
            (Shape::Rock, Shape::Scissors) => 6,
            (Shape::Paper, Shape::Rock) => 6,
            (Shape::Paper, Shape::Paper) => 3,
            (Shape::Paper, Shape::Scissors) => 0,
            (Shape::Scissors, Shape::Rock) => 0,
            (Shape::Scissors, Shape::Paper) => 6,
            (Shape::Scissors, Shape::Scissors) => 3,
        };
        score
    }
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_one(input)?, 15);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_two(input)?, 12);
    Ok(())
}
