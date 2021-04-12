// Converts board pos ([A-H][1-8]) to [0-7][0-7]
pub fn convert_board_pos_to_array_entry(file_and_rank:&str) -> Option<[usize; 2]>
{
    if file_and_rank.len() != 2
    {
        return None;
    }
    // no test here, we assume that the caller checked the format before !
    let to_upper = file_and_rank.to_uppercase();
    let file = to_upper.chars().nth(0).unwrap();
    let rank = to_upper.chars().nth(1).unwrap();
    let file_entry:usize;
    let rank_entry:usize = (rank.to_digit(10).unwrap() - 1) as usize ;

    if rank_entry > 7
    {
        return None
    }
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
        _ => return None
    }
    
    return Some([file_entry, rank_entry])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_entries() 
    {
        assert_eq!(convert_board_pos_to_array_entry("A1"), Some([0,0]));
        assert_eq!(convert_board_pos_to_array_entry("A2"), Some([0,1]));
        assert_eq!(convert_board_pos_to_array_entry("A3"), Some([0,2]));
        assert_eq!(convert_board_pos_to_array_entry("A4"), Some([0,3]));
        assert_eq!(convert_board_pos_to_array_entry("A5"), Some([0,4]));
        assert_eq!(convert_board_pos_to_array_entry("A6"), Some([0,5]));
        assert_eq!(convert_board_pos_to_array_entry("A7"), Some([0,6]));
        assert_eq!(convert_board_pos_to_array_entry("A8"), Some([0,7]));
        assert_eq!(convert_board_pos_to_array_entry("B1"), Some([1,0]));
        assert_eq!(convert_board_pos_to_array_entry("B2"), Some([1,1]));
        assert_eq!(convert_board_pos_to_array_entry("B3"), Some([1,2]));
        assert_eq!(convert_board_pos_to_array_entry("B4"), Some([1,3]));
        assert_eq!(convert_board_pos_to_array_entry("B5"), Some([1,4]));
        assert_eq!(convert_board_pos_to_array_entry("B6"), Some([1,5]));
        assert_eq!(convert_board_pos_to_array_entry("B7"), Some([1,6]));
        assert_eq!(convert_board_pos_to_array_entry("B8"), Some([1,7]));
        assert_eq!(convert_board_pos_to_array_entry("C1"), Some([2,0]));
        assert_eq!(convert_board_pos_to_array_entry("C2"), Some([2,1]));
        assert_eq!(convert_board_pos_to_array_entry("C3"), Some([2,2]));
        assert_eq!(convert_board_pos_to_array_entry("C4"), Some([2,3]));
        assert_eq!(convert_board_pos_to_array_entry("C5"), Some([2,4]));
        assert_eq!(convert_board_pos_to_array_entry("C6"), Some([2,5]));
        assert_eq!(convert_board_pos_to_array_entry("C7"), Some([2,6]));
        assert_eq!(convert_board_pos_to_array_entry("C8"), Some([2,7]));
        assert_eq!(convert_board_pos_to_array_entry("D1"), Some([3,0]));
        assert_eq!(convert_board_pos_to_array_entry("D2"), Some([3,1]));
        assert_eq!(convert_board_pos_to_array_entry("D3"), Some([3,2]));
        assert_eq!(convert_board_pos_to_array_entry("D4"), Some([3,3]));
        assert_eq!(convert_board_pos_to_array_entry("D5"), Some([3,4]));
        assert_eq!(convert_board_pos_to_array_entry("D6"), Some([3,5]));
        assert_eq!(convert_board_pos_to_array_entry("D7"), Some([3,6]));
        assert_eq!(convert_board_pos_to_array_entry("D8"), Some([3,7]));
        assert_eq!(convert_board_pos_to_array_entry("E1"), Some([4,0]));
        assert_eq!(convert_board_pos_to_array_entry("E2"), Some([4,1]));
        assert_eq!(convert_board_pos_to_array_entry("E3"), Some([4,2]));
        assert_eq!(convert_board_pos_to_array_entry("E4"), Some([4,3]));
        assert_eq!(convert_board_pos_to_array_entry("E5"), Some([4,4]));
        assert_eq!(convert_board_pos_to_array_entry("E6"), Some([4,5]));
        assert_eq!(convert_board_pos_to_array_entry("E7"), Some([4,6]));
        assert_eq!(convert_board_pos_to_array_entry("E8"), Some([4,7]));
        assert_eq!(convert_board_pos_to_array_entry("F1"), Some([5,0]));
        assert_eq!(convert_board_pos_to_array_entry("F2"), Some([5,1]));
        assert_eq!(convert_board_pos_to_array_entry("F3"), Some([5,2]));
        assert_eq!(convert_board_pos_to_array_entry("F4"), Some([5,3]));
        assert_eq!(convert_board_pos_to_array_entry("F5"), Some([5,4]));
        assert_eq!(convert_board_pos_to_array_entry("F6"), Some([5,5]));
        assert_eq!(convert_board_pos_to_array_entry("F7"), Some([5,6]));
        assert_eq!(convert_board_pos_to_array_entry("F8"), Some([5,7]));
        assert_eq!(convert_board_pos_to_array_entry("G1"), Some([6,0]));
        assert_eq!(convert_board_pos_to_array_entry("G2"), Some([6,1]));
        assert_eq!(convert_board_pos_to_array_entry("G3"), Some([6,2]));
        assert_eq!(convert_board_pos_to_array_entry("G4"), Some([6,3]));
        assert_eq!(convert_board_pos_to_array_entry("G5"), Some([6,4]));
        assert_eq!(convert_board_pos_to_array_entry("G6"), Some([6,5]));
        assert_eq!(convert_board_pos_to_array_entry("G7"), Some([6,6]));
        assert_eq!(convert_board_pos_to_array_entry("G8"), Some([6,7]));
        assert_eq!(convert_board_pos_to_array_entry("H1"), Some([7,0]));
        assert_eq!(convert_board_pos_to_array_entry("H2"), Some([7,1]));
        assert_eq!(convert_board_pos_to_array_entry("H3"), Some([7,2]));
        assert_eq!(convert_board_pos_to_array_entry("H4"), Some([7,3]));
        assert_eq!(convert_board_pos_to_array_entry("H5"), Some([7,4]));
        assert_eq!(convert_board_pos_to_array_entry("H6"), Some([7,5]));
        assert_eq!(convert_board_pos_to_array_entry("H7"), Some([7,6]));
        assert_eq!(convert_board_pos_to_array_entry("H8"), Some([7,7]));
    }

    #[test]
    fn test_too_long_entry()
    {
        assert_eq!(convert_board_pos_to_array_entry("Hello"), None);
    }
    
    #[test]
    fn test_not_on_board()
    {
        assert_eq!(convert_board_pos_to_array_entry("A9"), None);
    }
    
    #[test]
    fn test_empty()
    {
        assert_eq!(convert_board_pos_to_array_entry(""), None);
    }
}