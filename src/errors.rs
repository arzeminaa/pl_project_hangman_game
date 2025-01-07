use std::io;
use std::convert::From;

#[derive(Debug)]
pub enum HangmanErrors {
    IoError(io::Error),
    ParseError(String),
    BadGuess(String),
    InvalidSolution(String),
    HangmanGameOver,
    EmptyWordlist,
}

impl From<io::Error> for HangmanErrors {
    fn from(e: io::Error) -> Self {
        HangmanErrors::IoError(e)
    }
}
