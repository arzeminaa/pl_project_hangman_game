use std::io::{self, Write};
use clap::Parser;
use hangman::game::HangmanGame;
use hangman::input::{Command, Options};
use hangman::tui;
use hangman::game::get_random_word;
use hangman::game::GameMode;
use hangman::scoreboard::Scoreboard;

fn main() {
    let options = Options::parse();
    let stdin = io::stdin();

    let mut scoreboard = Scoreboard::new();

    println!("Choose game mode: (1) Single Player, (2) Multiplayer");
    let mut input = String::new();
    stdin.read_line(&mut input).expect("Failed to read input");
    let game_mode = match input.trim() {
        "1" => GameMode::SinglePlayer,
        "2" => GameMode::MultiPlayer,
        _ => {
            println!("Invalid choice, defaulting to Single Player mode");
            GameMode::SinglePlayer
        }
    };

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
                start_new_game(options.attempts, game_mode)
            }
        }
    } else {
        // Start a new game
        start_new_game(options.attempts, game_mode)
    };

    if !options.debug {
        tui::clear_screen(false);
    } else {
        println!("\n[DEBUG MODE: Screen clearing disabled]\n");
    }
    println!("Let's play hangman!");

    loop {
        let mut input = String::new();
        tui::draw_hangman(game.attempts_remaining);

        println!("{}", game);
        println!("Score: {}", game.score);

        if game.is_over() {
            scoreboard.update_score("Player1", game.score);
            break;
        }

        print!("Enter command: ");
        io::stdout().flush().unwrap();

        input.clear();
        stdin.read_line(&mut input).expect("Failed to read input");

        let command = match input.parse::<Command>() {
            Ok(c) => c,
            Err(e) => {
                println!("\n{}\n", e);
                continue;
            }
        };

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

fn start_new_game(attempts: u32, game_mode: GameMode) -> HangmanGame {
    let solution = get_random_word();
    let mut game = HangmanGame::new(attempts, game_mode).unwrap();
    game.solution = solution.to_string();
    game
}
