use std::collections::HashMap;

use aoc::*;

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 23;

pub fn part_one(input: &str) -> Result<i32> {
    let program = parse_program(input)?;
    let mut computer = build_computer(0);
    computer.execute(&program)?;
    computer.get("b").ok_or(error!("register b not defined"))
}

pub fn part_two(input: &str) -> Result<i32> {
    let program = parse_program(input)?;
    let mut computer = build_computer(1);
    computer.execute(&program)?;
    computer.get("b").ok_or(error!("register b not defined"))
}

fn parse_program(input: &str) -> Result<Program> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        let mut split = line.split([' ', ',']).filter(|x| !x.is_empty());
        let instruction = match split.next() {
            Some(instruction) => instruction,
            None => return err!("invalid program"),
        };
        let instruction = match instruction {
            "hlf" => match split.next() {
                Some(register) => Instruction::Half { register },
                None => return err!("invalid program: missing register"),
            },
            "tpl" => match split.next() {
                Some(register) => Instruction::Triple { register },
                None => return err!("invalid program: missing register"),
            },
            "inc" => match split.next() {
                Some(register) => Instruction::Increment { register },
                None => return err!("invalid program: missing register"),
            },
            "jmp" => match split.next() {
                Some(offset) => match offset.parse() {
                    Ok(offset) => Instruction::Jump { offset },
                    Err(_) => return err!("invalid program: invalid offset {}", offset),
                },
                None => return err!("invalid program: missing offset"),
            },
            "jie" => {
                let register = match split.next() {
                    Some(register) => register,
                    None => return err!("invalid program: missing register"),
                };
                let offset = match split.next() {
                    Some(offset) => match offset.parse() {
                        Ok(offset) => offset,
                        Err(_) => return err!("invalid program: invalid offset {}", offset),
                    },
                    None => return err!("invalid program: missing offset"),
                };
                Instruction::JumpIfEven { register, offset }
            }
            "jio" => {
                let register = match split.next() {
                    Some(register) => register,
                    None => return err!("invalid program: missing register"),
                };
                let offset = match split.next() {
                    Some(offset) => match offset.parse() {
                        Ok(offset) => offset,
                        Err(_) => return err!("invalid program: invalid offset {}", offset),
                    },
                    None => return err!("invalid program: missing offset"),
                };
                Instruction::JumpIfOne { register, offset }
            }
            _ => return err!("invalid program: invalid instruction {}", instruction),
        };
        instructions.push(instruction);
    }
    Ok(Program { instructions })
}

fn build_computer(a: i32) -> Computer {
    let mut computer = Computer::default();
    computer.set("a", a);
    computer.set("b", 0);
    computer
}

#[derive(Debug, Default)]
pub struct Computer {
    registers: HashMap<String, i32>,
    position: isize,
}

impl Computer {
    fn execute(&mut self, program: &Program) -> Result<()> {
        loop {
            if self.position == program.instructions.len() as isize {
                break Ok(());
            }
            let position: usize = match self.position.try_into() {
                Ok(position) => position,
                Err(_) => return err!("invalid program: negative position"),
            };
            let instruction = &program.instructions[position];
            match instruction {
                Instruction::Half { register } => {
                    match self.registers.get_mut(*register) {
                        Some(x) => {
                            *x /= 2;
                        }
                        None => return err!("unknown register: {}", register),
                    }
                    self.position += 1;
                }
                Instruction::Triple { register } => {
                    match self.registers.get_mut(*register) {
                        Some(x) => {
                            *x *= 3;
                        }
                        None => return err!("unknown register: {}", register),
                    }
                    self.position += 1;
                }
                Instruction::Increment { register } => {
                    match self.registers.get_mut(*register) {
                        Some(x) => {
                            *x += 1;
                        }
                        None => return err!("unknown register: {}", register),
                    }
                    self.position += 1;
                }
                Instruction::Jump { offset } => {
                    self.position += offset;
                }
                Instruction::JumpIfEven { register, offset } => {
                    let register = match self.registers.get(*register) {
                        Some(x) => x,
                        None => return err!("unknown register: {}", register),
                    };
                    if register % 2 == 0 {
                        self.position += offset;
                    } else {
                        self.position += 1;
                    }
                }
                Instruction::JumpIfOne { register, offset } => {
                    let register = match self.registers.get(*register) {
                        Some(x) => x,
                        None => return err!("unknown register: {}", register),
                    };
                    if *register == 1 {
                        self.position += offset;
                    } else {
                        self.position += 1;
                    }
                }
            }
        }
    }

    fn get(self, name: &str) -> Option<i32> {
        self.registers.get(name).copied()
    }

    fn set(&mut self, name: &str, value: i32) -> Option<i32> {
        self.registers.insert(name.to_string(), value)
    }
}

#[derive(Debug)]
pub struct Program<'input> {
    instructions: Vec<Instruction<'input>>,
}

#[derive(Debug)]
enum Instruction<'input> {
    Half { register: &'input str },
    Triple { register: &'input str },
    Increment { register: &'input str },
    Jump { offset: isize },
    JumpIfEven { register: &'input str, offset: isize },
    JumpIfOne { register: &'input str, offset: isize },
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    let program = parse_program(input)?;
    let mut computer = build_computer(0);
    computer.execute(&program)?;
    assert_eq!(computer.get("a"), Some(2));
    Ok(())
}
