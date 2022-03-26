use aoc::Result;

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 11;

pub fn part_one(input: &str) -> Result<String> {
    let mut password = input.trim().to_string();
    while !meet_requirements(&password) {
        password = next_password(&password)?;
    }
    Ok(password)
}

pub fn part_two(input: &str) -> Result<String> {
    let mut password = input.trim().to_string();
    while !meet_requirements(&password) {
        password = next_password(&password)?;
    }
    password = next_password(&password)?;
    while !meet_requirements(&password) {
        password = next_password(&password)?;
    }
    Ok(password)
}

fn next_password(password: &str) -> Result<String> {
    let mut bytes: Vec<_> = Vec::from(password.as_bytes());
    if let Some(idx) = bytes.iter().position(|&b| b == b'i' || b == b'l' || b == b'o') {
        bytes[idx] += 1;
        for byte in bytes[idx + 1..].iter_mut() {
            *byte = b'a'
        }
        return Ok(String::from_utf8(bytes.to_vec()).unwrap());
    }
    let mut new = Vec::new();
    let mut carry = true;
    for byte in bytes.iter().rev() {
        if carry {
            carry = false;
            match byte {
                x @ (b'a'..=b'g' | b'i'..=b'j' | b'l'..=b'm' | b'o'..=b'y') => new.push(*x + 1),
                x @ (b'h' | b'k' | b'n') => new.push(*x + 2),
                b'z' => {
                    carry = true;
                    new.push(b'a');
                }
                _ => return Err(format!("invalid bytes: '{}'", password).into()),
            }
        } else {
            match byte {
                x @ (b'a'..=b'h' | b'j'..=b'k' | b'm'..=b'n' | b'p'..=b'z') => new.push(*x),
                x @ (b'i' | b'l' | b'o') => new.push(*x + 1),
                _ => return Err(format!("invalid bytes: '{}'", password).into()),
            }
        }
    }
    Ok(String::from_utf8(new.into_iter().rev().collect()).unwrap())
}

fn meet_requirements(password: &str) -> bool {
    first_requirement(password) && second_requirement(password) && third_requirement(password)
}

fn first_requirement(password: &str) -> bool {
    password.as_bytes().windows(3).any(|x| x[1] == x[0] + 1 && x[2] == x[1] + 1)
}

fn second_requirement(password: &str) -> bool {
    !password.as_bytes().iter().any(|&x| x == b'i' || x == b'l' || x == b'o')
}

fn third_requirement(password: &str) -> bool {
    let mut count = 0;
    let mut iter = password.as_bytes().windows(2);
    while let Some(x) = iter.next() {
        if x[0] == x[1] {
            count += 1;
            iter.next();
        }
    }
    count >= 2
}

#[test]
fn example1() {
    assert!(first_requirement("hijklmmn"));
    assert!(!second_requirement("hijklmmn"));
}

#[test]
fn example2() {
    assert!(!first_requirement("abbceffg"));
    assert!(third_requirement("abbceffg"));
}

#[test]
fn example3() {
    assert!(!third_requirement("abbcegjk"));
}

#[test]
fn example4() -> Result<()> {
    assert_eq!(part_one("abcdefgh")?, "abcdffaa");
    Ok(())
}

#[test]
fn example5() -> Result<()> {
    assert_eq!(part_one("ghijklmn")?, "ghjaabcc");
    Ok(())
}
