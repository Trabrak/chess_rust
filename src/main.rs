mod game;

fn main() {
    println!("This project is meant to create a simple chess game, to pratice with Rust.");
    let new_game: game::Game = game::Game::new();
    new_game.move_from_to(&"A7", &"A3");
}

