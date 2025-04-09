use std::fmt::Error;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    const REFRESH_RATE: f64 = 1.0 / 12.0;

    let mut playing: bool = false;
    let mut player_1_turn: bool = true;
    let mut player_1_win: bool = false;

    // 0 = not played, 1 = player 1, 2 = player 2
    let mut board_vec: [u8; 9] = [0, 0, 0, 0, 1, 0, 0, 0, 0];
    let mut player_selector: u8 = 0;

    playing = true;
    clearscreen::clear().unwrap();

    let mut i: u64 = 0;

    while playing {
        i += 1;
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

        println!("frame {}",i);

        thread::sleep(Duration::from_secs_f64(REFRESH_RATE));
        clearscreen::clear().unwrap();

    }

    match end_game(player_1_win) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error : {}", e);
        }
    }
}

fn play_action(board: &mut [u8; 9], selector: u8) -> Result<(), Error> {

    let to_print: [&str; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8].map(|i| {
        if selector as usize == i {
            match board[i] {
                0 => "0*",
                1 => "1*",
                2 => "2*",
                _ => "x*",
            }
        } else {
            match board[i] {
                0 => " ",
                1 => "1",
                2 => "2",
                _ => "x",
            }
        }
    });

    println!("{} | {} | {}", to_print[0], to_print[1], to_print[2]);
    println!("------------");
    println!("{} | {} | {}", to_print[3], to_print[4], to_print[5]);
    println!("------------");
    println!("{} | {} | {}", to_print[6], to_print[7], to_print[8]);

    Ok(())
}

fn checks_win(board: &[u8; 9]) -> Result<bool, Error> {
    Ok(false)
}

fn end_game(player1_win: bool) -> Result<(), Error> {
    Ok(())
}
