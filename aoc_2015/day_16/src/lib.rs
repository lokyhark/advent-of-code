use aoc::Result;

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 16;

pub fn part_one(input: &str) -> Result<usize> {
    let sues = parse_input(input)?;
    let sue: Vec<_> = sues
        .iter()
        .enumerate()
        .filter(|(_, sue)| {
            (sue.children == None || sue.children == Some(3))
                && (sue.cats == None || sue.cats == Some(7))
                && (sue.samoyeds == None || sue.samoyeds == Some(2))
                && (sue.pomeranians == None || sue.pomeranians == Some(3))
                && (sue.akitas == None || sue.akitas == Some(0))
                && (sue.vizslas == None || sue.vizslas == Some(0))
                && (sue.goldfish == None || sue.goldfish == Some(5))
                && (sue.trees == None || sue.trees == Some(3))
                && (sue.cars == None || sue.cars == Some(2))
                && (sue.perfumes == None || sue.perfumes == Some(1))
        })
        .map(|(i, _)| i + 1)
        .collect();
    if sue.is_empty() {
        return Err("Sue not found".into());
    }
    if sue.len() != 1 {
        return Err("Multiple Sue found".into());
    }
    Ok(sue[0])
}

pub fn part_two(input: &str) -> Result<usize> {
    let sues = parse_input(input)?;
    let sue: Vec<_> = sues
        .iter()
        .enumerate()
        .filter(|(_, sue)| {
            (sue.children == None || sue.children == Some(3))
                && (sue.cats == None || sue.cats > Some(7))
                && (sue.samoyeds == None || sue.samoyeds == Some(2))
                && (sue.pomeranians == None || sue.pomeranians < Some(3))
                && (sue.akitas == None || sue.akitas == Some(0))
                && (sue.vizslas == None || sue.vizslas == Some(0))
                && (sue.goldfish == None || sue.goldfish < Some(5))
                && (sue.trees == None || sue.trees > Some(3))
                && (sue.cars == None || sue.cars == Some(2))
                && (sue.perfumes == None || sue.perfumes == Some(1))
        })
        .map(|(i, _)| i + 1)
        .collect();
    if sue.is_empty() {
        return Err("Sue not found".into());
    }
    if sue.len() != 1 {
        return Err("Multiple Sue found".into());
    }
    Ok(sue[0])
}

fn parse_input(input: &str) -> Result<Vec<Sue>> {
    let mut sues = Vec::new();
    for line in input.trim().lines() {
        let mut sue = Sue::default();
        let compounds = match line.split_once(':') {
            Some(split) => split.1.trim(),
            None => return Err("invalid input".into()),
        };
        for compound in compounds.split(',').map(|x| x.trim()) {
            let (name, count) = match compound.split_once(':') {
                Some(split) => (split.0.trim(), split.1.trim()),
                None => return Err("invalid input".into()),
            };
            let count = match count.parse() {
                Ok(value) => value,
                Err(_) => return Err("invalid input".into()),
            };
            match name {
                "children" => sue.children = Some(count),
                "cats" => sue.cats = Some(count),
                "samoyeds" => sue.samoyeds = Some(count),
                "pomeranians" => sue.pomeranians = Some(count),
                "akitas" => sue.akitas = Some(count),
                "vizslas" => sue.vizslas = Some(count),
                "goldfish" => sue.goldfish = Some(count),
                "trees" => sue.trees = Some(count),
                "cars" => sue.cars = Some(count),
                "perfumes" => sue.perfumes = Some(count),
                _ => return Err(format!("invalid compound: '{}'", compound).into()),
            }
        }
        sues.push(sue);
    }
    Ok(sues)
}

#[derive(Debug, Default)]
struct Sue {
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}
