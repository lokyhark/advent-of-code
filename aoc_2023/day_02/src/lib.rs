use aoc::*;

pub const YEAR: u32 = 2023;
pub const DAY: u32 = 2;

pub fn part_one(input: &str) -> Result<u32> {
    let mut sum = 0;
    'game: for line in input.lines() {
        let (prefix, suffix) = line.trim().split_once(':').expect("invalid line");
        let (_, id) = prefix.trim().split_once(' ').expect("invalid line");
        let id: u32 = id.trim().parse().expect("invalid id number");
        for set in suffix.split(';') {
            for draw in set.split(',') {
                let (cubes, color) = draw.trim().split_once(' ').expect("invalid line");
                let cubes: u32 = cubes.parse().expect("invalid cubes number");
                let color = match color {
                    "red" => Color::Red,
                    "green" => Color::Green,
                    "blue" => Color::Blue,
                    _ => return err!("invalid color: {}", color),
                };
                if cubes > color.max() {
                    continue 'game;
                }
            }
        }
        sum += id;
    }
    Ok(sum)
}

pub fn part_two(input: &str) -> Result<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let (_, suffix) = line.trim().split_once(':').expect("invalid line");
        let mut bag = Bag::default();
        for set in suffix.split(';') {
            for draw in set.split(',') {
                let (cubes, color) = draw.trim().split_once(' ').expect("invalid line");
                let cubes: u32 = cubes.parse().expect("invalid cubes number");
                let color = match color {
                    "red" => Color::Red,
                    "green" => Color::Green,
                    "blue" => Color::Blue,
                    _ => return err!("invalid color: {}", color),
                };
                bag.update(cubes, color);
            }
        }
        sum += bag.power();
    }
    Ok(sum)
}

#[derive(Default)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    pub fn update(&mut self, number: u32, color: Color) {
        let count = match color {
            Color::Red => &mut self.red,
            Color::Green => &mut self.green,
            Color::Blue => &mut self.blue,
        };
        if number > *count {
            *count = number;
        }
    }

    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    pub fn max(&self) -> u32 {
        match self {
            Color::Red => 12,
            Color::Green => 13,
            Color::Blue => 14,
        }
    }
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_one(input)?, 8);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_two(input)?, 2286);
    Ok(())
}
