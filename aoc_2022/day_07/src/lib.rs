use std::collections::HashMap;

use aoc::*;

pub const YEAR: u32 = 2022;
pub const DAY: u32 = 7;

pub fn part_one(input: &str) -> Result<usize> {
    let filesystem = parse_input(input)?;
    let size = filesystem.sizes().filter(|size| *size < 100_000).sum();
    Ok(size)
}

pub fn part_two(input: &str) -> Result<usize> {
    let filesystem = parse_input(input)?;
    let full = match filesystem.size("/") {
        Some(size) => size,
        None => return err!("root directory not found"),
    };
    let free = 70_000_000 - full;
    let remain = 30_000_000 - free;
    match filesystem.sizes().filter(|size| *size > remain).min() {
        Some(size) => Ok(size),
        None => err!("directory big enough not found"),
    }
}

fn parse_input(input: &str) -> Result<FileSystem> {
    let mut filesystem = FileSystem::default();
    let mut path = Path::default();
    let mut lines = input.lines();
    let mut last = lines.next().ok_or(error!("invalid filesystem"))?;
    'outer: loop {
        let mut split = last.split_ascii_whitespace();
        match split.nth(1) {
            Some("ls") => loop {
                let line = match lines.next() {
                    Some(line) => line,
                    None => break 'outer,
                };
                if line.starts_with('$') {
                    last = line;
                    break;
                }
                let mut split = line.split_ascii_whitespace();
                match split.next() {
                    Some(slice) if slice.bytes().all(|b| b.is_ascii_digit()) => {
                        let size: usize = slice.parse().unwrap();
                        let file = File::new(size);
                        filesystem.insert_file(&path, file);
                    }
                    Some("dir") => {
                        let name = split.next().ok_or(error!("invalid file name"))?;
                        filesystem.insert_dir(&path, name)
                    }
                    _ => return err!("invalid filesystem"),
                }
            },
            Some("cd") => {
                let name = split.next().ok_or(error!("invalid file name"))?;
                if name == ".." {
                    path.pop();
                } else {
                    path.push(name);
                }
                last = lines.next().ok_or(error!("invalid filesystem"))?;
            }
            _ => return err!("invalid command"),
        }
    }
    Ok(filesystem)
}

#[derive(Debug, Default)]
struct FileSystem {
    files: Vec<File>,
    directories: Vec<Directory>,
    table: HashMap<String, usize>,
}

impl FileSystem {
    fn push_dir(&mut self, name: &str) -> usize {
        let idx = self.directories.len();
        let dir = Directory::default();
        self.directories.push(dir);
        self.table.insert(name.to_string(), idx);
        idx
    }

    fn insert_file(&mut self, path: &Path, file: File) {
        let name = path.full();
        let idx = match self.table.get(&name) {
            Some(&idx) => idx,
            None => self.push_dir(&name),
        };
        let dir = &mut self.directories[idx];
        let idx = self.files.len();
        self.files.push(file);
        dir.push_file(idx);
    }

    fn insert_dir(&mut self, path: &Path, name: &str) {
        let name = path.join(name);
        let child = match self.table.get(&name) {
            Some(&idx) => idx,
            None => self.push_dir(&name),
        };
        let name = path.full();
        let parent = match self.table.get(&name) {
            Some(&idx) => idx,
            None => self.push_dir(&name),
        };
        self.directories[parent].push_dir(child);
    }

    fn size(&self, name: &str) -> Option<usize> {
        self.table.get(name).map(|x| self.size_of(&self.directories[*x]))
    }

    fn sizes(&self) -> impl Iterator<Item = usize> {
        let mut sizes = Vec::new();
        for dir in &self.directories {
            let size = self.size_of(dir);
            sizes.push(size)
        }
        sizes.into_iter()
    }

    fn size_of(&self, dir: &Directory) -> usize {
        let mut size = 0;
        for file in &dir.files {
            size += self.files[*file].size;
        }
        for dir in &dir.directories {
            size += self.size_of(&self.directories[*dir]);
        }
        size
    }
}

#[derive(Debug, Default, Eq, Hash, PartialEq)]
struct Path(Vec<String>);

impl Path {
    fn push(&mut self, name: &str) {
        self.0.push(name.to_string())
    }

    fn pop(&mut self) {
        self.0.pop();
    }

    fn full(&self) -> String {
        self.0.join("/")
    }

    fn join(&self, name: &str) -> String {
        self.0.join("/") + "/" + name
    }
}

#[derive(Debug, Default)]
struct Directory {
    files: Vec<usize>,
    directories: Vec<usize>,
}

impl Directory {
    fn push_file(&mut self, file: usize) {
        self.files.push(file);
    }

    fn push_dir(&mut self, dir: usize) {
        self.directories.push(dir);
    }
}

#[derive(Debug)]
struct File {
    size: usize,
}

impl File {
    pub fn new(size: usize) -> Self {
        Self { size }
    }
}

#[test]
fn part_one_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_one(input)?, 95437);
    Ok(())
}

#[test]
fn part_two_example() -> Result<()> {
    let input = include_str!("../input/example.txt");
    assert_eq!(part_two(input)?, 24933642);
    Ok(())
}
