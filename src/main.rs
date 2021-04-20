mod game;

use text_io::read;
fn main() {
    println!("This project is meant to create a simple chess game, to pratice with Rust.");
    println!("At any moment, enter q to quit the game.");

    let mut new_game: game::Game = game::Game::new();

    loop
    {
        new_game.print();
        println!("Please enter a case to choose a piece to move");
        let from: String = read!();
        if from == "q"
        {
            break;
        }
        println!("Please enter a case for your piece to be moved");
        let to: String = read!();
        if to == "q"
        {
            break;
        }
        new_game.move_from_to(&from, &to);
    }
}

