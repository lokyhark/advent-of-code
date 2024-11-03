use std::{
    error::Error as StdError,
    fmt::Display,
    io::{stdin, Read},
    ops::Deref,
    result::Result as StdResult,
};

/// Advent of Code `Error`.
#[derive(Debug)]
pub struct Error(pub Box<dyn StdError + Send + Sync + 'static>);

impl Deref for Error {
    type Target = dyn StdError + Send + Sync;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl<E: StdError + Send + Sync + 'static> From<E> for Error {
    fn from(value: E) -> Self {
        Self(Box::new(value))
    }
}

impl From<Error> for Box<dyn StdError + Send + Sync + 'static> {
    fn from(value: Error) -> Self {
        value.0
    }
}
/// Advent of Code `Result`.
pub type Result<T> = StdResult<T, Error>;

#[macro_export]
macro_rules! error {
    ($($tt:tt)*) => { Error(format!($($tt)*).into())}
}

/// Construct an Err variant.
#[macro_export]
macro_rules! err {
    ($($tt:tt)*) => { Err(Error(format!($($tt)*).into())) }
}

/// Read input from standard input.
pub fn input_from_stdin() -> Result<String> {
    let mut input = String::new();
    match stdin().read_to_string(&mut input) {
        Ok(_) => Ok(input),
        Err(error) => err!("stdin error: {}", error),
    }
}
