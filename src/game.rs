/// Struct to represent Pawn in chess game
#[derive(Copy, Clone)]
pub struct Pawn
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
            _ => { print!("*"); }
          }
    }
}

/// Struct to reprensent board in chess game
pub struct Board
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
        for i_file in 0..8 //file in cases
        {
            for i_rank in 0..8 //rank in file
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

}