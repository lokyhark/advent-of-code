use std::collections::HashMap;

use itertools::Itertools;

use aoc::Result;

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 9;

pub fn part_one(input: &str) -> Result<u32> {
    let cities = cities(input)?;
    let routes = routes(cities)?;
    match routes.into_iter().min() {
        Some(min) => Ok(min),
        None => Err("no route found".into()),
    }
}

pub fn part_two(input: &str) -> Result<u32> {
    let cities = cities(input)?;
    let routes = routes(cities)?;
    match routes.into_iter().max() {
        Some(max) => Ok(max),
        None => Err("no route found".into()),
    }
}

fn cities(input: &str) -> Result<HashMap<&str, HashMap<&str, u32>>> {
    let mut cities = HashMap::new();
    for line in input.lines() {
        let (path, distance) = match line.split_once('=') {
            Some((path, distance)) => {
                let path = path.trim();
                let distance = match distance.trim().parse::<u32>() {
                    Ok(x) => x,
                    Err(_) => return Err(format!("invalid path: '{}'", line).into()),
                };
                (path, distance)
            }
            None => return Err(format!("invalid path: '{}'", line).into()),
        };
        let (from, to) = match path.split_once("to") {
            Some((from, to)) => (from.trim(), to.trim()),
            None => return Err(format!("invalid path: '{}'", line).into()),
        };
        let map = cities.entry(from).or_insert_with(HashMap::new);
        map.insert(to, distance);
        let map = cities.entry(to).or_insert_with(HashMap::new);
        map.insert(from, distance);
    }
    Ok(cities)
}

fn routes(cities: HashMap<&str, HashMap<&str, u32>>) -> Result<Vec<u32>> {
    let mut routes = Vec::new();
    for path in cities.keys().permutations(cities.len()) {
        let route = path.windows(2).fold(0, |acc, x| acc + cities.get(x[0]).unwrap().get(x[1]).unwrap());
        routes.push(route);
    }
    Ok(routes)
}

#[test]
fn part_one_example() -> Result<()> {
    let input = "\
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
";
    assert_eq!(part_one(input)?, 605);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let input = "\
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
";
    assert_eq!(part_two(input)?, 982);
    Ok(())
}
