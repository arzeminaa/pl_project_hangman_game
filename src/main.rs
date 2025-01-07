use std::io::{self, Write};
use clap::Parser;
use hangman::game::HangmanGame;
use hangman::input::{self, Command, Options};
use hangman::tui;
use hangman::game::get_random_word;

fn main() {
    let options = Options::parse();

    let solution = get_random_word();
    let mut game = HangmanGame::new(options.attempts).unwrap();
    game.solution = solution.to_string();

    let stdin = io::stdin();
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
        if let Err(_) = stdin.read_line(&mut input) {
            println!("\nInvalid input!");
            continue;
        };

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
