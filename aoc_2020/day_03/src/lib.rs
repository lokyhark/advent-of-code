use aoc::Result;

pub const YEAR: u32 = 2020;
pub const DAY: u32 = 3;

pub fn part_one(input: &str) -> Result<usize> {
    let map = parse_map(input.trim());
    let mut pos = Position::default();
    let mut trees = 0;
    while pos.row < map.rows() {
        if let Kind::Tree = map.get(pos) {
            trees += 1
        }
        pos.slope(1, 3);
    }
    Ok(trees)
}

pub fn part_two(input: &str) -> Result<usize> {
    let map = parse_map(input.trim());
    let mut count = 1;
    for slope in [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)] {
        let mut pos = Position::default();
        let mut trees = 0;
        while pos.row < map.rows() {
            if let Kind::Tree = map.get(pos) {
                trees += 1
            }
            pos.slope(slope.0, slope.1);
        }
        count *= trees;
    }
    Ok(count)
}

fn parse_map(input: &str) -> Map {
    let mut bytes = Vec::new();
    let mut lines = input.lines();
    let line = lines.next().expect("invalid map");
    let cols = line.len();
    bytes.extend(line.bytes());
    let mut rows = 1;
    for line in lines {
        bytes.extend(line.bytes());
        rows += 1;
    }
    let shape = (rows, cols);
    Map { bytes, shape }
}

#[derive(Debug)]
struct Map {
    bytes: Vec<u8>,
    shape: (usize, usize),
}

impl Map {
    fn rows(&self) -> usize {
        self.shape.0
    }

    fn get(&self, position: Position) -> Kind {
        assert!(position.row < self.shape.0);
        let start = position.row * self.shape.1;
        let col = position.col % self.shape.1;
        match self.bytes.get(start + col) {
            Some(byte) => match byte {
                b'.' => Kind::Open,
                b'#' => Kind::Tree,
                _ => panic!("invalid kind"),
            },
            None => panic!("invalid position"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn slope(&mut self, rows: usize, cols: usize) {
        self.row += rows;
        self.col += cols;
    }
}

#[derive(Debug)]
enum Kind {
    Open,
    Tree,
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_one(input)?, 7);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_two(input)?, 336);
    Ok(())
}
