use std::{
    error::Error,
    io::{stdin, Read},
    result::Result as StdResult,
};

/// Advent of Code `Result`.
pub type Result<T> = StdResult<T, Box<dyn Error>>;

/// Read input from standard input.
pub fn input_from_stdin() -> Result<String> {
    let mut input = String::new();
    match stdin().read_to_string(&mut input) {
        Ok(_) => Ok(input),
        Err(error) => Err(format!("stdin error: {}", error).into()),
    }
}
