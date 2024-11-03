use aoc::*;

pub const YEAR: u32 = 2022;
pub const DAY: u32 = 8;

pub fn part_one(input: &str) -> Result<usize> {
    let forest = parse_input(input)?;
    Ok(forest.visible())
}

pub fn part_two(input: &str) -> Result<usize> {
    let forest = parse_input(input)?;
    Ok(forest.view())
}

fn parse_input(input: &str) -> Result<Forest> {
    let mut trees = Vec::new();
    let mut size = 0;
    for line in input.lines() {
        for char in line.chars() {
            let height = char.to_digit(10).map(|x| x as u8).ok_or(error!("invalid forest"))?;
            trees.push(height);
        }
        size += 1;
    }
    Ok(Forest::new(size, trees))
}

#[derive(Debug)]
struct Forest {
    size: usize,
    trees: Vec<u8>,
}

impl Forest {
    fn new(size: usize, trees: Vec<u8>) -> Self {
        Self { size, trees }
    }

    fn visible(&self) -> usize {
        self.size.pow(2) - self.hidden()
    }

    fn hidden(&self) -> usize {
        let mut count = 0;
        for (pos, &height) in self.trees.iter().enumerate() {
            let start = pos / self.size * self.size;
            let end = pos;
            let left = &self.trees[start..end];
            if !hidden(left, height) {
                continue;
            }

            let start = pos + 1;
            let end = (pos / self.size + 1) * self.size;
            let right = &self.trees[start..end];
            if !hidden(right, height) {
                continue;
            }

            let row = pos / self.size;
            let col = pos % self.size;
            let up: Vec<_> = self.trees.iter().skip(col).step_by(self.size).take(row).copied().collect();
            if !hidden(&up, height) {
                continue;
            }

            let row = pos / self.size;
            let col = pos % self.size;
            let down: Vec<_> = self.trees.iter().skip((row + 1) * self.size + col).step_by(self.size).copied().collect();
            if !hidden(&down, height) {
                continue;
            }

            count += 1;
        }
        count
    }

    fn view(&self) -> usize {
        let mut max = 0;
        for (pos, &height) in self.trees.iter().enumerate() {
            let start = pos / self.size * self.size;
            let end = pos;
            let mut left = self.trees[start..end].to_vec();
            left.reverse();
            let left = view(&left, height);

            let start = pos + 1;
            let end = (pos / self.size + 1) * self.size;
            let right = &self.trees[start..end];
            let right = view(right, height);

            let row = pos / self.size;
            let col = pos % self.size;
            let mut up: Vec<_> = self.trees.iter().skip(col).step_by(self.size).take(row).copied().collect();
            up.reverse();
            let up = view(&up, height);

            let row = pos / self.size;
            let col = pos % self.size;
            let down: Vec<_> = self.trees.iter().skip((row + 1) * self.size + col).step_by(self.size).copied().collect();
            let down = view(&down, height);

            max = max.max(left * right * up * down);
        }
        max
    }
}

fn hidden(slice: &[u8], height: u8) -> bool {
    !(slice.iter().all(|x| *x < height) || slice.is_empty())
}

fn view(slice: &[u8], height: u8) -> usize {
    let mut view = 0;
    for tree in slice.iter() {
        view += 1;
        if *tree >= height {
            break;
        }
    }
    view
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_one(input)?, 21);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_two(input)?, 8);
    Ok(())
}
