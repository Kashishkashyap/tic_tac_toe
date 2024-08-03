use std::io;

const PLAYER_X:char='X';
const PLAYER_Y:char='O';

const BOARD_SIZE:usize= 3;
type BOARD=[[char;BOARD_SIZE];BOARD_SIZE];

fn initialize_board()->BOARD{
    return [[' ';BOARD_SIZE];BOARD_SIZE];

}
fn print_board(board:&BOARD){
    for row in board{
        for cell in row{
            print!("{}",cell);
        }
        println!();
    }
}

fn play(){
    let mut board= initialize_board(); 
    let mut current_player= PLAYER_X;

    loop{
        println!("Current Board: ");
        print_board(&board);

        let (row,col)= get_player_move(current_player, &board);
        board[row][col]= current_player;

        if check_winner(current_player,&board){
            println!("{} is the winner", current_player);
            break;
        }
        if check_draw(&board){
            println!("Match Draw");
            break;
        }
        
        current_player= if current_player== PLAYER_X{
            PLAYER_Y
        }else{
            PLAYER_X
        }
    }

}

fn get_player_move(current_player:char, board:&BOARD)->(usize, usize){ 

    loop{
        println!("{} play your move in (Row, col) format", current_player);
        let mut input= String::new();
        io::stdin().read_line(&mut input).expect("Failed to load");    // here we will receive input in "0 1" format i.e., a string, so wee to split it to "0" "1" and then to array 0, 1

        let coordinates:Vec<usize>= input                                                  //"0 1"
                                        .split_whitespace()                                 //"0""1"
                                        .flat_map(str::parse::<usize>)                        //0 1
                                        .collect();                                         //make an vector of 0 1

        if coordinates.len()==2{
            let (row, col)= (coordinates[0],coordinates[1]);
            if row<BOARD_SIZE && col<BOARD_SIZE && board[row][col]==' '{
                return (row,col);
            }
        }
        println!("Invalid input");
    }
   

}

fn check_winner(current_player:char, board:&BOARD)->bool{
    for row in 0..BOARD_SIZE{
        if board[row][0]== current_player && board[row][1]== board[row][0] && board[row][2]== board[row][1]{
            return true;
        }
    }
    for col in 0..BOARD_SIZE{
        if board[0][col]== current_player && board[1][col]== board[0][col] && board[2][col]== board[1][col]{
            return true;
        }
    }
    if (board[0][0]== current_player && board[1][1]== current_player  && board[2][2]== current_player) || (board[0][2]== current_player && board[1][1]== current_player  && board[2][0]== current_player ){
        return true;
    }
    return false;
}

fn check_draw(board:&BOARD)->bool{
    for row in board{
        for cell in row{
            if *cell== ' '{
                return false;
            }
        }
    }
    return true;
}

fn main(){
    println!("Welcome to TIC TAC TOE"); 
    play();
}