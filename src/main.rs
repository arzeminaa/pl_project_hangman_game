use std::io::{self, Write};
use clap::Parser;
use hangman::game::HangmanGame;
use hangman::input::{self, Command, Options};
use hangman::tui;
use hangman::game::get_random_word;

fn main() {
    let options = Options::parse();
    let stdin = io::stdin();

    println!("Do you want to load a saved game? (y/n)");
    let mut input = String::new();
    stdin.read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    let mut game = if input == "y" {
        // Load the saved game
        println!("Enter the filename of the saved game:");
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read input");
        let filename = input.trim();
        match HangmanGame::load(filename) {
            Ok(loaded_game) => loaded_game,
            Err(e) => {
                println!("Error loading game: {}. Starting a new game.", e);
                start_new_game(options.attempts)
            }
        }
    } else {
        // Start a new game
        start_new_game(options.attempts)
    };

    tui::clear_screen(options.debug);
    println!("Let's play hangman!");

    loop {
        tui::clear_screen(options.debug);
        tui::draw_hangman(game.attempts_remaining);

        println!("{}", game);

        if game.is_over() {
            break;
        }

        print!("Enter command: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read input");

        let command = match input.parse::<Command>() {
            Ok(c) => c,
            Err(e) => {
                println!("\n{}\n", e);
                continue;
            }
        };

        tui::clear_screen(options.debug);

        match command {
            Command::TryLetter(c)  => tui::print_guess_response(game.guess_letter(c)),
            Command::TryWord(word) => tui::print_guess_response(game.guess_word(&word)),
            Command::Help          => tui::print_help(),
            Command::Info          => tui::print_info(&game),
            Command::Save(filename) => {
                if let Err(e) = game.save(&filename) {
                    println!("Error saving game: {}", e);
                } else {
                    println!("Game saved successfully.");
                }
            },
            Command::Load(filename) => {
                match HangmanGame::load(&filename) {
                    Ok(loaded_game) => game = loaded_game,
                    Err(e) => println!("Error loading game: {}", e),
                }
            },
            Command::Quit          => break,
        };

        println!("");
    }

    println!("Good game!");
}

fn start_new_game(attempts: u32) -> HangmanGame {
    let solution = get_random_word();
    let mut game = HangmanGame::new(attempts).unwrap();
    game.solution = solution.to_string();
    game
}
