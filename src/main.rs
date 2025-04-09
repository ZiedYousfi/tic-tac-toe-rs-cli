use anyhow::{Error, Result};
use std::thread;
use std::time::Duration;
// use std::result::Result;

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

        println!("frame {}", i);

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

fn selector_wrapper(i: i64) -> Result<u8, Error> {
    let mut result: i64 = 0;
    let mut diff: i64 = 0;

    if (0..9).contains(&i) {
        return Err(anyhow::format_err!(
            "absolutely useless call of this mighty wrapper"
        ));
    }

    diff = match &i {
        n if n > &8 => n - (9 + 1),
        n if n < &0 => 0 - n,
        _ => return Err(anyhow::format_err!("how did we get here? (diff)")),
    };

    result = match &i {
        n if n > &8 => diff,
        n if n < &0 => 9 - &diff,
        _ => i,
    };

    if !(0..9).contains(&result) {
        result = selector_wrapper(result).expect("msg") as i64;
    }

    if result < 0 {
        return Err(anyhow::format_err!("im dumb"));
    }

    Ok(result as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selector_wrapper_rejects_valid_inputs() {
        // Function should reject inputs that are already in range
        for i in 0..9 {
            let result = selector_wrapper(i);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("useless call"));
        }
    }

    #[test]
    fn test_selector_wrapper_handles_out_of_bounds_positive() {
        // Test simple out of bounds positive cases
        assert_eq!(selector_wrapper(10).unwrap(), 0);  // 10 wraps to 0
        assert_eq!(selector_wrapper(11).unwrap(), 1);  // 11 wraps to 1
        assert_eq!(selector_wrapper(17).unwrap(), 7);  // 17 wraps to 7
        assert_eq!(selector_wrapper(18).unwrap(), 8);  // 18 wraps to 8
    }

    #[test]
    fn test_selector_wrapper_handles_out_of_bounds_negative() {
        // Test simple out of bounds negative cases
        assert_eq!(selector_wrapper(-1).unwrap(), 8);  // -1 wraps to 8
        assert_eq!(selector_wrapper(-2).unwrap(), 7);  // -2 wraps to 7
        assert_eq!(selector_wrapper(-8).unwrap(), 1);  // -8 wraps to 1
        assert_eq!(selector_wrapper(-9).unwrap(), 0);  // -9 wraps to 0
    }

    #[test]
    fn test_selector_wrapper_handles_far_out_of_bounds() {
        // Test values far outside the range that require multiple wraps
        assert_eq!(selector_wrapper(19).unwrap(), 0);   // Double wrap: 19 -> 9 -> 0
        assert_eq!(selector_wrapper(28).unwrap(), 0);   // Triple wrap
        assert_eq!(selector_wrapper(-10).unwrap(), 8);  // Double wrap: -10 -> -1 -> 8
        assert_eq!(selector_wrapper(-19).unwrap(), 8);  // Triple wrap
    }

    #[test]
    fn test_selector_wrapper_handles_extreme_values() {
        // Test with larger values
        assert!(selector_wrapper(100).is_ok());  // Should handle without panicking
        assert!(selector_wrapper(-100).is_ok()); // Should handle without panicking
    }
}
