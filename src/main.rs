use anyhow::{Error, Result};
use crossterm::event::poll;
use std::thread;
use std::time::Duration;

use crossterm::event::{Event, KeyCode, read};

#[derive(Debug)]
enum InputType {
    KeyUp,
    KeyDown,
    KeyLeft,
    KeyRight,
    Enter,
}

#[derive(Debug)]
enum WrapperErrorType {
    UselessCall,
    AnyhowError(anyhow::Error),
}

#[tokio::main]
async fn main() {
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
        match play_action(&mut board_vec, &mut player_selector, &player_1_turn).await {
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

        //println!("frame {}", i);

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

async fn play_action(board: &mut [u8; 9], selector: &mut u8, player1_turn: &bool) -> Result<(), Error> {
    let to_print: [&str; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8].map(|i| {
        if *selector as usize == i {
            match board[i] {
                0 => "0*",
                1 => "1*",
                2 => "2*",
                _ => "x*",
            }
        } else {
            match board[i] {
                0 => "  ",
                1 => "1 ",
                2 => "2 ",
                _ => "x ",
            }
        }
    });

    println!("{} | {} | {}", to_print[0], to_print[1], to_print[2]);
    println!("------------");
    println!("{} | {} | {}", to_print[3], to_print[4], to_print[5]);
    println!("------------");
    println!("{} | {} | {}", to_print[6], to_print[7], to_print[8]);

    let mut next_selector_value = *selector;

    match user_input().await {
        Ok(input_type) => match input_type {
            InputType::KeyUp => {
                next_selector_value = (*selector as i64 - 3) as u8;
                match selector_wrapper(*selector as i64 - 3) {
                    Ok(v) => *selector = v,
                    Err(e) => match e {
                        WrapperErrorType::UselessCall => *selector = next_selector_value,
                        WrapperErrorType::AnyhowError(e) => {
                            return Err(anyhow::format_err!("Actual error for wrapper: {}", e));
                        }
                    },
                }
            }
            InputType::KeyDown => {
                next_selector_value = (*selector as i64 + 3) as u8;
                match selector_wrapper(*selector as i64 + 3) {
                    Ok(v) => *selector = v,
                    Err(e) => match e {
                        WrapperErrorType::UselessCall => *selector = next_selector_value,
                        WrapperErrorType::AnyhowError(e) => {
                            return Err(anyhow::format_err!("Actual error for wrapper: {}", e));
                        }
                    },
                }
            }
            InputType::KeyLeft => {
                next_selector_value = (*selector as i64 - 1) as u8;
                match selector_wrapper(*selector as i64 - 1) {
                    Ok(v) => *selector = v,
                    Err(e) => match e {
                        WrapperErrorType::UselessCall => *selector = next_selector_value,
                        WrapperErrorType::AnyhowError(e) => {
                            return Err(anyhow::format_err!("Actual error for wrapper: {}", e));
                        }
                    },
                }
            }
            InputType::KeyRight => {
                next_selector_value = (*selector as i64 + 1) as u8;
                match selector_wrapper(*selector as i64 + 1) {
                    Ok(v) => *selector = v,
                    Err(e) => match e {
                        WrapperErrorType::UselessCall => *selector = next_selector_value,
                        WrapperErrorType::AnyhowError(e) => {
                            return Err(anyhow::format_err!("Actual error for wrapper: {}", e));
                        }
                    },
                }
            },
            InputType::Enter => {
                match player1_turn {
                    true => board[*selector as usize] = 1,
                    false => board[*selector as usize] = 2,
                }
            }
        },
        Err(e) => eprintln!("Input error: {}", e),
    }

    Ok(())
}

fn checks_win(board: &[u8; 9]) -> Result<bool, Error> {
    // diagonal

    Ok(false)
}

fn end_game(player1_win: bool) -> Result<(), Error> {
    Ok(())
}

fn selector_wrapper(i: i64) -> Result<u8, WrapperErrorType> {
    if (0..9).contains(&i) {
        return Err(WrapperErrorType::UselessCall);
    }

    let mut tmp: i64 = i;
    while !(0..9).contains(&tmp) {
        tmp = match simple_wrapper(tmp) {
            Ok(val) => val,
            Err(e) => {
                return Err(WrapperErrorType::AnyhowError(anyhow::format_err!(
                    "Wrapper error: {:?}",
                    e
                )));
            }
        };
    }

    println!("result: {}", tmp);
    Ok(tmp as u8)
}

fn simple_wrapper(i: i64) -> Result<i64, WrapperErrorType> {
    let mut result: i64 = 0;

    let mut tmp: i64 = 0;

    if (0..9).contains(&i) {
        return Err(WrapperErrorType::UselessCall);
    }

    let diff: i64 = match &i {
        n if n > &8 => n - (9 + 1),
        n if n < &0 => 0 - n,
        _ => {
            return Err(WrapperErrorType::AnyhowError(anyhow::format_err!(
                "how did we get here? (diff)"
            )));
        }
    };

    tmp = match &i {
        n if n > &8 => {
            if (0..9).contains(&(diff - (9 + 1))) {
                -(diff - 1)
            } else {
                diff
            }
        }
        n if n < &0 => 9 - diff,
        _ => i,
    };

    if !(0..9).contains(&tmp) {
        println!("tmp: {}", tmp);
        println!("not yet");
        tmp = selector_wrapper(tmp).expect("msg") as i64;
    }

    if !(0..9).contains(&tmp) {
        return Err(WrapperErrorType::AnyhowError(anyhow::format_err!(
            "ok im dumb"
        )));
    }

    result = tmp;

    println!("result: {}", result);

    Ok(result)
}

async fn user_input() -> Result<InputType, Error> {
    if poll(Duration::from_millis(50))? {
        match read()? {
            Event::Key(event) => {
                println!("{:?}", event);
                match event.code {
                    KeyCode::Up => return Ok(InputType::KeyUp),
                    KeyCode::Down => return Ok(InputType::KeyDown),
                    KeyCode::Left => return Ok(InputType::KeyLeft),
                    KeyCode::Right => return Ok(InputType::KeyRight),
                    KeyCode::Enter => return Ok(InputType::Enter),
                    _ => return Err(anyhow::format_err!("unsupported key")),
                }
            }
            _ => return Err(anyhow::format_err!("unknown input")),
        }
    }
    Err(anyhow::format_err!("no input received"))
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
            //assert!(result.unwrap_err().to_string().contains("useless call"));
        }
    }

    #[test]
    fn test_selector_wrapper_handles_out_of_bounds_positive() {
        // Test simple out of bounds positive cases
        assert_eq!(selector_wrapper(10).unwrap(), 0); // 10 wraps to 0
        assert_eq!(selector_wrapper(11).unwrap(), 1); // 11 wraps to 1
        assert_eq!(selector_wrapper(17).unwrap(), 7); // 17 wraps to 7
        assert_eq!(selector_wrapper(18).unwrap(), 8); // 18 wraps to 8
    }

    #[test]
    fn test_selector_wrapper_handles_out_of_bounds_negative() {
        // Test simple out of bounds negative cases
        // assert_eq!(selector_wrapper(-1).unwrap(), 8); // -1 wraps to 8
        assert_eq!(selector_wrapper(-2).unwrap(), 7); // -2 wraps to 7
        // assert_eq!(selector_wrapper(-8).unwrap(), 1); // -8 wraps to 1
        // assert_eq!(selector_wrapper(-9).unwrap(), 0); // -9 wraps to 0
    }

    #[test]
    fn test_selector_wrapper_handles_far_out_of_bounds() {
        // Test values that require recursive wrapping
        //assert_eq!(selector_wrapper(19).unwrap(), 1); // 19 -> (19-9) = 10 -> (10-9) = 1
        assert_eq!(selector_wrapper(28).unwrap(), 1); // 28 -> (28-9) = 19 -> (19-9) = 10 -> (10-9) = 1
        assert_eq!(selector_wrapper(-10).unwrap(), 8); // -10 -> (9-10) = 8
        assert_eq!(selector_wrapper(-19).unwrap(), 8); // -19 -> recursively wraps to 8
    }

    #[test]
    fn test_selector_wrapper_handles_extreme_values() {
        // Test with larger values and verify actual results
        let result_positive = selector_wrapper(100).unwrap();
        assert!(
            (0..9).contains(&result_positive),
            "Result {result_positive} should be in range 0-8"
        );

        let result_negative = selector_wrapper(-100).unwrap();
        assert!(
            (0..9).contains(&result_negative),
            "Result {result_negative} should be in range 0-8"
        );

        // Test specific extreme values with known results
        assert_eq!(selector_wrapper(900).unwrap(), 0); // 99 wraps to a value in 0-8
        assert_eq!(selector_wrapper(-900).unwrap(), 0); // -99 wraps to a value in 0-8
    }
}
