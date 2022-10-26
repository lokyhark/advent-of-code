use aoc::Result;

pub const YEAR: u32 = 2016;
pub const DAY: u32 = 2;

pub fn part_one(input: &str) -> Result<String> {
    let mut keypad = Keypad1::default();
    let mut code = String::new();
    for line in input.trim().lines() {
        for byte in line.trim().bytes() {
            match byte {
                b'U' => keypad.up(),
                b'D' => keypad.down(),
                b'L' => keypad.left(),
                b'R' => keypad.right(),
                _ => return Err(format!("invalid move: '{}'", byte as char).into()),
            }
        }
        code.push_str(&keypad.0.to_string());
    }
    Ok(code)
}

pub fn part_two(input: &str) -> Result<String> {
    let mut keypad = Keypad2::default();
    let mut code = Vec::new();
    for line in input.trim().lines() {
        for byte in line.trim().bytes() {
            match byte {
                b'U' => keypad.up(),
                b'D' => keypad.down(),
                b'L' => keypad.left(),
                b'R' => keypad.right(),
                _ => return Err(format!("invalid move: '{}'", byte as char).into()),
            }
        }
        code.push(keypad.0);
    }
    match String::from_utf8(code) {
        Ok(code) => Ok(code),
        Err(_) => unreachable!(),
    }
}

struct Keypad1(u8);

impl Keypad1 {
    fn up(&mut self) {
        match self.0 {
            1 | 2 | 3 => {}
            4 | 5 | 6 | 7 | 8 | 9 => self.0 -= 3,
            x => unreachable!("invalid position {}", x as char),
        }
    }

    fn down(&mut self) {
        match self.0 {
            7 | 8 | 9 => {}
            1 | 2 | 3 | 4 | 5 | 6 => self.0 += 3,
            x => unreachable!("invalid position {}", x as char),
        }
    }

    fn left(&mut self) {
        match self.0 {
            1 | 4 | 7 => {}
            2 | 3 | 5 | 6 | 8 | 9 => self.0 -= 1,
            x => unreachable!("invalid position {}", x as char),
        }
    }

    fn right(&mut self) {
        match self.0 {
            3 | 6 | 9 => {}
            1 | 2 | 4 | 5 | 7 | 8 => self.0 += 1,
            x => unreachable!("invalid position {}", x as char),
        }
    }
}

impl Default for Keypad1 {
    fn default() -> Self {
        Self(5)
    }
}

struct Keypad2(u8);

impl Keypad2 {
    fn up(&mut self) {
        match self.0 {
            b'6' => self.0 = b'2',
            b'A' => self.0 = b'6',
            b'D' => self.0 = b'B',
            b'B' => self.0 = b'7',
            b'7' => self.0 = b'3',
            b'3' => self.0 = b'1',
            b'C' => self.0 = b'8',
            b'8' => self.0 = b'4',
            b'5' | b'2' | b'1' | b'4' | b'9' => {}
            x => unreachable!("invalid position {}", x as char),
        }
    }

    fn down(&mut self) {
        match self.0 {
            b'2' => self.0 = b'6',
            b'6' => self.0 = b'A',
            b'1' => self.0 = b'3',
            b'3' => self.0 = b'7',
            b'7' => self.0 = b'B',
            b'B' => self.0 = b'D',
            b'4' => self.0 = b'8',
            b'8' => self.0 = b'C',
            b'5' | b'A' | b'D' | b'C' | b'9' => {}
            x => unreachable!("invalid position {}", x as char),
        }
    }

    fn right(&mut self) {
        match self.0 {
            b'2' | b'3' | b'5' | b'6' | b'7' | b'8' => self.0 += 1,
            b'A' | b'B' => self.0 += 1,
            b'1' | b'4' | b'9' | b'C' | b'D' => {}
            x => unreachable!("invalid position {}", x as char),
        }
    }

    fn left(&mut self) {
        match self.0 {
            b'3' | b'4' | b'6' | b'7' | b'8' | b'9' => self.0 -= 1,
            b'B' | b'C' => self.0 -= 1,
            b'1' | b'2' | b'5' | b'A' | b'D' => {}
            x => unreachable!("invalid position {}", x as char),
        }
    }
}

impl Default for Keypad2 {
    fn default() -> Self {
        Self(b'5')
    }
}

#[test]
fn part_one_example() -> Result<()> {
    let input = ["ULL", "RRDDD", "LURDL", "UUUUD"].join("\n");
    assert_eq!(part_one(&input)?, "1985");
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let input = ["ULL", "RRDDD", "LURDL", "UUUUD"].join("\n");
    assert_eq!(part_two(&input)?, "5DB3");
    Ok(())
}
