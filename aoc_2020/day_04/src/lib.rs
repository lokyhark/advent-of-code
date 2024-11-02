use std::collections::HashMap;

use aoc::Result;

pub const YEAR: u32 = 2020;
pub const DAY: u32 = 4;

pub fn part_one(input: &str) -> Result<usize> {
    let passports = parse_passports(input);
    let count = passports.iter().filter(|x| x.valid1()).count();
    Ok(count)
}

pub fn part_two(input: &str) -> Result<usize> {
    let passports = parse_passports(input);
    let count = passports.iter().filter(|x| x.valid2()).count();
    Ok(count)
}

fn parse_passports(input: &str) -> Vec<Passport> {
    let mut passports = Vec::new();
    let lines = input.lines();
    let mut passport = Passport::default();
    for line in lines {
        if line.trim().is_empty() {
            passports.push(passport);
            passport = Passport::default();
        }
        for data in line.split_ascii_whitespace() {
            let (field, value) = data.split_once(':').expect("invalid passport");
            let old = match field {
                "byr" => passport.fields.insert("byr", value),
                "iyr" => passport.fields.insert("iyr", value),
                "eyr" => passport.fields.insert("eyr", value),
                "pid" => passport.fields.insert("pid", value),
                "hgt" => passport.fields.insert("hgt", value),
                "hcl" => passport.fields.insert("hcl", value),
                "ecl" => passport.fields.insert("ecl", value),
                "cid" => passport.fields.insert("cid", value),
                _ => panic!("invalid field {}", field),
            };
            if let Some(value) = old {
                panic!("duplicate field {}: {}", field, value);
            }
        }
    }
    passports.push(passport);
    passports
}

#[derive(Debug, Default)]
struct Passport<'input> {
    fields: HashMap<&'input str, &'input str>,
}

impl<'a> Passport<'a> {
    fn valid1(&self) -> bool {
        for field in ["byr", "iyr", "eyr", "pid", "hgt", "hcl", "ecl"] {
            if !self.fields.contains_key(field) {
                return false;
            }
        }
        true
    }

    fn valid2(&self) -> bool {
        match self.fields.get("byr") {
            None => return false,
            Some(value) => {
                let year: u32 = value.parse().expect("invalid birth year");
                if !(1920..=2002).contains(&year) {
                    return false;
                }
            }
        }
        match self.fields.get("iyr") {
            None => return false,
            Some(value) => {
                let year = value.parse().expect("invalid issue year");
                if !(2010..=2020).contains(&year) {
                    return false;
                }
            }
        }
        match self.fields.get("eyr") {
            None => return false,
            Some(value) => {
                let year = value.parse().expect("invalid expiration year");
                if !(2020..=2030).contains(&year) {
                    return false;
                }
            }
        }
        match self.fields.get("hgt") {
            None => return false,
            Some(value) => {
                let len = match value.chars().position(|x| !x.is_ascii_digit()) {
                    Some(pos) => pos,
                    None => return false,
                };
                let height = value[..len].parse().expect("invalid height");
                let len = value.len();
                let unit = &value[(len - 2)..len];
                match unit {
                    "cm" => {
                        if !(150..=193).contains(&height) {
                            return false;
                        }
                    }
                    "in" => {
                        if !(59..=76).contains(&height) {
                            return false;
                        }
                    }
                    _ => panic!("invalid unit {}", unit),
                }
            }
        }
        match self.fields.get("hcl") {
            None => return false,
            Some(value) => match value.len() {
                7 => {
                    if !value.starts_with('#') {
                        return false;
                    }
                    if !value[1..].chars().all(|x| x.is_ascii_hexdigit()) {
                        return false;
                    }
                }
                _ => return false,
            },
        }
        match self.fields.get("ecl") {
            None => return false,
            Some(value) => {
                if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(value) {
                    return false;
                }
            }
        }
        match self.fields.get("pid") {
            None => return false,
            Some(value) => match value.len() {
                9 => {
                    if !value.chars().all(|x| x.is_ascii_digit()) {
                        return false;
                    }
                }
                _ => return false,
            },
        }
        true
    }
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_one(input)?, 2);
    Ok(())
}

#[test]
fn part_two_example1() -> Result<()> {
    let input = include_str!("../input/invalid.txt");
    let passports = parse_passports(input);
    assert!(passports.iter().all(|p| !p.valid2()));
    Ok(())
}

#[test]
fn part_two_example2() -> Result<()> {
    let input = include_str!("../input/valid.txt");
    let passports = parse_passports(input);
    assert!(passports.iter().all(|p| p.valid2()));
    Ok(())
}
