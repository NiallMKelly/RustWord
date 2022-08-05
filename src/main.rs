mod game;
use colored::Colorize;
use game::Game;

fn main() {
    let mut game = Game::new();

    welcome_message();

    while !game.game_over() {
        // Prompt for the users input
        game.show_guesses();
        let _input = game.wait_for_guess();
    }

    game.show_guesses();
}

fn welcome_message() {
    println!("Welcome to {}{}!\n", "Rust".green(), "Word".yellow());
    println!("Guess the word in 6 tries.");
    println!("Each of your guesses must be a valid 5-letter word.");
    println!("After each guess, the color of the letters will change to show how close your guess was to the word.");
    println!("A {} letter means that the letter is in the word and in the correct spot.", "green".green());
    println!("A {} letter means that the letter is in the word but not in the correct.", "yellow".yellow());
    println!("A {} letter means that the letter is not in the word in any spot.\n", "gray".dimmed());
}