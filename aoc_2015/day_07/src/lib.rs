use std::collections::HashMap;

use aoc::Result;

pub const YEAR: u32 = 2015;
pub const DAY: u32 = 7;

pub fn part_one(input: &str) -> Result<u16> {
    let mut signals = HashMap::new();
    solve(input.trim(), &mut signals, ["a"])?;
    match signals.get("a") {
        Some(signal) => Ok(*signal),
        None => return Err("signal 'a' not found".into()),
    }
}

pub fn part_two(input: &str) -> Result<u16> {
    let mut signals = HashMap::new();
    solve(input.trim(), &mut signals, ["a"])?;
    let signal = match signals.get("a") {
        Some(signal) => *signal,
        None => return Err("signal 'a' not found".into()),
    };
    signals.clear();
    signals.insert("b", signal);
    solve(input.trim(), &mut signals, ["a"])?;
    match signals.get("a") {
        Some(signal) => Ok(*signal),
        None => return Err("signal 'a' not found".into()),
    }
}

fn solve<'a>(input: &'a str, signals: &mut HashMap<&'a str, u16>, wires: impl IntoIterator<Item = &'a str>) -> Result<()> {
    let circuit = circuit(input.trim())?;
    let mut stack: Vec<_> = wires.into_iter().collect();
    while !stack.is_empty() {
        let wire = *stack.last().unwrap();
        let gate = match circuit.get(wire) {
            Some(gate) => gate,
            None => return Err(format!("gate not found '{}'", wire).into()),
        };
        match gate {
            Gate::Set(connection) => {
                let signal = match connection {
                    Connection::Wire(x) => match signals.get(x) {
                        Some(x) => *x,
                        None => {
                            stack.push(x);
                            continue;
                        }
                    },
                    Connection::Provider(x) => *x,
                };
                signals.insert(wire, signal);
            }
            Gate::And(left, right) => {
                let left = match left {
                    Connection::Wire(x) => match signals.get(x) {
                        Some(x) => *x,
                        None => {
                            stack.push(x);
                            continue;
                        }
                    },
                    Connection::Provider(x) => *x,
                };
                let right = match right {
                    Connection::Wire(x) => match signals.get(x) {
                        Some(x) => *x,
                        None => {
                            stack.push(x);
                            continue;
                        }
                    },
                    Connection::Provider(x) => *x,
                };
                signals.insert(wire, left & right);
            }
            Gate::Or(left, right) => {
                let left = match left {
                    Connection::Wire(x) => match signals.get(x) {
                        Some(x) => *x,
                        None => {
                            stack.push(x);
                            continue;
                        }
                    },
                    Connection::Provider(x) => *x,
                };
                let right = match right {
                    Connection::Wire(x) => match signals.get(x) {
                        Some(x) => *x,
                        None => {
                            stack.push(x);
                            continue;
                        }
                    },
                    Connection::Provider(x) => *x,
                };
                signals.insert(wire, left | right);
            }
            Gate::Lsh(left, right) => {
                let left = match left {
                    Connection::Wire(x) => match signals.get(x) {
                        Some(x) => *x,
                        None => {
                            stack.push(x);
                            continue;
                        }
                    },
                    Connection::Provider(x) => *x,
                };
                signals.insert(wire, left << right);
            }
            Gate::Rsh(left, right) => {
                let left = match left {
                    Connection::Wire(x) => match signals.get(x) {
                        Some(x) => *x,
                        None => {
                            stack.push(x);
                            continue;
                        }
                    },
                    Connection::Provider(x) => *x,
                };
                signals.insert(wire, left >> right);
            }
            Gate::Not(connection) => {
                let signal = match connection {
                    Connection::Wire(x) => match signals.get(x) {
                        Some(x) => *x,
                        None => {
                            stack.push(x);
                            continue;
                        }
                    },
                    Connection::Provider(x) => *x,
                };
                signals.insert(wire, !signal);
            }
        };
        stack.pop();
    }
    Ok(())
}

