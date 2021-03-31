use regex::Regex;

#[path="util.rs"]
mod util;

/// Struct to represent Pawn in chess game
#[derive(Copy, Clone)]
struct Pawn
{
    is_white:bool
}

impl Pawn 
{
    pub fn how_can_i_move()
    {
        println!("I am a pawn, I can move forward to the unoccupied square immediately \
        in front of me on the same file, or on my first move I can advance two squares \
        along the same file ");
    }

    pub fn print(&self)
    {
        match self.is_white {
            true => { print!("W"); }
            false => { print!("B"); }
        }
    }
}

/// Struct to reprensent board in chess game
struct Board
{
    /// cases[file(A-H)][rank(1-8)] -- direct access [0-7][0-7] of course
    pub cases: [[Option<Pawn>; 8]; 8]
}

impl Board
{
    /// Put 16 White & 16 Black pawn on empty board 
    pub fn init_pawns(&mut self)
    {
        for i_file in 0..8 //file in cases
        {
            for i_rank in 0..2 //rank in file
            {
                self.cases[i_file][i_rank] = Some(Pawn{is_white:true}); 
            }
        }

        for i_file in 0..8 //file in cases
        {
            for i_rank in 6..8 //rank in file
            {
                self.cases[i_file][i_rank] = Some(Pawn{is_white:false}); 
            }
        }
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
                    Some(pawn) => { pawn.print(); }
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
        // A: Check "from" is within A1<->H8

        // B: Check there is a piece on "from"
        // C: Check that for the piece at "from" the move to "to" is authorized
        // D: Move the option from "from" to "to"
            // --> if the case "to" wasn't empty, print the piece that was taken
            // --> Check if there is a check or a mate
        let re = Regex::new(r"[a-hA-H][1-8]").unwrap();
        //println!("move_from_to(): from matches A1<->H8 {}", re.is_match(&from));
        if !re.is_match(&from)
        {
            println!("Your input \"from\"={} is not within A1 && H8 ", &from);
        }
        else
        {
            let array_entries = util::convert_board_pos_to_array_entry(&from);
            println!("For {}, array entries are {} - {}", &from, array_entries[0], array_entries[1]);
        }
    }
}