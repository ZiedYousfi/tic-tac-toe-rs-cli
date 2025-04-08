use std::fmt::Error;

fn main() {
    println!("Hello, world!");

    let mut playing: bool = false;
    let mut player_1_turn: bool = true;
    let mut player_1_win: bool = false;

    // 0 = not played, 1 = player 1, 2 = player 2
    let mut board_vec: [u8; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut player_selector: u8 = 0;

    playing = true;

    while playing {
        match play_action(&mut board_vec, player_selector) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error : {}", e);
                continue;
            }
        }

        match checks_win(&board_vec) {
            Ok(b) => {
                if b {
                    player_1_win = player_1_turn;
                    playing = false;
                }
            }
            Err(e) => {
                eprintln!("Error : {}", e);
                continue;
            }
        }

        player_1_turn = !player_1_turn;
    }

    match end_game(player_1_win) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error : {}", e);
        }
    }
}

fn play_action(board: &mut [u8; 9], selector: u8) -> Result<(), Error> {

    println!("{} | {} | {}", board[0],board[1],board[2]);
    println!("------------");
    println!("{} | {} | {}", board[3],board[4],board[5]);
    println!("------------");
    println!("{} | {} | {}", board[6],board[7],board[8]);


    Ok(())
}

fn checks_win(board: &[u8; 9]) -> Result<bool, Error> {
    Ok(false)
}

fn end_game(player1_win: bool) -> Result<(), Error> {
    Ok(())
}