fn circuit(circuit: &str) -> Result<HashMap<&str, Gate>> {
    let mut map = HashMap::new();
    for line in circuit.lines() {
        let (prefix, suffix) = match line.split_once("->") {
            Some(split) => split,
            None => return Err(format!("invalid circuit: '{}'", line).into()),
        };
        let gate = gate(prefix.trim())?;
        map.insert(suffix.trim(), gate);
    }
    Ok(map)
}

fn gate(gate: &str) -> Result<Gate> {
    if gate.contains("NOT") {
        let connection = match gate.split_ascii_whitespace().nth(1) {
            Some(wire) => connection(wire),
            None => return Err(format!("invalid gate: '{}'", gate).into()),
        };
        Ok(Gate::Not(connection))
    } else if gate.contains("AND") || gate.contains("OR") {
        let mut iter = gate.split_ascii_whitespace();
        let left = match iter.next() {
            Some(left) => connection(left),
            None => return Err(format!("invalid gate: '{}'", gate).into()),
        };
        let operation = match iter.next() {
            Some(operation) => operation,
            None => return Err(format!("invalid gate: '{}'", gate).into()),
        };
        let right = match iter.next() {
            Some(right) => connection(right),
            None => return Err(format!("invalid gate: '{}'", gate).into()),
        };
        match operation {
            "OR" => Ok(Gate::Or(left, right)),
            "AND" => Ok(Gate::And(left, right)),
            _ => Err(format!("invalid gate: '{}'", gate).into()),
        }
    } else if gate.contains("SHIFT") {
        let mut iter = gate.split_ascii_whitespace();
        let left = match iter.next() {
            Some(left) => connection(left),
            None => return Err(format!("invalid gate: '{}'", gate).into()),
        };
        let operation = match iter.next() {
            Some(operation) => operation,
            None => return Err(format!("invalid gate: '{}'", gate).into()),
        };
        let right = match iter.next().map(|x| x.parse()) {
            Some(Ok(right)) => right,
            _ => return Err(format!("invalid gate: '{}'", gate).into()),
        };
        match operation {
            "LSHIFT" => Ok(Gate::Lsh(left, right)),
            "RSHIFT" => Ok(Gate::Rsh(left, right)),
            _ => Err(format!("invalid gate: '{}'", gate).into()),
        }
    } else if gate.chars().all(|x| !x.is_ascii_whitespace()) {
        let connection = connection(gate);
        Ok(Gate::Set(connection))
    } else {
        Err(format!("invalid gate: '{}'", gate).into())
    }
}

fn connection(connection: &str) -> Connection {
    if connection.chars().all(|c| c.is_digit(10)) {
        Connection::Provider(connection.parse().unwrap())
    } else {
        Connection::Wire(connection)
    }
}

#[derive(Debug)]
enum Gate<'a> {
    Set(Connection<'a>),
    And(Connection<'a>, Connection<'a>),
    Or(Connection<'a>, Connection<'a>),
    Lsh(Connection<'a>, u16),
    Rsh(Connection<'a>, u16),
    Not(Connection<'a>),
}

#[derive(Debug)]
enum Connection<'a> {
    Wire(&'a str),
    Provider(u16),
}

#[test]
fn example() -> Result<()> {
    let input = "\
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
";
    let mut signals = HashMap::new();
    solve(input, &mut signals, ["d", "e", "f", "g", "h", "i", "x", "y"])?;
    assert_eq!(signals.get("d"), Some(&72));
    assert_eq!(signals.get("e"), Some(&507));
    assert_eq!(signals.get("f"), Some(&492));
    assert_eq!(signals.get("g"), Some(&114));
    assert_eq!(signals.get("h"), Some(&65412));
    assert_eq!(signals.get("i"), Some(&65079));
    assert_eq!(signals.get("x"), Some(&123));
    assert_eq!(signals.get("y"), Some(&456));
    Ok(())
}
