use std::fmt::Write;

use aoc::Result;

pub const YEAR: u32 = 2022;
pub const DAY: u32 = 5;

pub fn part_one(input: &str) -> Result<String> {
    let (mut ship, procedure) = parse_input(input)?;
    let crane = CrateMover9000;
    for action in procedure.actions() {
        crane.operate(&mut ship, action)?;
    }
    ship.top()
}

pub fn part_two(input: &str) -> Result<String> {
    let (mut ship, procedure) = parse_input(input)?;
    let crane = CrateMover9001;
    for action in procedure.actions() {
        crane.operate(&mut ship, action)?;
    }
    ship.top()
}

fn parse_input(input: &str) -> Result<(Ship, Procedure)> {
    let mut lines = input.lines();
    let mut line = match lines.next() {
        None => return Err("invalid ship".into()),
        Some(line) => line,
    };
    if line.trim().is_empty() {
        return Err("invalid ship".into());
    }
    let mut last = line;
    let mut ship = String::new();
    loop {
        line = match lines.next() {
            None => return Err("invalid ship".into()),
            Some("") => break,
            Some(line) => line,
        };
        writeln!(ship, "{}", last)?;
        last = line;
    }
    let size = last.split_ascii_whitespace().count();
    let ship = parse_ship(&ship, size)?;
    let slice = lines.collect::<Vec<_>>().join("\n");
    let procedure = parse_procedure(&slice)?;
    Ok((ship, procedure))
}

fn parse_ship(slice: &str, size: usize) -> Result<Ship> {
    let mut ship = Ship::new(size);
    for line in slice.lines() {
        for (idx, mark) in line.bytes().skip(1).step_by(4).enumerate() {
            match mark {
                b' ' => continue,
                mark @ b'A'..=b'Z' => ship.insert(idx, mark),
                _ => return Err("invalid mark".into()),
            }
        }
    }
    ship.reverse();
    Ok(ship)
}

fn parse_procedure(slice: &str) -> Result<Procedure> {
    let mut procedure = Procedure::default();
    for line in slice.lines() {
        let mut split = line.split_ascii_whitespace();
        let count = match split.nth(1).map(|x| x.parse::<usize>()) {
            Some(Ok(count)) => count,
            _ => return Err("invalid procedure".into()),
        };
        let from = match split.nth(1).map(|x| x.parse::<usize>()) {
            Some(Ok(count)) => count - 1,
            _ => return Err("invalid procedure".into()),
        };
        let into = match split.nth(1).map(|x| x.parse::<usize>()) {
            Some(Ok(count)) => count - 1,
            _ => return Err("invalid procedure".into()),
        };
        let action = Action::new(count, from, into);
        procedure.push(action);
    }
    Ok(procedure)
}

trait Crane {
    fn operate(&self, ship: &mut Ship, action: &Action) -> Result<()>;
}

struct CrateMover9000;

impl Crane for CrateMover9000 {
    fn operate(&self, ship: &mut Ship, action: &Action) -> Result<()> {
        for _ in 0..action.count {
            let from = &mut ship.stacks[action.from];
            match from.pop() {
                Some(mark) => {
                    let into = &mut ship.stacks[action.into];
                    into.push(mark)
                }
                None => return Err("empty stack".into()),
            };
        }
        Ok(())
    }
}

struct CrateMover9001;

impl Crane for CrateMover9001 {
    fn operate(&self, ship: &mut Ship, action: &Action) -> Result<()> {
        let mut vec = Vec::new();
        let from = &mut ship.stacks[action.from];
        for _ in 0..action.count {
            match from.pop() {
                Some(mark) => vec.push(mark),
                None => return Err("empty stack".into()),
            };
        }
        let into = &mut ship.stacks[action.into];
        into.extend(vec.into_iter().rev());
        Ok(())
    }
}

#[derive(Debug)]
struct Ship {
    stacks: Vec<Stack>,
}

impl Ship {
    fn new(size: usize) -> Self {
        Self { stacks: vec![Stack::default(); size] }
    }

    fn insert(&mut self, index: usize, mark: u8) {
        self.stacks[index].push(mark);
    }

    fn reverse(&mut self) {
        for stack in &mut self.stacks {
            stack.reverse();
        }
    }

    fn top(&self) -> Result<String> {
        let mut vec = Vec::new();
        for stack in &self.stacks {
            match stack.top() {
                Some(top) => vec.push(top),
                None => return Err("empty stack".into()),
            }
        }
        let top = String::from_utf8(vec)?;
        Ok(top)
    }
}

#[derive(Clone, Debug, Default)]
struct Stack {
    crates: Vec<u8>,
}

impl Stack {
    fn reverse(&mut self) {
        self.crates.reverse();
    }

    fn push(&mut self, mark: u8) {
        self.crates.push(mark)
    }

    fn pop(&mut self) -> Option<u8> {
        self.crates.pop()
    }

    fn extend<I: Iterator<Item = u8>>(&mut self, marks: I) {
        self.crates.extend(marks);
    }

    fn top(&self) -> Option<u8> {
        self.crates.last().copied()
    }
}

#[derive(Debug, Default)]
struct Procedure {
    actions: Vec<Action>,
}

impl Procedure {
    fn push(&mut self, action: Action) {
        self.actions.push(action)
    }

    fn actions(&self) -> impl Iterator<Item = &Action> {
        self.actions.iter()
    }
}

#[derive(Debug)]
struct Action {
    count: usize,
    from: usize,
    into: usize,
}

impl Action {
    fn new(count: usize, from: usize, into: usize) -> Self {
        Self { count, from, into }
    }
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_one(input)?, "CMZ");
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_two(input)?, "MCD");
    Ok(())
}
