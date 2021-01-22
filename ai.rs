pub extern "C" fn ai(
    board: *const [[u32; 10]; 24],
    current_piece: *const [[i32; 2]; 4],
    saved_piece: *const [[i32; 2]; 4],
) -> u32 {
    let mut action = 3;
    if board[0][0] != 0{
        action = 6;
    } 
    action
}