use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

use crate::wordlist::Wordlist;
use crate::errors::HangmanErrors;

#[derive(clap::Parser, Debug)]
#[command(name="hangman", about="Hangman game")]
pub struct Options {
    #[arg(short='w', long="wordlist", help="The path to the list of words")]
    pub wordlist_path: Option<PathBuf>,

    #[arg(short='a', long="attempts", help="Number of guesses", default_value="10")]
    pub attempts: u32,

    #[arg(short='d', long="debug", help="Show debug info")]
    pub debug: bool,
}

#[derive(Debug)]
pub enum Command {
    TryLetter(char),
    TryWord(String),
    Info,
    Help,
    Quit,
}

impl FromStr for Command {
    type Err = HangmanErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();

        macro_rules! err {
            () => { Err(HangmanErrors::ParseError(s.clone())) }
        }

        match s.chars().nth(0) {
            Some('q') => Ok(Command::Quit),
            Some('i') => Ok(Command::Info),
            Some('h') => Ok(Command::Help),
            Some(c) if c.is_alphabetic() && s.len() == 1 => Ok(Command::TryLetter(c)),
            _ if s.split_whitespace().count() == 1 => Ok(Command::TryWord(s)),
            _ => err!(),
        }
    }
}
