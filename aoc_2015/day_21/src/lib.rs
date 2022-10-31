use aoc::Result;
use itertools::Itertools;

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 21;

pub fn part_one(input: &str) -> Result<u32> {
    let boss = parse_boss(input)?;
    let initial = Player { points: 100, ..Default::default() };
    let mut shop = parse_shop()?;
    shop.armor.push(Item { cost: 0, damage: 0, armor: 0 });
    shop.rings.push(Item { cost: 0, damage: 0, armor: 0 });
    shop.rings.push(Item { cost: 0, damage: 0, armor: 0 });
    let mut min = u32::MAX;
    for weapon in shop.weapons.iter() {
        for armor in shop.armor.iter() {
            for rings in shop.rings.iter().combinations(2) {
                let mut player = initial.clone();
                player.damage += weapon.damage;
                player.armor += armor.armor;
                let left = rings[0];
                let right = rings[1];
                player.damage += left.damage + right.damage;
                player.armor += left.armor + right.armor;
                let gold = weapon.cost + armor.cost + left.cost + right.cost;
                if gold > min {
                    continue;
                } else if play(&player, &boss) {
                    min = gold;
                }
            }
        }
    }
    Ok(min)
}

pub fn part_two(input: &str) -> Result<u32> {
    let boss = parse_boss(input)?;
    let initial = Player { points: 100, ..Default::default() };
    let mut shop = parse_shop()?;
    shop.armor.push(Item { cost: 0, damage: 0, armor: 0 });
    shop.rings.push(Item { cost: 0, damage: 0, armor: 0 });
    shop.rings.push(Item { cost: 0, damage: 0, armor: 0 });
    let mut max = 0;
    for weapon in shop.weapons.iter() {
        for armor in shop.armor.iter() {
            for rings in shop.rings.iter().combinations(2) {
                let mut player = initial.clone();
                player.damage += weapon.damage;
                player.armor += armor.armor;
                let left = rings[0];
                let right = rings[1];
                player.damage += left.damage + right.damage;
                player.armor += left.armor + right.armor;
                let gold = weapon.cost + armor.cost + left.cost + right.cost;
                if gold < max {
                    continue;
                } else if !play(&player, &boss) {
                    max = gold;
                }
            }
        }
    }
    Ok(max)
}

#[derive(Debug, Default)]
struct Boss {
    points: i32,
    damage: u32,
    armor: u32,
}

#[derive(Clone, Debug, Default)]
struct Player {
    points: i32,
    damage: u32,
    armor: u32,
}

#[derive(Debug)]
struct Item {
    cost: u32,
    damage: u32,
    armor: u32,
}

#[derive(Debug)]
struct Shop {
    weapons: Vec<Item>,
    armor: Vec<Item>,
    rings: Vec<Item>,
}

fn play(player: &Player, boss: &Boss) -> bool {
    let mut player_points = player.points;
    let mut boss_points = boss.points;
    loop {
        boss_points -= player.damage.checked_sub(boss.armor).unwrap_or(1) as i32;
        if boss_points <= 0 {
            break true;
        }
        player_points -= boss.damage.checked_sub(player.armor).unwrap_or(1) as i32;
        if player_points <= 0 {
            break false;
        }
    }
}

fn parse_boss(input: &str) -> Result<Boss> {
    let mut boss = Boss::default();
    for line in input.lines() {
        let line = line.trim();
        let (kind, value) = line.split_once(':').expect("invalid input");
        match kind {
            "Hit Points" => boss.points = value.trim().parse()?,
            "Damage" => boss.damage = value.trim().parse()?,
            "Armor" => boss.armor = value.trim().parse()?,
            _ => panic!("invalid input"),
        }
    }
    Ok(boss)
}

fn parse_shop() -> Result<Shop> {
    let mut shop = Shop {
        weapons: Vec::new(),
        armor: Vec::new(),
        rings: Vec::new(),
    };
    let input = include_str!("../input/shop.txt");
    let mut iter = input.lines().map(|l| l.trim());
    while let Some(line) = iter.next() {
        let (kind, _) = line.split_once(':').expect("invalid shop");
        let items = match kind {
            "Weapons" => &mut shop.weapons,
            "Armor" => &mut shop.armor,
            "Rings" => &mut shop.rings,
            _ => panic!("invalid shop"),
        };
        loop {
            let line = match iter.next() {
                Some(line) if line.is_empty() => break,
                None => break,
                Some(line) => line,
            };
            let cost = line[12..15].trim().parse()?;
            let damage = line[20..21].trim().parse()?;
            let armor = line[28..29].trim().parse()?;
            items.push(Item { cost, damage, armor })
        }
    }
    Ok(shop)
}

#[test]
fn part_one_example() {
    let boss = Boss { points: 12, damage: 7, armor: 2 };
    let player = Player { points: 8, damage: 5, armor: 5 };
    assert!(play(&player, &boss));
}
