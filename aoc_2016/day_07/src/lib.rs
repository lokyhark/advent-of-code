use std::collections::HashSet;

use aoc::*;

pub const YEAR: u32 = 2016;
pub const DAY: u32 = 7;

pub fn part_one(input: &str) -> Result<usize> {
    Ok(input.lines().filter(|l| tls(l)).count())
}

pub fn part_two(input: &str) -> Result<usize> {
    Ok(input.lines().filter(|l| ssl(l)).count())
}

fn tls(ip: &str) -> bool {
    let packets: Vec<_> = ip.split(['[', ']']).collect();
    for hypernet in packets.iter().skip(1).step_by(2) {
        if abba(hypernet) {
            return false;
        }
    }
    for supernet in packets.iter().step_by(2) {
        if abba(supernet) {
            return true;
        }
    }
    false
}

fn ssl(ip: &str) -> bool {
    let packets: Vec<_> = ip.split(['[', ']']).collect();
    let mut accessors = Vec::new();
    for supernet in packets.iter().step_by(2) {
        accessors.extend(abas(supernet));
    }
    let mut blocks = Vec::new();
    for hypernet in packets.iter().skip(1).step_by(2) {
        blocks.extend(abas(hypernet));
    }
    let set: HashSet<_> = accessors.iter().map(|x| [x[1], x[0], x[1]]).collect();
    blocks.iter().any(|b| set.contains(b))
}

fn abba(packet: &str) -> bool {
    let bytes: Vec<_> = packet.bytes().collect();
    for window in bytes.windows(4) {
        if window[0] == window[3] && window[1] == window[2] && window[0] != window[1] {
            return true;
        }
    }
    false
}

fn abas(packet: &str) -> Vec<[u8; 3]> {
    let mut abas = Vec::new();
    let bytes: Vec<_> = packet.bytes().collect();
    for window in bytes.windows(3) {
        if window[0] == window[2] && window[0] != window[1] {
            abas.push(window.try_into().unwrap());
        }
    }
    abas
}

#[test]
fn part_one_example() -> Result<()> {
    assert!(tls("abba[mnop]qrst"));
    assert!(!tls("abcd[bddb]xyyx"));
    assert!(!tls("aaaa[qwer]tyui"));
    assert!(tls("ioxxoj[asdfgh]zxcvbn"));
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    assert!(ssl("aba[bab]xyz"));
    assert!(!ssl("xyx[xyx]xyx"));
    assert!(ssl("aaa[kek]eke"));
    assert!(ssl("zazbz[bzb]cdb"));
    Ok(())
}
