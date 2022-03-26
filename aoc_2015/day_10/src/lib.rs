use aoc::Result;

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 10;

pub fn part_one(input: &str) -> Result<usize> {
    let mut input = input.trim().to_string();
    for _ in 0..40 {
        input = look_and_say(&input);
    }
    Ok(input.len())
}

pub fn part_two(input: &str) -> Result<usize> {
    let mut input = input.trim().to_string();
    for _ in 0..50 {
        input = look_and_say(&input);
    }
    Ok(input.len())
}

fn look_and_say(input: &str) -> String {
    let mut string = String::new();
    let mut iter = input.chars().peekable();
    let mut count = 0;
    while iter.peek().is_some() {
        let char = iter.next().unwrap();
        count += 1;
        while iter.peek() == Some(&char) {
            count += 1;
            iter.next();
        }
        string.push_str(&format!("{}{}", count, char));
        count = 0;
    }
    string
}

#[test]
fn example1() {
    assert_eq!(look_and_say("1"), "11");
}

#[test]
fn example2() {
    assert_eq!(look_and_say("11"), "21");
}

#[test]
fn example3() {
    assert_eq!(look_and_say("21"), "1211");
}

#[test]
fn example4() {
    assert_eq!(look_and_say("1211"), "111221");
}

#[test]
fn example5() {
    assert_eq!(look_and_say("111221"), "312211");
}
