use regex::Regex;

#[path="util.rs"]
mod util;

mod piece
{
    /// Struct to represent Pawn in chess game
    #[derive(Copy, Clone)] 
    pub enum Type
    {
        Pawn,
        King,
        Queen,
        Rook,
        Bishop,
        Knight
    }

    /// Struct to represent Pawn in chess game
    #[derive(Copy, Clone)]    
    pub struct Piece
    {
        is_white:bool,
        p_type:Type
    }

    impl Piece 
    {
        pub fn description(&self)
        {
            match self.p_type
            {
                Pawn => {println!("I am a pawn, I can move forward to the unoccupied square immediately in front of me on the same file, or on my first move I can advance two squares along the same file ");}
                King => { println!("I move one square in any direction. I also have a special move called castling that involves also moving a rook.");}
                Queen => {println!("I combine the power of a rook and bishop and can move any number of squares along a rank, file, or diagonal, but cannot leap over other pieces.");}
                Rook => {println!(" Ican move any number of squares along a rank or file, but cannot leap over other pieces. Along with the king, I am involved during the king's castling move.");}
                Bishop => {println!("I can move any number of squares diagonally, but cannot leap over other pieces.");}
                Knight => {println!("I move to any of the closest square that are not on the same rank, file, or diagonal. (Thus the move forms an \"L\"-shape: two squares vertically and one square horizontally, or two squares horizontally and one square vertically.) The knight is the only piece that can leap over other pieces.");}
            }            
        }
    
        pub fn print(&self)
        {
            match self.is_white {
                true => { print!("W"); }
                false => { print!("B"); }
            }
        }

        pub fn new_piece(white:bool, piece_type:Type) -> Piece
        {
            Piece{is_white:true, p_type:piece_type}
        }
    }
}

/// Struct to reprensent board in chess game
struct Board
{
    /// cases[file(A-H)][rank(1-8)] -- direct access [0-7][0-7] of course
    pub cases: [[Option<piece::Piece>; 8]; 8]
}

impl Board
{
    /// Put 16 White & 16 Black pieces on empty board 
    pub fn init(&mut self)
    {
        // Pawns
        for i_file in 0..8 //file in cases
        {
            for i_rank in 1..2 //rank in file
            {
                self.cases[i_file][i_rank] = Some(piece::Piece::new_piece(true, piece::Type::Pawn));
            }
        }
        for i_file in 0..8 //file in cases
        {
            for i_rank in 6..7 //rank in file
            {
                self.cases[i_file][i_rank] = Some(piece::Piece::new_piece(false, piece::Type::Pawn)); 
            }
        }
        // Rooks
        self.cases[0][0] = Some(piece::Piece::new_piece(true, piece::Type::Rook));
        self.cases[7][0] = Some(piece::Piece::new_piece(true, piece::Type::Rook)); 
        self.cases[0][7] = Some(piece::Piece::new_piece(false, piece::Type::Rook)); 
        self.cases[7][7] = Some(piece::Piece::new_piece(false, piece::Type::Rook)); 

        // Knights
        self.cases[1][0] = Some(piece::Piece::new_piece(true, piece::Type::Knight));
        self.cases[6][0] = Some(piece::Piece::new_piece(true, piece::Type::Knight)); 
        self.cases[1][7] = Some(piece::Piece::new_piece(false, piece::Type::Knight)); 
        self.cases[6][7] = Some(piece::Piece::new_piece(false, piece::Type::Knight)); 

        // Bishops
        self.cases[2][0] = Some(piece::Piece::new_piece(true, piece::Type::Bishop));
        self.cases[5][0] = Some(piece::Piece::new_piece(true, piece::Type::Bishop)); 
        self.cases[2][7] = Some(piece::Piece::new_piece(false, piece::Type::Bishop)); 
        self.cases[5][7] = Some(piece::Piece::new_piece(false, piece::Type::Bishop)); 

        // Queens
        self.cases[3][0] = Some(piece::Piece::new_piece(true, piece::Type::Bishop));
        self.cases[3][7] = Some(piece::Piece::new_piece(true, piece::Type::Bishop)); 

        // Kings
        self.cases[4][0] = Some(piece::Piece::new_piece(true, piece::Type::Bishop));
        self.cases[4][7] = Some(piece::Piece::new_piece(true, piece::Type::Bishop));
    }

    /// From a given position, tells you where you can move your pawn.
    pub fn print_authorized_moves(&self, file:char, rank:u8)
    {
        if file < 'A' || file > 'H' || rank < 1 || rank > 8
        {
            println!("This position doesn't exist.")
        }

        // TODO : Check piece on position, print available positions in consequence
    }

    pub fn print(&self)
    {
        println!("    A B C D E F G H\n");
        for i_rank in 0..8 //rank in cases
        {
            print!("{}   ", i_rank + 1);
            for i_file in 0..8 //file in rank
            {
                match self.cases[i_file][i_rank]
                {
                    Some(piece) => { piece.print(); }
                    None => { print!("*") }
                }
                print!(" ");
            }
            println!();
        }
    }
}

/// Struct to manage chess game
pub struct Game
{ 
    board:Board,
}

impl Game
{
    pub fn new() -> Game
    {
        Game {board:Board{cases:[[None; 8]; 8]}}
    }

    pub fn move_from_to(&self, from:&str, to:&str)
    {             
        let re = Regex::new(r"[a-hA-H][1-8]").unwrap();
        //println!("move_from_to(): from matches A1<->H8 {}", re.is_match(&from));
        if !re.is_match(&from)
        {
            // A: Check "from" is within A1<->H8
            println!("Your input \"from\"={} is not within A1 && H8 ", &from);
        }
        else
        {
            let array_entries = util::convert_board_pos_to_array_entry(&from).unwrap();
            //println!("For {}, array entries are {} - {}", &from, array_entries[0], array_entries[1]);
            if self.board.cases[array_entries[0]][array_entries[1]].is_some()
            {
                // do stuff
                // C: Check that for the piece at "from" the move to "to" is authorized
                // D: Move the option from "from" to "to"
                    // --> if the case "to" wasn't empty, print the piece that was taken
                    // --> Check if there is a check or a mate
            }
            else
            {
                // B: Check there is a piece on "from"
                println!("There is no piece on {}", &from);
            }
        }
    }
}