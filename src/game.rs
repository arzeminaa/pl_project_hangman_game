use std::collections::HashSet;
use std::iter::FromIterator;
use rand::Rng;

use crate::errors::HangmanErrors;

pub enum State {
    Pending,
    Victory,
    Unsuccessful,
}

pub struct HangmanGame {
    pub attempted_letters: HashSet<char>,
    pub attempted_words: HashSet<String>,
    pub attempts_remaining: u32,
    pub state: State,
    pub solution: String,
    pub solution_letters: HashSet<char>
}

impl HangmanGame {
    pub fn new(attempts: u32) -> Result<Self, HangmanErrors> {
        let solution = get_random_word();

        if solution.is_empty() {
            return Err(HangmanErrors::InvalidSolution(String::new()))
        }

        if !solution.chars().all(char::is_alphabetic) {
            return Err(HangmanErrors::InvalidSolution(String::from(solution)))
        }

        let solution = String::from(solution);
        let attempted_letters = HashSet::new();
        let attempted_words = HashSet::new();
        let solution_letters = HashSet::from_iter(solution.chars());

        Ok(HangmanGame {
            solution, solution_letters,
            attempted_letters, attempted_words,
            attempts_remaining: attempts,
            state: State::Pending,
        })
    }

    pub fn guess_letter(&mut self, guess: char) -> Result<bool, HangmanErrors> {
        if self.is_over() {
            return Err(HangmanErrors::HangmanGameOver);
        }

        if self.attempted_letters.contains(&guess) {
            return Err(HangmanErrors::BadGuess(String::from("You already tried this letter!")));
        }

        self.attempted_letters.insert(guess);

        if self.attempted_letters.is_superset(&self.solution_letters) {
            self.state = State::Victory;
            Ok(true)
        } else if self.solution.chars().find(|c| *c == guess).is_some() {
            Ok(true)
        } else {
            self.attempts_remaining -= 1;

            if self.attempts_remaining == 0 {
                self.state = State::Unsuccessful;
            }

            Ok(false)
        }
    }

    pub fn guess_word(&mut self, guess: &str) -> Result<bool, HangmanErrors> {
        if self.is_over() {
            return Err(HangmanErrors::HangmanGameOver);
        }

        if self.attempted_words.contains(guess) {
            return Err(HangmanErrors::BadGuess(String::from("You already tried this word!")));
        }

        self.attempted_words.insert(String::from(guess));

        if guess == self.solution {
            self.state = State::Victory;
            Ok(true)
        } else {
            self.attempts_remaining -= 1;

            if self.attempts_remaining == 0 {
                self.state = State::Unsuccessful;
            }

            Ok(false)
        }
    }

    pub fn is_over(&self) -> bool {
        match self.state {
            State::Pending => false,
            _                     => true,
        }
    }
}

fn get_random_word() -> &'static str {
    const WORDS: &[&str] = &["rust", "cargo", "compiler", "borrow", "ownership", "trait", "macro"];
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..WORDS.len());
    WORDS[index]
}
