mod game;
use game::Game;

fn main() {
    println!("Welcome to RustWord!");

    let mut game = Game::new();

    while !game.game_over() {
        // Prompt for the users input
        let input = game.wait_for_guess();
        game.check_guess(input);
    }
}
