use std::{cmp::min, collections::HashMap};

use aoc::Result;

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 14;

pub fn part_one(input: &str) -> Result<u32> {
    let reindeers = reindeers(input)?;
    Ok(race(&reindeers, 2503))
}

pub fn part_two(input: &str) -> Result<u32> {
    let reindeers = reindeers(input)?;
    Ok(point(&reindeers, 2503))
}

fn reindeers(input: &str) -> Result<Vec<Reindeer>> {
    let mut reindeers = Vec::new();
    for line in input.trim().lines() {
        let mut split = line.split_ascii_whitespace();
        let speed = match split.nth(3).map(|x| x.parse::<u32>()) {
            Some(Ok(speed)) => speed,
            _ => return Err("invalid input".into()),
        };
        let fly_time = match split.nth(2).map(|x| x.parse::<u32>()) {
            Some(Ok(time)) => time,
            _ => return Err("invalid input".into()),
        };
        let rest_time = match split.nth_back(1).map(|x| x.parse::<u32>()) {
            Some(Ok(time)) => time,
            _ => return Err("invalid input".into()),
        };
        let reindeer = Reindeer { speed, fly_time, rest_time };
        reindeers.push(reindeer);
    }
    if reindeers.is_empty() {
        return Err("invalid input".into());
    }
    Ok(reindeers)
}

fn race(reindeers: &[Reindeer], time: u32) -> u32 {
    let mut max = 0;
    for reindeer in reindeers {
        let mut chrono = 0;
        let mut distance = 0;
        while chrono < time {
            let fly_time = min(reindeer.fly_time, time - chrono);
            distance += reindeer.speed * fly_time;
            chrono += fly_time + reindeer.rest_time;
        }
        if distance > max {
            max = distance;
        }
    }
    max
}

fn point(reindeers: &[Reindeer], time: u32) -> u32 {
    let mut chrono = 0;
    let mut scores: HashMap<_, _> = reindeers.iter().map(|x| (x, 0)).collect();
    let mut ranking: HashMap<_, _> = reindeers.iter().map(|x| (x, 0)).collect();
    let mut status: HashMap<_, _> = reindeers.iter().map(|x| (x, Status::Flying(0))).collect();
    while chrono < time {
        chrono += 1;
        for reindeer in reindeers {
            let status = status.get_mut(reindeer).unwrap();
            let distance = ranking.get_mut(reindeer).unwrap();
            match status {
                Status::Flying(x) if *x == reindeer.fly_time => {
                    *status = Status::Resting(1);
                }
                Status::Flying(x) => {
                    *distance += reindeer.speed;
                    *x += 1;
                }
                Status::Resting(x) if *x == reindeer.rest_time => {
                    *distance += reindeer.speed;
                    *status = Status::Flying(1);
                }
                Status::Resting(x) => {
                    *x += 1;
                }
            }
        }
        let max = ranking.values().max().unwrap();
        for reindeer in ranking.iter().filter(|(_, distance)| *distance == max).map(|(k, _)| k) {
            *scores.get_mut(reindeer).unwrap() += 1;
        }
    }
    *scores.values().max().unwrap()
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Reindeer {
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

enum Status {
    Flying(u32),
    Resting(u32),
}

#[test]
fn part_one_example() {
    let comet = Reindeer {
        speed: 14,
        fly_time: 10,
        rest_time: 127,
    };
    let dancer = Reindeer {
        speed: 16,
        fly_time: 11,
        rest_time: 162,
    };
    assert_eq!(race(&[comet, dancer], 1000), 1120)
}

#[test]
fn part_two_example() {
    let comet = Reindeer {
        speed: 14,
        fly_time: 10,
        rest_time: 127,
    };
    let dancer = Reindeer {
        speed: 16,
        fly_time: 11,
        rest_time: 162,
    };
    assert_eq!(point(&[comet, dancer], 1000), 689)
}
