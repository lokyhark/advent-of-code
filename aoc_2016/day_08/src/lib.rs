use aoc::*;

pub const YEAR: u32 = 2016;
pub const DAY: u32 = 8;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

pub fn part_one(input: &str) -> Result<usize> {
    let instructions = parse(input)?;
    let mut grid = Grid::new(WIDTH, HEIGHT);
    for instruction in instructions {
        grid.apply(instruction);
    }
    grid.print();
    Ok(grid.count())
}

fn parse(input: &str) -> Result<Vec<Instruction>> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        let mut split = line.split_ascii_whitespace();
        let instruction = match split.next() {
            None => return err!("invalid input"),
            Some("rect") => match split.next() {
                Some(shape) => {
                    let (width, height) = match shape.split_once('x') {
                        Some((width, height)) => {
                            let width = match width.parse::<usize>() {
                                Ok(width) => width,
                                Err(_) => return err!("invalid rect width: {}", width),
                            };
                            let height = match height.parse() {
                                Ok(height) => height,
                                Err(_) => return err!("invalid rect height: {}", height),
                            };
                            (width, height)
                        }
                        None => return err!("invalid rect instruction"),
                    };
                    Instruction::Rectangle { width, height }
                }
                None => return err!("invalid rect instruction"),
            },
            Some("rotate") => match split.next() {
                None => return err!("invalid rotate instruction"),
                kind @ Some("row") | kind @ Some("column") => {
                    let index = match split.next() {
                        None => return err!("invalid rotate instruction"),
                        Some(index) => match index.split_once('=') {
                            Some((_, index)) => match index.parse::<usize>() {
                                Ok(index) => index,
                                Err(_) => return err!("invalid index: {}", index),
                            },
                            None => return err!("invalid rotate instruction"),
                        },
                    };
                    let shift = match split.nth(1) {
                        None => return err!("invalid rotate instruction"),
                        Some(shift) => match shift.parse::<usize>() {
                            Ok(shift) => shift,
                            Err(_) => return err!("invalid shift: {}", shift),
                        },
                    };
                    match kind {
                        Some("row") => Instruction::RowShift { index, shift },
                        Some("column") => Instruction::ColShift { index, shift },
                        _ => unreachable!(),
                    }
                }
                Some(x) => return err!("invalid rotate kind : {}", x),
            },
            Some(x) => return err!("invalid instruction: {}", x),
        };
        instructions.push(instruction);
    }
    Ok(instructions)
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    pixels: Vec<bool>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        assert!(width > 0);
        assert!(height > 0);
        Self {
            width,
            height,
            pixels: vec![false; width * height],
        }
    }

    fn count(&self) -> usize {
        self.pixels.iter().filter(|x| **x).count()
    }

    fn apply(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Rectangle { width, height } => {
                for row in 0..height {
                    for col in 0..width {
                        self.set(row, col, true);
                    }
                }
            }
            Instruction::RowShift { index, shift } => {
                assert!(shift < self.width);
                let row = index;
                let mut vec = vec![false; self.width];
                for old in 0..self.width {
                    let new = (old + shift) % self.width;
                    vec[new] = self.get(row, old);
                }
                for (col, value) in vec.iter().enumerate().take(self.width) {
                    self.set(row, col, *value);
                }
            }
            Instruction::ColShift { index, shift } => {
                assert!(shift < self.height);
                let col = index;
                let mut vec = vec![false; self.height];
                for old in 0..self.height {
                    let new = (old + shift) % self.height;
                    vec[new] = self.get(old, col);
                }
                for (row, value) in vec.iter().enumerate().take(self.height) {
                    self.set(row, col, *value)
                }
            }
        }
    }

    fn get(&self, row: usize, col: usize) -> bool {
        self.pixels[row * self.width + col]
    }

    fn set(&mut self, row: usize, col: usize, on: bool) {
        self.pixels[row * self.width + col] = on
    }

    fn print(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                if self.get(row, col) {
                    print!("#");
                } else {
                    print!(".")
                }
            }
            println!()
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Rectangle { width: usize, height: usize },
    RowShift { index: usize, shift: usize },
    ColShift { index: usize, shift: usize },
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    let instructions = parse(input)?;
    let mut grid = Grid::new(7, 3);
    for instruction in instructions {
        grid.apply(instruction);
    }
    assert_eq!(grid.count(), 6);
    Ok(())
}
