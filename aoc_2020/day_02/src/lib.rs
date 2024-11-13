use aoc::*;

pub const YEAR: u32 = 2020;
pub const DAY: u32 = 2;

pub fn part_one(input: &str) -> Result<usize> {
    let mut count = 0;
    for line in input.lines() {
        let (policy, password) = line.split_once(':').expect("invalid password database");
        let policy = parse_policy(policy.trim());
        if policy.valid1(password.trim()) {
            count += 1;
        }
    }
    Ok(count)
}

pub fn part_two(input: &str) -> Result<usize> {
    let mut count = 0;
    for line in input.lines() {
        let (policy, password) = line.split_once(':').expect("invalid password database");
        let policy = parse_policy(policy.trim());
        if policy.valid2(password.trim()) {
            count += 1;
        }
    }
    Ok(count)
}

fn parse_policy(policy: &str) -> Policy {
    let mut split = policy.split([' ', '-']);
    let inf = split.next().unwrap().parse().unwrap();
    let sup = split.next().unwrap().parse().unwrap();
    let char = split.next().unwrap().chars().next().unwrap();
    Policy { inf, sup, char }
}

#[derive(Debug)]
pub struct Policy {
    inf: usize,
    sup: usize,
    char: char,
}

impl Policy {
    fn valid1(&self, password: &str) -> bool {
        let count = password.chars().filter(|&x| x == self.char).count();
        (self.inf..=self.sup).contains(&count)
    }

    fn valid2(&self, password: &str) -> bool {
        let first = password.chars().nth(self.inf - 1).unwrap() == self.char;
        let second = password.chars().nth(self.sup - 1).unwrap() == self.char;
        first ^ second
    }
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
    assert_eq!(part_two(input)?, 1);
    Ok(())
}
