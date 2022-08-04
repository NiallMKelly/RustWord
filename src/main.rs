mod game;
use game::Game;

fn main() {
    println!("Welcome to RustWord!");

    let mut game = Game::new();

    while !game.game_over() {
        // Prompt for the users input
        game.show_guesses();
        let input = game.wait_for_guess();
    }

    game.show_guesses();
}
