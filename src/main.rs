mod game;

fn main() {
    println!("This project is meant to create a simple chess game, to pratice with Rust.");
    
    let mut new_board = game::Board{cases:[[None; 8]; 8]};
    new_board.init_pawns();
    new_board.print();
}

