use std::collections::HashMap;

use aoc::*;

pub const YEAR: u32 = 2021;
pub const DAY: u32 = 4;

pub fn part_one(input: &str) -> Result<u32> {
    let (mut stack, mut grids) = parse_input(input)?;
    while let Some(draw) = stack.pop() {
        for grid in &mut grids {
            grid.mark(draw);
            if grid.bingo() {
                return Ok(grid.score(draw));
            }
        }
    }
    err!("no bingo")
}

pub fn part_two(input: &str) -> Result<u32> {
    let (mut stack, mut grids) = parse_input(input)?;
    while let Some(draw) = stack.pop() {
        for grid in &mut grids {
            grid.mark(draw);
        }
        grids.retain(|g| !g.bingo());
        if grids.len() == 1 {
            let grid = &mut grids[0];
            while let Some(draw) = stack.pop() {
                grid.mark(draw);
                if grid.bingo() {
                    return Ok(grids[0].score(draw));
                }
            }
        }
    }
    err!("no bingo")
}

fn parse_input(input: &str) -> Result<(Vec<u32>, Vec<Grid>)> {
    let mut lines = input.lines().peekable();
    let mut stack: Vec<_> = match lines.next() {
        None => return err!("invalid input"),
        Some(line) => match line.split(',').map(|x| x.parse()).collect() {
            Ok(stack) => stack,
            Err(_) => return err!("invalid input"),
        },
    };
    stack.reverse();
    lines.next();
    let mut grids = Vec::new();
    while let Some(line) = lines.peek() {
        let size = line.split_ascii_whitespace().count();
        let mut grid = Grid::new(size);
        for row in 0..size {
            let line = match lines.next() {
                Some(line) => line,
                None => return err!("invalid grid"),
            };
            for (col, value) in line.split_ascii_whitespace().enumerate() {
                match value.parse::<u32>() {
                    Ok(value) => {
                        grid.insert(row, col, value)?;
                    }
                    Err(_) => return err!("invalid grid"),
                };
            }
        }
        grids.push(grid);
        lines.next();
    }
    Ok((stack, grids))
}

#[derive(Debug)]
struct Grid {
    size: usize,
    slots: HashMap<u32, Slot>,
}

impl Grid {
    fn new(size: usize) -> Self {
        Self { size, slots: Default::default() }
    }

    fn insert(&mut self, row: usize, col: usize, value: u32) -> Result<()> {
        if row > self.size || col >= self.size {
            return err!("invalid slot ({}, {})", row, col);
        }
        let slot = Slot { row, col, mark: false };
        if self.slots.insert(value, slot).is_some() {
            return err!("duplicate value in grid: {}", value);
        }
        Ok(())
    }

    fn mark(&mut self, value: u32) {
        if let Some(slot) = self.slots.get_mut(&value) {
            slot.mark = true
        }
    }

    fn bingo(&self) -> bool {
        // Check rows
        for row in 0..self.size {
            if self.slots.values().filter(|x| x.row == row).all(|x| x.mark) {
                return true;
            }
        }
        // Check cols
        for col in 0..self.size {
            if self.slots.values().filter(|x| x.col == col).all(|x| x.mark) {
                return true;
            }
        }
        false
    }

    fn score(&self, number: u32) -> u32 {
        self.slots.iter().filter(|(_, slot)| !slot.mark).map(|x| x.0).sum::<u32>() * number
    }
}

#[derive(Debug)]
struct Slot {
    row: usize,
    col: usize,
    mark: bool,
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_one(input)?, 4512);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_two(input)?, 1924);
    Ok(())
}
