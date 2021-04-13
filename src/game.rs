use regex::Regex;

#[path="util.rs"]
mod util;

mod piece
{
    /// Struct to represent Pawn in chess game
    #[derive(Copy, Clone, Debug, PartialEq, Eq)] 
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
        pub fn is_white(&self) -> bool
        {
            return self.is_white;
        }

        pub fn my_type(&self) -> Type
        {
            return self.p_type;
        }

        pub fn description(&self) -> &str
        {
            match self.p_type
            {
                Type::Pawn => {"I am a pawn, I can move forward to the unoccupied square immediately in front of me on the same file, or on my first move I can advance two squares along the same file "}
                Type::King => {"I move one square in any direction. I also have a special move called castling that involves also moving a rook."}
                Type::Queen => {"I combine the power of a rook and bishop and can move any number of squares along a rank, file, or diagonal, but cannot leap over other pieces."}
                Type::Rook => {"I can move any number of squares along a rank or file, but cannot leap over other pieces. Along with the king, I am involved during the king's castling move."}
                Type::Bishop => {"I can move any number of squares diagonally, but cannot leap over other pieces."}
                Type::Knight => {"I move to any of the closest square that are not on the same rank, file, or diagonal. (Thus the move forms an \"L\"-shape: two squares vertically and one square horizontally, or two squares horizontally and one square vertically.) The knight is the only piece that can leap over other pieces."}
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
            Piece{is_white:white, p_type:piece_type}
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
    pub fn new() -> Board
    {
        Board{cases:[[None; 8]; 8]}
    }
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
        self.cases[3][0] = Some(piece::Piece::new_piece(true, piece::Type::Queen));
        self.cases[3][7] = Some(piece::Piece::new_piece(true, piece::Type::Queen)); 

        // Kings
        self.cases[4][0] = Some(piece::Piece::new_piece(true, piece::Type::King));
        self.cases[4][7] = Some(piece::Piece::new_piece(true, piece::Type::King));
    }

    /// From a given position, tells you where you can move your pawn.
    /*pub fn print_authorized_moves(&self, file:char, rank:u8)
    {
        if file < 'A' || file > 'H' || rank < 1 || rank > 8
        {
            println!("This position doesn't exist.")
        }

        // TODO : Check piece on position, print available positions in consequence
    }*/

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
        let mut new_board = Board::new();
        new_board.init(); 
        Game{board:new_board}        
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

#[cfg(test)]
mod test
{
    use super::*;
    #[test]
    fn check_piece_color() 
    {
        let piece1 = piece::Piece::new_piece(true, piece::Type::Pawn);
        let piece2 = piece::Piece::new_piece(false, piece::Type::Pawn);

        assert_eq!(piece1.is_white(), true);
        assert_eq!(piece2.is_white(), false);
    }

    #[test]
    fn check_piece_type() {        
        let piece1 = piece::Piece::new_piece(true, piece::Type::Pawn);
        let piece2 = piece::Piece::new_piece(true, piece::Type::King);
        let piece3 = piece::Piece::new_piece(true, piece::Type::Queen);
        let piece4 = piece::Piece::new_piece(true, piece::Type::Knight);
        let piece5 = piece::Piece::new_piece(true, piece::Type::Bishop);
        let piece6 = piece::Piece::new_piece(true, piece::Type::Rook);

        assert_eq!(piece1.my_type(), piece::Type::Pawn);
        assert_eq!(piece2.my_type(), piece::Type::King);
        assert_eq!(piece3.my_type(), piece::Type::Queen);
        assert_eq!(piece4.my_type(), piece::Type::Knight);
        assert_eq!(piece5.my_type(), piece::Type::Bishop);
        assert_eq!(piece6.my_type(), piece::Type::Rook);
    }

    #[test]
    fn check_descriptions() {
        let piece1 = piece::Piece::new_piece(true, piece::Type::Pawn);
        let piece2 = piece::Piece::new_piece(true, piece::Type::King);
        let piece3 = piece::Piece::new_piece(true, piece::Type::Queen);
        let piece4 = piece::Piece::new_piece(true, piece::Type::Knight);
        let piece5 = piece::Piece::new_piece(true, piece::Type::Bishop);
        let piece6 = piece::Piece::new_piece(true, piece::Type::Rook);
        assert_ne!(piece1.description().len(), 0);
        assert_ne!(piece2.description().len(), 0);
        assert_ne!(piece3.description().len(), 0);
        assert_ne!(piece4.description().len(), 0);
        assert_ne!(piece5.description().len(), 0);
        assert_ne!(piece6.description().len(), 0);
    }
}