use aoc::*;

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 25;

const INI: u64 = 20151125;
const MUL: u64 = 252533;
const MOD: u64 = 33554393;

pub fn part_one(input: &str) -> Result<u64> {
    let (row, col) = parse(input)?;
    let index = index(row, col);
    Ok(code(index))
}

fn parse(input: &str) -> Result<(usize, usize)> {
    let mut split = input.split([' ', '.', ',']).filter(|x| !x.is_empty() && x.chars().all(|c| c.is_ascii_digit()));
    let row: usize = match split.next() {
        Some(slice) => slice.parse().unwrap(),
        None => return err!("invalid input: no row"),
    };
    let col: usize = match split.next() {
        Some(slice) => slice.parse().unwrap(),
        None => return err!("invalid input: no col"),
    };
    Ok((row, col))
}

fn index(row: usize, col: usize) -> usize {
    let diag = col + row - 1;
    let last = diag * (diag + 1) / 2;
    last - row
}

fn code(index: usize) -> u64 {
    let exp = index as u64;
    (INI * dbg!(modexp(MUL, exp, MOD))) % MOD
}

fn modexp(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp >>= 1;
    }
    result
}

#[test]
fn part_one_example1() -> Result<()> {
    assert_eq!(index(4, 2), 11);
    Ok(())
}

#[test]
fn part_one_example2() -> Result<()> {
    assert_eq!(index(1, 5), 14);
    Ok(())
}

#[test]
fn part_one_example3() -> Result<()> {
    let grid = include_str!("../input/example.txt");
    let numbers: Vec<_> = grid.split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
    for i in 0..6 {
        for j in 0..6 {
            assert_eq!(code(index(i + 1, j + 1)), numbers[i * 6 + j])
        }
    }
    Ok(())
}
