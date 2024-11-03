use aoc::*;

pub const YEAR: u32 = 2021;
pub const DAY: u32 = 3;

pub fn part_one(input: &str) -> Result<usize> {
    let report = parse_report(input)?;
    let gamma = report.gamma_rate();
    let epsilon = report.epsilon_rate();
    Ok(gamma * epsilon)
}

pub fn part_two(input: &str) -> Result<usize> {
    let report = parse_report(input)?;
    let oxygen = report.oxygen_rate();
    let co2 = report.co2_rate();
    Ok(oxygen * co2)
}

fn parse_report(input: &str) -> Result<Report> {
    let mut numbers = Vec::new();
    for line in input.lines() {
        let mut number = Number::default();
        for char in line.chars() {
            match char {
                '0' => number.push(Bit::Zero),
                '1' => number.push(Bit::One),
                _ => return err!("invalid bit '{}'", char.escape_debug()),
            }
        }
        numbers.push(number);
    }
    if numbers.is_empty() {
        return err!("invalid input: empty");
    }
    let len = numbers[0].len();
    if !numbers.iter().all(|n| n.len() == len) {
        return err!("invalid input: different number of digits per number");
    }
    let report = Report::new(numbers);
    Ok(report)
}

fn parse_number(number: &Number) -> usize {
    usize::from_str_radix(&number.to_str(), 2).expect("invalid binary number")
}

fn count_bits(numbers: &[Number], col: usize) -> (usize, usize) {
    let mut zeros = 0;
    let mut ones = 0;
    for number in numbers {
        match number.get(col) {
            Bit::Zero => zeros += 1,
            Bit::One => ones += 1,
        }
    }
    (zeros, ones)
}

fn most_common_bit(numbers: &[Number], col: usize) -> Bit {
    let (zeros, ones) = count_bits(numbers, col);
    if zeros > ones {
        Bit::Zero
    } else {
        Bit::One
    }
}

fn least_common_bit(numbers: &[Number], col: usize) -> Bit {
    let (zeros, ones) = count_bits(numbers, col);
    if zeros > ones {
        Bit::One
    } else {
        Bit::Zero
    }
}

#[derive(Debug)]
struct Report {
    numbers: Vec<Number>,
}

impl Report {
    fn new(numbers: Vec<Number>) -> Self {
        Self { numbers }
    }

    fn gamma_rate(&self) -> usize {
        let mut number = Number::default();
        for col in 0..self.numbers[0].len() {
            let bit = most_common_bit(&self.numbers, col);
            number.push(bit);
        }
        parse_number(&number)
    }

    fn epsilon_rate(&self) -> usize {
        let mut number = Number::default();
        for col in 0..self.numbers[0].len() {
            let bit = least_common_bit(&self.numbers, col);
            number.push(bit);
        }
        parse_number(&number)
    }

    fn oxygen_rate(&self) -> usize {
        let mut i = 0;
        let mut numbers: Vec<Number> = self.numbers.to_vec();
        loop {
            let bit = most_common_bit(numbers.as_slice(), i);
            numbers.retain(|n| n.get(i) == bit);
            if numbers.len() == 1 {
                return parse_number(&numbers[0]);
            }
            i += 1;
        }
    }

    fn co2_rate(&self) -> usize {
        let mut i = 0;
        let mut numbers: Vec<Number> = self.numbers.to_vec();
        loop {
            let bit = least_common_bit(numbers.as_slice(), i);
            numbers.retain(|n| n.get(i) == bit);
            if numbers.len() == 1 {
                return parse_number(&numbers[0]);
            }
            i += 1;
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Number {
    bits: Vec<Bit>,
}

impl Number {
    fn len(&self) -> usize {
        self.bits.len()
    }

    fn push(&mut self, bit: Bit) {
        self.bits.push(bit)
    }

    fn get(&self, col: usize) -> Bit {
        self.bits[col]
    }

    fn to_str(&self) -> String {
        let mut string = String::new();
        for bit in &self.bits {
            match bit {
                Bit::Zero => string.push('0'),
                Bit::One => string.push('1'),
            }
        }
        string
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Bit {
    Zero,
    One,
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_one(input)?, 198);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_two(input)?, 230);
    Ok(())
}
