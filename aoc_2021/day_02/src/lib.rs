use aoc::*;

pub const YEAR: u32 = 2021;
pub const DAY: u32 = 2;

pub fn part_one(input: &str) -> Result<u32> {
    let mut submarine = Submarine1::default();
    course(&mut submarine, input)?;
    Ok(submarine.position * submarine.depth)
}

pub fn part_two(input: &str) -> Result<u32> {
    let mut submarine = Submarine2::default();
    course(&mut submarine, input)?;
    Ok(submarine.position * submarine.depth)
}

fn course<S: Submarine>(submarine: &mut S, input: &str) -> Result<()> {
    for line in input.lines() {
        let mut split = line.split_ascii_whitespace();
        let command = match split.next() {
            Some(command) => command,
            None => return err!("invalid command"),
        };
        let value = match split.next().map(|x| x.parse::<u32>()) {
            Some(Ok(value)) => value,
            _ => return err!("invalid command"),
        };
        match command {
            "forward" => submarine.forward(value),
            "down" => submarine.down(value),
            "up" => submarine.up(value),
            _ => return err!("invalid command"),
        }
    }
    Ok(())
}

pub trait Submarine {
    fn forward(&mut self, value: u32);
    fn down(&mut self, value: u32);
    fn up(&mut self, value: u32);
}

#[derive(Debug, Default)]
struct Submarine1 {
    position: u32,
    depth: u32,
}

impl Submarine for Submarine1 {
    fn forward(&mut self, value: u32) {
        self.position += value;
    }

    fn down(&mut self, value: u32) {
        self.depth += value;
    }

    fn up(&mut self, value: u32) {
        self.depth -= value;
    }
}

#[derive(Debug, Default)]
struct Submarine2 {
    position: u32,
    depth: u32,
    aim: u32,
}

impl Submarine for Submarine2 {
    fn forward(&mut self, value: u32) {
        self.position += value;
        self.depth += self.aim * value;
    }

    fn down(&mut self, value: u32) {
        self.aim += value;
    }

    fn up(&mut self, value: u32) {
        self.aim -= value;
    }
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_one(input)?, 150);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_two(input)?, 900);
    Ok(())
}
