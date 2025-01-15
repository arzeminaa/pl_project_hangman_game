use std::path::PathBuf;

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
    Save(String),
    Load(String),
    Quit,
}

impl std::str::FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim().to_lowercase();
        let mut parts = trimmed.split_whitespace();

        match parts.next() {
            Some("try") => match parts.next() {
                Some("letter") => {
                    if let Some(letter) = parts.next() {
                        if letter.len() == 1 {
                            return Ok(Command::TryLetter(letter.chars().next().unwrap()));
                        }
                        return Err("Invalid letter format. Use 'try letter <letter>'.".to_string());
                    }
                    Err("Missing letter. Use 'try letter <letter>'.".to_string())
                }
                Some("word") => {
                    if let Some(word) = parts.next() {
                        return Ok(Command::TryWord(word.to_string()));
                    }
                    Err("Missing word. Use 'try word <word>'.".to_string())
                }
                _ => Err("Invalid 'try' command. Use 'try letter <letter>' or 'try word <word>'.".to_string()),
            },
            Some("help") => Ok(Command::Help),
            Some("info") => Ok(Command::Info),
            Some("save") => {
                if let Some(filename) = parts.next() {
                    return Ok(Command::Save(filename.to_string()));
                }
                Err("Missing filename. Use 'save <filename>'.".to_string())
            }
            Some("load") => {
                if let Some(filename) = parts.next() {
                    return Ok(Command::Load(filename.to_string()));
                }
                Err("Missing filename. Use 'load <filename>'.".to_string())
            }
            Some("quit") => Ok(Command::Quit),
            _ => Err("Invalid command. Type 'help' for a list of valid commands.".to_string()),
        }
    }
}
