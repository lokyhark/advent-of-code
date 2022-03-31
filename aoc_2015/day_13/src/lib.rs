use std::collections::HashMap;

use itertools::Itertools;

use aoc::Result;

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 13;

pub fn part_one(input: &str) -> Result<i32> {
    let table = table(input)?;
    let optimal = optimal_seating_arrangment_happiness(table);
    Ok(optimal)
}

pub fn part_two(input: &str) -> Result<i32> {
    let mut table = table(input)?;
    let my_happiness = table.keys().map(|&k| (k, 0)).collect();
    table.insert("Me", my_happiness);
    for guest_happiness in table.values_mut() {
        guest_happiness.insert("Me", 0);
    }
    let optimal = optimal_seating_arrangment_happiness(table);
    Ok(optimal)
}

fn table(input: &str) -> Result<HashMap<&str, HashMap<&str, i32>>> {
    let mut table = HashMap::new();
    for line in input.trim().lines() {
        let line = line.trim_end_matches('.');
        let mut split = line.split_ascii_whitespace();
        let guest = match split.next() {
            Some(name) => table.entry(name).or_insert_with(HashMap::new),
            None => return Err("invalid input".into()),
        };
        let negative = match split.nth(1) {
            Some("gain") => false,
            Some("lose") => true,
            _ => return Err("invalid input".into()),
        };
        let happiness = match split.next().map(|x| x.parse::<i32>()) {
            Some(Ok(happiness)) => happiness,
            _ => return Err("invalid input".into()),
        };
        match split.next_back() {
            Some(name) => {
                if negative {
                    guest.insert(name, -happiness)
                } else {
                    guest.insert(name, happiness)
                }
            }
            None => return Err("invalid input".into()),
        };
    }
    Ok(table)
}

fn optimal_seating_arrangment_happiness(table: HashMap<&str, HashMap<&str, i32>>) -> i32 {
    let mut optimal = 0;
    for arrangement in table.keys().permutations(table.len()) {
        let mut happiness = 0;
        for (first, second) in arrangement.into_iter().circular_tuple_windows() {
            happiness += table.get(first).unwrap().get(second).unwrap();
            happiness += table.get(second).unwrap().get(first).unwrap();
        }
        if happiness > optimal {
            optimal = happiness
        }
    }
    optimal
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_one(input)?, 330);
    Ok(())
}
