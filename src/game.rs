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
            let mut piece_print:String;
            match self.my_type()
            {
                Type::Pawn => {piece_print = String::from("Pa");}
                Type::King => {piece_print = String::from("Ki");}
                Type::Queen => {piece_print = String::from("Qu");}
                Type::Rook => {piece_print = String::from("Ro");}
                Type::Bishop => {piece_print = String::from("Bi");}
                Type::Knight => {piece_print = String::from("Kn");}
            } 
            match self.is_white {
                true => { piece_print += "W"; }
                false => { piece_print += "B"; }
            }
            print!("{}", piece_print);
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
    cases: [[Option<piece::Piece>; 8]; 8]
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
        self.set_case_content_by_str(Some(piece::Piece::new_piece(true, piece::Type::Rook)), &"A1");
        self.set_case_content_by_str(Some(piece::Piece::new_piece(true, piece::Type::Rook)), &"H1");
        self.set_case_content_by_str(Some(piece::Piece::new_piece(false, piece::Type::Rook)), &"A8");
        self.set_case_content_by_str(Some(piece::Piece::new_piece(false, piece::Type::Rook)), &"H8");

        // Knights
        self.set_case_content_by_str(Some(piece::Piece::new_piece(true, piece::Type::Knight)), &"B1");
        self.set_case_content_by_str(Some(piece::Piece::new_piece(true, piece::Type::Knight)), &"G1");
        self.set_case_content_by_str(Some(piece::Piece::new_piece(false, piece::Type::Knight)), &"B8");
        self.set_case_content_by_str(Some(piece::Piece::new_piece(false, piece::Type::Knight)), &"G8");

        // Bishops
        self.set_case_content_by_str(Some(piece::Piece::new_piece(true, piece::Type::Bishop)), &"C1");
        self.set_case_content_by_str(Some(piece::Piece::new_piece(true, piece::Type::Bishop)), &"F1");
        self.set_case_content_by_str(Some(piece::Piece::new_piece(false, piece::Type::Bishop)), &"C8");
        self.set_case_content_by_str(Some(piece::Piece::new_piece(false, piece::Type::Bishop)), &"F8");

        // Queens
        self.set_case_content_by_str(Some(piece::Piece::new_piece(true, piece::Type::Queen)), &"D1");
        self.set_case_content_by_str(Some(piece::Piece::new_piece(false, piece::Type::Queen)), &"D8");

        // Kings
        self.set_case_content_by_str(Some(piece::Piece::new_piece(true, piece::Type::King)), &"E1");
        self.set_case_content_by_str(Some(piece::Piece::new_piece(false, piece::Type::King)), &"E8");
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
        println!("     A   B   C   D   E   F   G   H\n");
        for i_rank in 0..8 //rank in cases
        {
            print!("{}   ", i_rank + 1);
            for i_file in 0..8 //file in rank
            {
                match self.cases[i_file][i_rank]
                {
                    Some(piece) => { piece.print(); }
                    None => { print!("***") }
                }
                print!(" ");
            }
            println!();
        }
    }

    pub fn get_case_content(&self, file:usize, rank:usize) -> Option<piece::Piece>
    {
        return self.cases[file][rank]
    }
    pub fn get_case_content_by_str(&self, file_and_rank:&str) -> Option<piece::Piece>
    {
        let file_rank_pos:Option<[usize; 2]> = util::convert_board_pos_to_array_entry(&file_and_rank);
        if file_rank_pos.is_none()
        {
            return None
        }
        let unwrapped = file_rank_pos.unwrap();
        return self.cases[unwrapped[0]][unwrapped[1]]
    }

    pub fn set_case_content(&mut self, piece:Option<piece::Piece>, file:usize, rank:usize)
    {
        self.cases[file][rank] = piece;
    }

    pub fn set_case_content_by_str(&mut self, piece:Option<piece::Piece>, file_and_rank:&str)
    {
        let file_rank_pos:Option<[usize; 2]> = util::convert_board_pos_to_array_entry(&file_and_rank);
        if file_rank_pos.is_some()
        {
            let unwrapped = file_rank_pos.unwrap();
            self.cases[unwrapped[0]][unwrapped[1]] = piece;
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

    pub fn print(&self)
    {
        self.board.print();
    }
    
    /// Function that takes a position, and a destination, and checks if it's possible
    /// Considers that both of positions are realistic && self.board.is_some()
    /// True if you can move, false if you can't
    pub fn check_move(&self, from:&[usize; 2], to:&[usize; 2]) -> bool
    {
        /*
        *   Type::Pawn => (On start = (0;1) (0;2), else (0;1)) + if enemy in (-1;+1) || (+1;+1) then possible
        *   Type::King => +1;0, +1;+1, 0;+1, -1;+1, -1;0, -1;-1, 0;-1, +1;-1 => abs(mvt file) == 1 || abs(mvt rank) == 1 && still on board && no friendly piece there
        *   Type::Queen => abs(file) == abs(rank) || abs(file|rank) == 0 && for each case in direction, there is no piece (excepted on dest if it's enemy)
        *   Type::Rook => abs(file) == 0 XOR abs(rank) == 0 && for each case in direction, there is no piece (excepted on dest if it's enemy)
        *   Type::Bishop => abs(file) == abs(rank) && for each case in direction, there is no piece (excepted on dest if it's enemy)
        *   Type::Knight => (abs(file) == 2 && abs(rank) == 1) || (abs(file) == 1 && abs(rank) == 2) && dest = None || enemy 
        */
        let piece_to_check = self.board.get_case_content(from[0], from[1]).unwrap();
        match piece_to_check.my_type()
        {
            piece::Type::Pawn => 
            { 
                let mut value = from[0] == to[0];
                if piece_to_check.is_white()
                {
                    // Move forward, no one is ahead
                    value = value && (((from[1] == 1) && (to[1] == 3)) || (to[1] == from[1] + 1));
                    for file in 2..(to[1] + 1) as usize
                    {
                        value = value && (self.board.get_case_content(to[0], file).is_none());
                    }

                    // Let's eat another piece in diag
                    if !value
                    {
                        value = ((to[0] as i32 - from[0] as i32).abs() == 1) && ((to[1] - from[1]) == 1);
                        value = value && self.board.get_case_content(to[0], to[1]).is_some() && !self.board.get_case_content(to[0], to[1]).unwrap().is_white();
                    }
                }
                else
                {
                    // Move forward, no one is ahead
                    value = value && (((from[1] == 6) && (to[1] == 4)) || (to[1] == from[1] - 1));
                    eprintln!("should be ok{}", value);
                    for file in to[1]..6 as usize
                    {
                        value = value && (self.board.get_case_content(to[0], file).is_none());
                    }

                    // Let's eat another piece in diag
                    if !value
                    {
                        value = ((to[0] as i32 - from[0] as i32).abs() == 1) && ((from[1] - to[1]) == 1);
                        value = value && self.board.get_case_content(to[0], to[1]).is_some() && self.board.get_case_content(to[0], to[1]).unwrap().is_white();
                    }
                }
                return value
            }
            piece::Type::King => { return false }
            piece::Type::Queen => { return false }
            piece::Type::Rook => { return false }
            piece::Type::Bishop => { return false }
            piece::Type::Knight => { return false }
        }
    }

    pub fn move_from_to(&mut self, from:&str, to:&str)
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
            let from_array_entries = util::convert_board_pos_to_array_entry(&from).unwrap();
            let to_array_entries = util::convert_board_pos_to_array_entry(&to).unwrap();
            //println!("For {}, array entries are {} - {}", &from, array_entries[0], array_entries[1]);
            let piece_to_move = self.board.get_case_content(from_array_entries[0], from_array_entries[1]);
            if piece_to_move.is_some()
            {
                // C: Check that for the piece at "from" the move to "to" is authorized
                if self.check_move(&from_array_entries, &to_array_entries)
                {                    
                    // D: Move the option from "from" to "to"
                        // --> if the case "to" wasn't empty, print the piece that was taken
                        // --> Check if there is a check or a mate
                    self.board.set_case_content(piece_to_move, to_array_entries[0], to_array_entries[1]);
                    self.board.set_case_content(None, from_array_entries[0], from_array_entries[1]);
                }
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

    #[test]
    fn check_board_get_set() {
        let mut board = Board::new();
        let piece1 = piece::Piece::new_piece(true, piece::Type::Pawn);
        board.set_case_content(Some(piece1), 0, 0);
        let get_content1 = board.get_case_content(0, 0);
        let get_content2 = board.get_case_content(0, 1);

        assert!(get_content1.is_some());
        assert!(get_content2.is_none());
        assert_eq!(get_content1.unwrap().my_type(), piece::Type::Pawn);
        assert_eq!(get_content1.unwrap().is_white(), true);
    }

    #[test]
    fn check_pawn_move_white() {
        let game = Game::new();
        assert!(game.check_move(&[1,1], &[1,2]));
        assert!(game.check_move(&[1,1], &[1,3]));   
        assert!(!game.check_move(&[1,1], &[1,4]));           
    }

    #[test]
    fn check_pawn_move_black() {
        let game = Game::new();
        assert!(game.check_move(&[6,6], &[6,5]));
        assert!(game.check_move(&[6,6], &[6,4]));   
        assert!(!game.check_move(&[6,6], &[6,3]));           
    }

    #[test]
    fn check_board_init() {
        let mut board = Board::new();
        board.init();
        assert!(board.get_case_content(0, 0).is_some());
        assert!(board.get_case_content(1, 0).is_some());
        assert!(board.get_case_content(2, 0).is_some());
        assert!(board.get_case_content(3, 0).is_some());
        assert!(board.get_case_content(4, 0).is_some());
        assert!(board.get_case_content(5, 0).is_some());
        assert!(board.get_case_content(6, 0).is_some());
        assert!(board.get_case_content(7, 0).is_some());

        assert!(board.get_case_content(0, 1).is_some());
        assert!(board.get_case_content(1, 1).is_some());
        assert!(board.get_case_content(2, 1).is_some());
        assert!(board.get_case_content(3, 1).is_some());
        assert!(board.get_case_content(4, 1).is_some());
        assert!(board.get_case_content(5, 1).is_some());
        assert!(board.get_case_content(6, 1).is_some());
        assert!(board.get_case_content(7, 1).is_some());
        
        assert!(board.get_case_content(0, 7).is_some());
        assert!(board.get_case_content(1, 7).is_some());
        assert!(board.get_case_content(2, 7).is_some());
        assert!(board.get_case_content(3, 7).is_some());
        assert!(board.get_case_content(4, 7).is_some());
        assert!(board.get_case_content(5, 7).is_some());
        assert!(board.get_case_content(6, 7).is_some());
        assert!(board.get_case_content(7, 7).is_some());
        
        assert!(board.get_case_content(0, 7).is_some());
        assert!(board.get_case_content(1, 7).is_some());
        assert!(board.get_case_content(2, 7).is_some());
        assert!(board.get_case_content(3, 7).is_some());
        assert!(board.get_case_content(4, 7).is_some());
        assert!(board.get_case_content(5, 7).is_some());
        assert!(board.get_case_content(6, 7).is_some());
        assert!(board.get_case_content(7, 7).is_some());
    }
}