use aoc::Result;

pub const YEAR: u32 = 2016;
pub const DAY: u32 = 3;

pub fn part_one(input: &str) -> Result<usize> {
    let mut count = 0;
    for line in input.lines() {
        let mut split = line.split_ascii_whitespace();
        let mut sides = [0u32; 3];
        for idx in 0..3 {
            match split.next().map(|x| x.parse()) {
                Some(Ok(x)) => sides[idx] = x,
                Some(Err(_)) => return Err("invalid side length".into()),
                None => return Err("invalid triangle".into()),
            }
        }
        if is_triangle_possible(sides) {
            count += 1;
        }
    }
    Ok(count)
}

pub fn part_two(input: &str) -> Result<usize> {
    let mut count = 0;
    let mut lines = input.lines().peekable();
    while lines.peek().is_some() {
        let mut sides = [[0u32; 3]; 3];
        for i in 0..3 {
            let line = match lines.next() {
                Some(line) => line,
                None => return Err("invalid number of lines".into()),
            };
            let mut split = line.split_ascii_whitespace();
            for j in 0..3 {
                match split.next().map(|x| x.parse()) {
                    Some(Ok(x)) => sides[j][i] = x,
                    Some(Err(_)) => return Err("invalid side length".into()),
                    None => return Err("invalid triangle".into()),
                }
            }
        }
        for triangle in sides {
            if is_triangle_possible(triangle) {
                count += 1;
            }
        }
    }
    Ok(count)
}

fn is_triangle_possible(sides: [u32; 3]) -> bool {
    let perimeter: u32 = sides.iter().sum();
    sides.iter().all(|&side| side < perimeter - side)
}

#[test]
fn part_one_example() {
    assert!(!is_triangle_possible([5, 10, 25]));
}
