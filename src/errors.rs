use std::io;
use std::convert::From;
use serde_json;

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

impl From<serde_json::Error> for HangmanErrors {
    fn from(e: serde_json::Error) -> Self {
        HangmanErrors::ParseError(e.to_string())
    }
}
