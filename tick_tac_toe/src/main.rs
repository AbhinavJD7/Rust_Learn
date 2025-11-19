use std::io;

const PLAYER_X:char='X';
const PLAYER_O:char='O';
const BOARD_SIZE:usize=3;

//2d array define of 3x3
type Board = [[char;BOARD_SIZE];BOARD_SIZE]; //type is like making an alias

fn initialize_board()-> Board{
    return [['';BOARD_SIZE];BOARD_SIZE];
}
fn print_board(board:&Board){
    for row in board
    {
        for cell in row
        {
            print!("{}",cell);
        }
        println!();
    }   
}
fn play_game(){
    let mut board = initialize_board();
    let mut current_player = PLAYER_X;

    loop{
    println!("Current Board: ");
    print_board(&board);
    let (row,col)=get_player_move(current_player, &board);
    board[row
    current_player = if current_player==PLAYER_X{
        PLAYER_O;
    } else {
        PLAYER_X;
    }
  } 
}
fn main() {
   println!("Welcome to the tick tac toe Game");
   play_game();

}
