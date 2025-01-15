use std::fmt::{self, Display};

use crate::errors::HangmanErrors;
use crate::game::{HangmanGame, State};

impl Display for HangmanErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &HangmanErrors::IoError(ref e) => {
                f.write_fmt(format_args!("{}", e))
            }
            &HangmanErrors::ParseError(ref input) => {
                f.write_fmt(format_args!("Invalid command: {:?}. ", &input))?;
                f.write_str("Try the 'help' command for a list of valid commands")
            }
            &HangmanErrors::BadGuess(ref message) => {
                f.write_str(&message)
            },
            &HangmanErrors::InvalidSolution(ref word) => {
                f.write_fmt(format_args!("Invalid word given for solution: {:?}. ", &word))?;
                f.write_str("Needs to consist of alphabetical letters only")
            },
            &HangmanErrors::HangmanGameOver => {
                f.write_str("Game is already over!")
            },
            &HangmanErrors::EmptyWordlist => {
                f.write_str("Couldn't load any valid wordlist!\n")?;
                f.write_str("Searched locations:\n")?;
                f.write_str("  - ~/.hangman_words.txt\n")?;
                f.write_str("  - A file provided via `-w <filename>`\n")?;
                f.write_str("\n")?;
                f.write_str("Note that a wordlist needs to consist of at least one valid word\n")?;
                f.write_str("(made up of alphabetical characters only)\n")?;
                Ok(())
            },
        }
    }
}

impl Display for HangmanGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let chars = self.solution.chars();

        match self.state {
            State::Victory => {
                return f.write_fmt(format_args!("Winner!\nThe word is: {}", self.solution))
            },
            State::Unsuccessful => {
                return f.write_fmt(format_args!("Sorry, no attempts left. You lost!\nThe word is: {}", self.solution))
            },
            State::Pending => f.write_fmt(format_args!("Attempts left: {}\n", self.attempts_remaining))?,
        }

        for letter in chars {
            if self.attempted_letters.contains(&letter) {
                f.write_fmt(format_args!("{} ", letter))?;
            } else {
                f.write_str("_ ")?;
            }
        }

        Ok(())
    }
}

pub fn print_help() {
    println!("");
    println!("Valid commands:");
    println!("  - help            => Show these instructions");
    println!("  - info            => Information about attempted words/letters");
    println!("  - quit            => Quit the game");
    println!("  - <letter>        => Try this letter");
    println!("  - <word>          => Try an entire word");
    println!("  - save <filename> => Save the current game to a file");
    println!("  - load <filename> => Load a saved game from a file");
    println!("");
}

pub fn print_info(game: &HangmanGame) {
    println!("Information about the state of the game:");
    println!("  - Attempted letters: {:?}", game.attempted_letters);
    println!("  - Attempted words:   {:?}", game.attempted_words);
    println!("  - Score: {}", game.score);
}

pub fn print_guess_response(response: Result<bool, HangmanErrors>) {
    match response {
        Ok(true) => println!("Good guess!"),
        Ok(false) => println!("Sorry, no dice"),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn draw_hangman(attempts_remaining: u32) {
    let stages = [
        "   \n   \n   \n   \n   \n",
        "   \n   \n   \n   \n   \n  +\n",
        "   \n   \n   \n   \n  |\n  +\n",
        "   \n   \n   \n  |\n  |\n  +\n",
        "   \n   \n  O\n  |\n  |\n  +\n",
        "   \n   \n  O\n /|\n  |\n  +\n",
        "   \n   \n  O\n /|\\\n  |\n  +\n",
        "   \n   \n  O\n /|\\\n / \n  +\n",
        "   \n   \n  O\n /|\\\n / \\\n  +\n",
    ];
    let index = stages.len().saturating_sub(attempts_remaining as usize + 1);
    println!("{}", stages[index]);
}

pub fn clear_screen(debug: bool) {
    if debug { return; }

    print!("{}[2J", 27 as char);
    print!("{}[1;1H", 27 as char);
}
