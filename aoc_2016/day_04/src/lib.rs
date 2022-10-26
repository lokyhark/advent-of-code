use std::collections::BTreeMap;

use aoc::Result;

pub const YEAR: u32 = 2016;
pub const DAY: u32 = 4;

pub fn part_one(input: &str) -> Result<u32> {
    let mut sum: u32 = 0;
    for line in input.trim().lines() {
        let room: Room = line.trim().try_into()?;
        if room.is_real() {
            sum += room.id;
        }
    }
    Ok(sum)
}

pub fn part_two(input: &str) -> Result<u32> {
    for line in input.trim().lines() {
        let room: Room = line.trim().try_into()?;
        if room.real_name() == "northpole object storage" {
            return Ok(room.id);
        }
    }
    Err("northpole object storage not found".into())
}

#[derive(Debug, Eq, PartialEq)]
struct Room<'a> {
    name: &'a str,
    id: u32,
    checksum: &'a str,
}

impl<'a> Room<'a> {
    fn is_real(&self) -> bool {
        let mut map = BTreeMap::new();
        for char in self.name.chars() {
            match char {
                '-' => continue,
                _ => {
                    map.entry(char).and_modify(|x| *x += 1).or_insert(1);
                }
            }
        }
        let mut vec: Vec<_> = map.into_iter().map(|(char, count)| (count, char)).collect();
        vec.sort_by(|a, b| a.0.cmp(&b.0).reverse().then(a.1.cmp(&b.1)));
        let checksum: String = vec.into_iter().map(|(_, char)| char).take(5).collect();
        checksum == self.checksum
    }

    fn real_name(&self) -> String {
        let mut name = Vec::new();
        for byte in self.name.bytes() {
            match byte {
                b'-' => name.push(b' '),
                b => {
                    let byte = (((b - b'a') as u32 + self.id) % 26) as u8 + b'a';
                    name.push(byte);
                }
            }
        }
        String::from_utf8(name).unwrap()
    }
}

impl<'a> TryFrom<&'a str> for Room<'a> {
    type Error = String;

    fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
        let mut start = 0;
        let mut stop = match value.rfind('-') {
            Some(index) => index,
            None => return Err("invalid room identifier {value}".into()),
        };
        let name = &value[start..stop];
        start = stop + 1;
        stop = match value.find('[') {
            Some(index) => index,
            None => return Err("invalid room name {value}".into()),
        };
        let id: u32 = match value[start..stop].parse() {
            Ok(id) => id,
            Err(_) => return Err("invalid room identiufier {value}".into()),
        };
        start = stop + 1;
        stop = match value.rfind(']') {
            Some(index) => index,
            _ => return Err("invalid room checksum {value}".into()),
        };
        let checksum = &value[start..stop];
        Ok(Room { name, id, checksum })
    }
}

#[test]
fn part_one_example() -> Result<()> {
    let rooms = "
    aaaaa-bbb-z-y-x-123[abxyz]
    a-b-c-d-e-f-g-h-987[abcde]
    not-a-real-room-404[oarel]
    totally-real-room-200[decoy]
    ";
    assert_eq!(part_one(rooms)?, 1514);
    Ok(())
}

#[test]
fn part_one_example1() -> Result<()> {
    let room = Room::try_from("aaaaa-bbb-z-y-x-123[abxyz]")?;
    assert_eq!(
        room,
        Room {
            name: "aaaaa-bbb-z-y-x",
            id: 123,
            checksum: "abxyz"
        }
    );
    assert!(room.is_real());
    Ok(())
}

#[test]
fn part_one_example2() -> Result<()> {
    let room = Room::try_from("a-b-c-d-e-f-g-h-987[abcde]")?;
    assert_eq!(
        room,
        Room {
            name: "a-b-c-d-e-f-g-h",
            id: 987,
            checksum: "abcde"
        }
    );
    assert!(room.is_real());
    Ok(())
}

#[test]
fn part_one_example3() -> Result<()> {
    let room = Room::try_from("not-a-real-room-404[oarel]")?;
    assert_eq!(
        room,
        Room {
            name: "not-a-real-room",
            id: 404,
            checksum: "oarel"
        }
    );
    assert!(room.is_real());
    Ok(())
}

#[test]
fn part_one_example4() -> Result<()> {
    let room = Room::try_from("totally-real-room-200[decoy]")?;
    assert_eq!(
        room,
        Room {
            name: "totally-real-room",
            id: 200,
            checksum: "decoy"
        }
    );
    assert!(!room.is_real());
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let room = Room::try_from("qzmt-zixmtkozy-ivhz-343[zimth]")?;
    assert!(room.is_real());
    assert_eq!(room.real_name(), "very encrypted name");
    Ok(())
}
