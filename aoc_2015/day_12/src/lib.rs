use regex::Regex;

use aoc::Result;
use serde_json::Value;

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 12;

pub fn part_one(input: &str) -> Result<i64> {
    let regex = Regex::new(r"-?\d+").unwrap();
    let sum = regex.find_iter(input.trim()).map(|m| m.as_str()).map(|x| x.parse::<i64>().unwrap()).sum();
    Ok(sum)
}

pub fn part_two(input: &str) -> Result<i64> {
    let json: Value = match serde_json::from_str(input.trim()) {
        Ok(json) => json,
        Err(_) => return Err("invalid JSON input".into()),
    };
    let mut stack = vec![json];
    let mut sum = 0;
    while !stack.is_empty() {
        let value = stack.pop().unwrap();
        match value {
            Value::Null | Value::Bool(_) | Value::String(_) => {}
            Value::Number(number) => match number.as_i64() {
                Some(number) => sum += number,
                None => return Err(format!("invalid JSON number: {}", number).into()),
            },
            Value::Array(values) => {
                stack.extend_from_slice(&values);
            }
            Value::Object(map) => {
                if !map.values().any(|v| v.as_str() == Some("red")) {
                    stack.extend(map.into_iter().map(|(_, v)| v))
                }
            }
        }
    }
    Ok(sum)
}

#[test]
pub fn part_one_example1() -> Result<()> {
    assert_eq!(part_one(r#"[1,2,3]"#)?, 6);
    assert_eq!(part_one(r#"{"a":2,"b":4}"#)?, 6);
    Ok(())
}

#[test]
pub fn part_one_example2() -> Result<()> {
    assert_eq!(part_one(r#"[[[3]]]"#)?, 3);
    assert_eq!(part_one(r#"{"a":{"b":4},"c":-1}"#)?, 3);
    Ok(())
}

#[test]
pub fn part_one_example3() -> Result<()> {
    assert_eq!(part_one(r#"{"a":[-1,1]}"#)?, 0);
    assert_eq!(part_one(r#"[-1,{"a":1}]"#)?, 0);
    Ok(())
}

#[test]
pub fn part_one_example4() -> Result<()> {
    assert_eq!(part_one(r#"[]"#)?, 0);
    assert_eq!(part_one(r#"{}"#)?, 0);
    Ok(())
}

#[test]
fn part_two_example1() -> Result<()> {
    assert_eq!(part_two(r#"[1,2,3]"#)?, 6);
    Ok(())
}

#[test]
fn part_two_example2() -> Result<()> {
    assert_eq!(part_two(r#"[1,{"c":"red","b":2},3]"#)?, 4);
    Ok(())
}

#[test]
fn part_two_example3() -> Result<()> {
    assert_eq!(part_two(r#"{"d":"red","e":[1,2,3,4],"f":5}"#)?, 0);
    Ok(())
}

#[test]
fn part_two_example4() -> Result<()> {
    assert_eq!(part_two(r#"[1,"red",5]"#)?, 6);
    Ok(())
}
