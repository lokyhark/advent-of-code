use std::{
    fs::File,
    io::{stdin, Read},
    path::Path,
};

use anyhow::{anyhow, Result};
use isahc::{HttpClient, ReadResponseExt};

fn main() -> Result<()> {
    let mut cookie = String::new();
    match stdin().read_to_string(&mut cookie) {
        Ok(_) => (),
        Err(error) => return Err(anyhow!("stdin error: {}", error)),
    }
    let client = client(&cookie)?;
    let root = std::env::current_dir().unwrap();
    for year in 2022..=2022 {
        for day in 6..=6 {
            setup_layout(&root, &client, year, day)?;
        }
    }
    Ok(())
}

fn client(cookie: &str) -> Result<HttpClient> {
    let cookie = format!("session={}", cookie);
    let client = HttpClient::builder().default_header("Cookie", cookie).build()?;
    Ok(client)
}

fn setup_layout(root: &Path, client: &HttpClient, year: u32, day: u32) -> Result<()> {
    let path = root.join(format!("aoc_{}/day_{:02}", year, day));
    if !path.exists() {
        let url = format!("https://adventofcode.com/{}/day/{}", year, day);
        let response = client.get(&url)?;
        if response.status().is_success() {
            let crate_name = format!("aoc_{}_day_{:02}", year, day);
            dir(&path)?;
            cargo(&path, &crate_name, year, day)?;
            src_dir(&path)?;
            bin(&path, &crate_name)?;
            lib(&path, year, day)?;
            tests_dir(&path)?;
            test(&path, &crate_name)?;
            readme(&path, client, year, day)?;
            input_dir(&path)?;
            input(&path, client, year, day)?;
        }
    }
    Ok(())
}

fn dir(path: &Path) -> Result<()> {
    std::fs::create_dir_all(path)?;
    Ok(())
}

fn cargo(path: &Path, crate_name: &str, year: u32, day: u32) -> Result<()> {
    let path = path.join("Cargo.toml");
    let contents = format!(
        "[package]\n\
        name = \"{}\"\n\
        version = \"1.0.0\"\n\
        edition = \"2021\"\n\
        description = \"Advent of Code {}-{}\"\n\
        license = \"MIT\"\n\
        \n\
        [dependencies]\n\
        aoc = {{ path = \"../..\" }}\n",
        crate_name, year, day
    );
    std::fs::write(path, contents)?;
    Ok(())
}

fn src_dir(path: &Path) -> Result<()> {
    let path = path.join("src");
    std::fs::create_dir_all(path)?;
    Ok(())
}

fn bin(path: &Path, crate_name: &str) -> Result<()> {
    let path = path.join("src/main.rs");
    let contents = format!(
        "\
use std::io::{{stdout, Write}};

use aoc::Result;

use {}::*;

fn main() -> Result<()> {{
    println!(\"Advent of Code {{}}-{{:02}}\", YEAR, DAY);
    let input = aoc::input_from_stdin()?;
    let answer = part_one(&input)?;
    writeln!(stdout(), \"--> part one:\")?;
    writeln!(stdout(), \"{{}}\", answer)?;
    let answer = part_two(&input)?;
    writeln!(stdout(), \"--> part two:\")?;
    writeln!(stdout(), \"{{}}\", answer)?;
    Ok(())
}}
",
        crate_name,
    );
    std::fs::write(path, contents)?;
    Ok(())
}

fn lib(path: &Path, year: u32, day: u32) -> Result<()> {
    let path = path.join("src/lib.rs");
    let contents = format!(
        "\
use aoc::Result;

pub const YEAR: u32 = {};
pub const DAY: u32 = {};

pub fn part_one(input: &str) -> Result<usize> {{
    Ok(input.len())
}}

pub fn part_two(input: &str) -> Result<usize> {{
    Ok(input.len())
}}

#[test]
fn part_one_example() -> Result<()> {{
    Ok(())
}}

#[test]
fn part_two_example() -> Result<()> {{
    Ok(())
}}
",
        year, day
    );
    std::fs::write(path, contents)?;
    Ok(())
}

fn tests_dir(path: &Path) -> Result<()> {
    let path = path.join("tests");
    std::fs::create_dir_all(path)?;
    Ok(())
}

fn test(path: &Path, crate_name: &str) -> Result<()> {
    let mut path = path.join("tests");
    path.push(crate_name);
    path.set_extension("rs");
    let contents = format!(
        "\
use aoc::Result;

use {}::*;

#[test]
fn part_one_answer() -> Result<()> {{
    let input = include_str!(\"../input/input.txt\");
    assert_eq!(part_one(input)?, input.len());
    Ok(())
}}

#[test]
fn part_two_answer() -> Result<()> {{
    let input = include_str!(\"../input/input.txt\");
    assert_eq!(part_two(input)?, input.len());
    Ok(())
}}
",
        crate_name,
    );
    std::fs::write(path, contents)?;
    Ok(())
}

fn readme(path: &Path, client: &HttpClient, year: u32, day: u32) -> Result<()> {
    let path = path.join("README.md");
    let url = format!("https://adventofcode.com/{}/day/{}", year, day);
    let mut response = client.get(&url)?;
    if response.status().is_success() {
        let text = response.text()?;
        let mut split = text.split("---");
        let title = split.nth(1).unwrap().trim();
        let contents = format!(
            "\
# Advent of Code -- Year {} - Day {:02}

[{}]({})
",
            year, day, title, url
        );
        std::fs::write(path, contents)?;
    }
    Ok(())
}

fn input_dir(path: &Path) -> Result<()> {
    let path = path.join("input");
    std::fs::create_dir_all(path)?;
    Ok(())
}

fn input(path: &Path, client: &HttpClient, year: u32, day: u32) -> Result<()> {
    let path = path.join("input/input.txt");
    let mut file = File::create(path)?;
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let mut response = client.get(url)?;
    response.copy_to(&mut file)?;
    Ok(())
}
