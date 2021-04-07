// Converts board pos ([A-H][1-8]) to [0-7][0-7]
pub fn convert_board_pos_to_array_entry(file_and_rank:&str) -> [usize; 2]
{
    // no test here, we assume that the caller checked the format before !
    let to_upper = file_and_rank.to_uppercase();
    let file = to_upper.chars().nth(0).unwrap();
    let rank = to_upper.chars().nth(1).unwrap();
    let mut file_entry:usize = 0;
    let rank_entry:usize = (rank.to_digit(10).unwrap() - 1) as usize ;
    match file
    {
        'A' => {file_entry = 0;}
        'B' => {file_entry = 1;}
        'C' => {file_entry = 2;}
        'D' => {file_entry = 3;}
        'E' => {file_entry = 4;}
        'F' => {file_entry = 5;}
        'G' => {file_entry = 6;}
        'H' => {file_entry = 7;}
        _ => {println!("Hum... Have you checked the entry before calling this function?");}
    }
    
    return [file_entry, rank_entry]
}