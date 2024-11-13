use std::{
    cmp::{max, Ordering},
    collections::BinaryHeap,
};

use aoc::Result;

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 22;

pub fn part_one(input: &str) -> Result<i32> {
    let wizard = Wizard::new(50, 500);
    let boss = parse_boss(input);
    let combat = search(wizard, boss, false);
    Ok(combat.mana)
}

pub fn part_two(input: &str) -> Result<i32> {
    let wizard = Wizard::new(50, 500);
    let boss = parse_boss(input);
    let combat = search(wizard, boss, true);
    Ok(combat.mana)
}

fn parse_boss(input: &str) -> Boss {
    let mut lines = input.lines();
    let health: i32 = lines.next().map(|l| l.split_once(':').unwrap()).map(|x| x.1.trim().parse().unwrap()).unwrap();
    let damage: i32 = lines.next().map(|l| l.split_once(':').unwrap()).map(|x| x.1.trim().parse().unwrap()).unwrap();
    Boss::new(health, damage)
}

fn search(wizard: Wizard, boss: Boss, hard: bool) -> Combat {
    let combat = Combat::new(wizard, boss);
    let mut queue = BinaryHeap::from([combat.clone()]);
    while let Some(combat) = queue.pop() {
        if combat.boss.health <= 0 {
            return combat;
        }
        for combat in rounds(&combat, hard) {
            queue.push(combat);
        }
    }
    panic!("search not found")
}

fn rounds(combat: &Combat, hard: bool) -> Vec<Combat> {
    let mut combats = Vec::new();
    // Wizard turn
    let mut start = combat.clone();
    if hard {
        start.wizard.health -= 1;
    }
    if start.wizard.health < 1 {
        return combats;
    }
    start.effects();
    if start.wizard.health < 1 {
        return combats;
    }
    // Spell choices
    for spell in Spell::iter() {
        if start.effects.iter().any(|x| x.spell == spell) {
            continue;
        }
        if spell.cost() > combat.wizard.mana {
            continue;
        }
        let mut combat = start.clone();
        combat.wizard.mana -= spell.cost();
        combat.mana += spell.cost();
        match spell {
            Spell::Missile => {
                combat.boss.health -= 4;
            }
            Spell::Drain => {
                combat.wizard.health += 2;
                combat.boss.health -= 2;
            }
            Spell::Shield => {
                combat.effects.push(Effect::new(spell, 6));
            }
            Spell::Poison => {
                combat.effects.push(Effect::new(spell, 6));
            }
            Spell::Recharge => {
                combat.effects.push(Effect::new(spell, 5));
            }
        }
        combat.spells.push(spell);
        // Boss turn
        combat.effects();
        if combat.boss.health > 0 {
            combat.wizard.health -= max(1, combat.boss.damage - combat.wizard.armor);
        }
        if combat.wizard.health > 0 {
            combats.push(combat);
        }
    }
    combats
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Wizard {
    health: i32,
    mana: i32,
    armor: i32,
}

impl Wizard {
    fn new(health: i32, mana: i32) -> Self {
        Self { health, mana, armor: 0 }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Boss {
    health: i32,
    damage: i32,
}

impl Boss {
    fn new(health: i32, damage: i32) -> Self {
        Self { health, damage }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Spell {
    Missile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn iter() -> impl Iterator<Item = Spell> {
        [Self::Missile, Self::Drain, Self::Shield, Self::Poison, Self::Recharge].into_iter()
    }
    fn cost(&self) -> i32 {
        match self {
            Spell::Missile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Effect {
    spell: Spell,
    timer: u32,
}

impl Effect {
    fn new(spell: Spell, timer: u32) -> Self {
        Self { spell, timer }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Combat {
    wizard: Wizard,
    boss: Boss,
    spells: Vec<Spell>,
    effects: Vec<Effect>,
    mana: i32,
}

impl Combat {
    fn new(wizard: Wizard, boss: Boss) -> Self {
        Self {
            wizard,
            boss,
            spells: Vec::new(),
            effects: Vec::new(),
            mana: 0,
        }
    }

    fn effects(&mut self) {
        let mut effects = Vec::new();
        self.wizard.armor = 0;
        for effect in &self.effects {
            match effect.spell {
                Spell::Missile | Spell::Drain => unreachable!(),
                Spell::Shield => self.wizard.armor = 7,
                Spell::Poison => self.boss.health -= 3,
                Spell::Recharge => self.wizard.mana += 101,
            }
            if effect.timer > 1 {
                effects.push(Effect::new(effect.spell, effect.timer - 1));
            }
        }
        self.effects = effects;
    }
}

impl PartialOrd for Combat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Combat {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.mana.cmp(&other.mana) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
}

#[test]
fn part_one_example() -> Result<()> {
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    Ok(())
}
