use std::{
    error::Error,
    fmt::Display,
    ops::{Index, IndexMut},
    str::FromStr,
    vec,
};

use aoc::Result;

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 18;

pub fn part_one(input: &str) -> Result<usize> {
    let mut grid: Grid = input.parse()?;
    for _ in 0..100 {
        grid = grid.step(false);
    }
    Ok(grid.count())
}

pub fn part_two(input: &str) -> Result<usize> {
    let mut grid: Grid = input.parse()?;
    for _ in 0..100 {
        grid = grid.step(true);
    }
    Ok(grid.count())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Grid {
    dim: usize,
    lights: Vec<bool>,
}

impl Grid {
    fn new(dim: usize) -> Self {
        Grid { dim, lights: vec![false; dim * dim] }
    }

    fn step(self, corners_stuck: bool) -> Self {
        let dim = self.dim;
        let mut new = Self::new(dim);
        let old = self;
        let last = dim - 1;
        for i in 0..dim {
            for j in 0..dim {
                let neighbors = match (i, j) {
                    // corners
                    // North-West
                    (0, 0) => vec![(1, 0), (1, 1), (0, 1)],
                    // North-East
                    (0, y) if y == last => vec![(0, y - 1), (1, y - 1), (1, y)],
                    // South-West
                    (x, 0) if x == last => vec![(x - 1, 0), (x - 1, 1), (x, 1)],
                    // South-East
                    (x, y) if x == last && y == last => vec![(x - 1, y), (x - 1, y - 1), (x, y - 1)],

                    // sides
                    // North
                    (0, y) => vec![(0, y - 1), (1, y - 1), (1, y), (1, y + 1), (0, y + 1)],
                    // West
                    (x, 0) => vec![(x - 1, 0), (x - 1, 1), (x, 1), (x + 1, 1), (x + 1, 0)],
                    // South
                    (x, y) if x == last => vec![(x, y - 1), (x - 1, y - 1), (x - 1, y), (x - 1, y + 1), (x, y + 1)],
                    // East
                    (x, y) if y == last => vec![(x - 1, y), (x - 1, y - 1), (x, y - 1), (x + 1, y - 1), (x + 1, y)],

                    // default
                    (x, y) => {
                        vec![(x - 1, y - 1), (x - 1, y), (x - 1, y + 1), (x, y - 1), (x, y + 1), (x + 1, y - 1), (x + 1, y), (x + 1, y + 1)]
                    }
                };
                match (i, j) {
                    (0, 0) if corners_stuck => new[(i, j)] = true,
                    (0, y) if corners_stuck && y == last => new[(i, j)] = true,
                    (x, 0) if corners_stuck && x == last => new[(i, j)] = true,
                    (x, y) if corners_stuck && x == last && y == last => new[(i, j)] = true,
                    (i, j) if old[(i, j)] => {
                        let count = neighbors.into_iter().filter(|&idx| old[idx]).count();
                        if count == 2 || count == 3 {
                            new[(i, j)] = true
                        };
                    }
                    (i, j) => {
                        if neighbors.into_iter().filter(|&idx| old[idx]).count() == 3 {
                            new[(i, j)] = true
                        }
                    }
                }
            }
        }
        new
    }

    fn count(&self) -> usize {
        self.lights.iter().filter(|x| **x).count()
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = bool;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let idx = index.0 * self.dim + index.1;
        &self.lights[idx]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let idx = index.0 * self.dim + index.1;
        &mut self.lights[idx]
    }
}

impl Display for Grid {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.dim {
            for j in 0..self.dim {
                if self[(i, j)] {
                    write!(fmt, "#")?;
                } else {
                    write!(fmt, ".")?;
                }
            }
            writeln!(fmt)?;
        }
        Ok(())
    }
}

impl FromStr for Grid {
    type Err = ParseGridError;

    fn from_str(string: &str) -> std::result::Result<Self, Self::Err> {
        let dim = string.trim().lines().count();
        let mut grid = Grid::new(dim);
        for (i, line) in string.trim().lines().enumerate() {
            for (j, byte) in line.as_bytes().iter().enumerate() {
                if *byte == b'#' {
                    grid[(i, j)] = true;
                    continue;
                }
                if *byte != b'.' {
                    return Err(ParseGridError);
                }
            }
        }
        Ok(grid)
    }
}

#[derive(Debug)]
struct ParseGridError;

impl Display for ParseGridError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "parse grid error")
    }
}

impl Error for ParseGridError {}

#[test]
fn part_one_example() -> Result<()> {
    let initial: Grid = include_str!("../input/initial_part_one.txt").parse()?;
    let step1: Grid = include_str!("../input/step1_part_one.txt").parse()?;
    let step2: Grid = include_str!("../input/step2_part_one.txt").parse()?;
    let step3: Grid = include_str!("../input/step3_part_one.txt").parse()?;
    let step4: Grid = include_str!("../input/step4_part_one.txt").parse()?;
    let grid = initial.step(false);
    assert_eq!(grid, step1);
    let grid = grid.step(false);
    assert_eq!(grid, step2);
    let grid = grid.step(false);
    assert_eq!(grid, step3);
    let grid = grid.step(false);
    assert_eq!(grid, step4);
    assert_eq!(grid.count(), 4);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let initial: Grid = include_str!("../input/initial_part_two.txt").parse()?;
    let step1: Grid = include_str!("../input/step1_part_two.txt").parse()?;
    let step2: Grid = include_str!("../input/step2_part_two.txt").parse()?;
    let step3: Grid = include_str!("../input/step3_part_two.txt").parse()?;
    let step4: Grid = include_str!("../input/step4_part_two.txt").parse()?;
    let step5: Grid = include_str!("../input/step5_part_two.txt").parse()?;
    let grid = initial.step(true);
    assert_eq!(grid, step1);
    let grid = grid.step(true);
    assert_eq!(grid, step2);
    let grid = grid.step(true);
    assert_eq!(grid, step3);
    let grid = grid.step(true);
    assert_eq!(grid, step4);
    let grid = grid.step(true);
    assert_eq!(grid, step5);
    assert_eq!(grid.count(), 17);
    Ok(())
}
