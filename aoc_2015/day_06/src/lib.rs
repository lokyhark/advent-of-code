use aoc::Result;

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 6;

pub fn part_one(input: &str) -> Result<usize> {
    let mut grid = Grid::default();
    for line in input.trim().lines() {
        let (action, range) = parse_instruction(line)?;
        match action {
            Action::TurnOff => grid.turn_off(range),
            Action::TurnOn => grid.turn_on(range),
            Action::Toggle => grid.toggle(range),
        }
    }
    Ok(grid.count())
}

pub fn part_two(input: &str) -> Result<usize> {
    let mut grid = Grid::default();
    for line in input.trim().lines() {
        let (action, range) = parse_instruction(line)?;
        match action {
            Action::TurnOff => grid.decrease_by(1, range),
            Action::TurnOn => grid.increase_by(1, range),
            Action::Toggle => grid.increase_by(2, range),
        }
    }
    Ok(grid.count())
}

fn parse_instruction(line: &str) -> Result<(Action, Range)> {
    let (action, from, to) = if line.starts_with("turn off") {
        let (from, to) = parse_range(line, 2)?;
        (Action::TurnOff, from, to)
    } else if line.starts_with("turn on ") {
        let (from, to) = parse_range(line, 2)?;
        (Action::TurnOn, from, to)
    } else if line.starts_with("toggle") {
        let (from, to) = parse_range(line, 1)?;
        (Action::Toggle, from, to)
    } else {
        return Err(format!("invalid instruction: '{}'", line).into());
    };
    let (xmin, ymin) = parse_indices(from)?;
    let (xmax, ymax) = parse_indices(to)?;
    let range = Range { xmin, xmax, ymin, ymax };
    Ok((action, range))
}

fn parse_range(instruction: &str, skip: usize) -> Result<(&str, &str)> {
    let mut iter = instruction.split_whitespace().skip(skip);
    let from = match iter.next() {
        Some(from) => from,
        None => return Err(format!("invalid instruction: '{}'", instruction).into()),
    };
    let to = match iter.nth(1) {
        Some(to) => to,
        None => return Err(format!("invalid instruction: '{}'", instruction).into()),
    };
    Ok((from, to))
}

fn parse_indices(str: &str) -> Result<(usize, usize)> {
    let (x, y) = match str.split_once(',') {
        Some((x, y)) => (x, y),
        None => return Err(format!("invalid indices: '{}'", str).into()),
    };
    let x = match x.parse() {
        Ok(x) => x,
        Err(_) => return Err(format!("invalid index: '{}'", x).into()),
    };
    let y = match y.parse() {
        Ok(y) => y,
        Err(_) => return Err(format!("invalid index: '{}'", x).into()),
    };
    Ok((x, y))
}

#[derive(Debug)]
struct Grid {
    lights: Vec<usize>,
}

impl Grid {
    fn turn_on(&mut self, range: Range) {
        for x in range.xmin..=range.xmax {
            for y in range.ymin..=range.ymax {
                let idx = x * 1000 + y;
                self.lights[idx] = 1
            }
        }
    }

    fn turn_off(&mut self, range: Range) {
        for x in range.xmin..=range.xmax {
            for y in range.ymin..=range.ymax {
                let idx = x * 1000 + y;
                self.lights[idx] = 0
            }
        }
    }

    fn toggle(&mut self, range: Range) {
        for x in range.xmin..=range.xmax {
            for y in range.ymin..=range.ymax {
                let idx = x * 1000 + y;
                self.lights[idx] = if self.lights[idx] == 1 { 0 } else { 1 }
            }
        }
    }

    fn increase_by(&mut self, step: usize, range: Range) {
        for x in range.xmin..=range.xmax {
            for y in range.ymin..=range.ymax {
                let idx = x * 1000 + y;
                self.lights[idx] += step
            }
        }
    }

    fn decrease_by(&mut self, step: usize, range: Range) {
        for x in range.xmin..=range.xmax {
            for y in range.ymin..=range.ymax {
                let idx = x * 1000 + y;
                self.lights[idx] = self.lights[idx].saturating_sub(step);
            }
        }
    }

    fn count(&self) -> usize {
        self.lights.iter().sum()
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self { lights: vec![0; 1_000_000] }
    }
}

#[derive(Debug)]
struct Range {
    xmin: usize,
    xmax: usize,
    ymin: usize,
    ymax: usize,
}

enum Action {
    TurnOff,
    TurnOn,
    Toggle,
}

#[test]
fn part_one_example1() -> Result<()> {
    assert_eq!(part_one("turn on 0,0 through 999,999")?, 1_000_000);
    Ok(())
}

#[test]
fn part_one_example2() -> Result<()> {
    assert_eq!(part_one("toggle 0,0 through 999,0")?, 1000);
    assert_eq!(part_one("toggle 0,0 through 999,0\ntoggle 0,0 through 999,0")?, 0);
    Ok(())
}

#[test]
fn part_one_example3() -> Result<()> {
    assert_eq!(part_one("turn on 0,0 through 999,999\nturn off 499,499 through 500,500")?, 1_000_000 - 4);
    Ok(())
}

#[test]
fn part_two_example1() -> Result<()> {
    assert_eq!(part_two("turn on 0,0 through 0,0")?, 1);
    Ok(())
}

#[test]
fn part_two_example2() -> Result<()> {
    assert_eq!(part_two("toggle 0,0 through 999,999")?, 2_000_000);
    Ok(())
}
