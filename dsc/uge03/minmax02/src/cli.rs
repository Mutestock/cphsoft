use crate::tic_tac_toe::{Board, PlayerType};
use indoc::indoc;
use std::io::{self};
use std::{thread, time};
use crate::ai::{ai_turn};

enum StateType {
    Configuration,
    Game,
}

struct State {
    state_type: StateType,
}

pub fn handle_cli() {
    let mut kill_switch: bool = false;
    let mut state: State = State {
        state_type: StateType::Configuration,
    };

    let mut board: Board = Board::new();
    println!("Press any key to continue");
    while !kill_switch {
        take_turn(&mut state, &mut kill_switch, &mut board);
    }
}

fn take_turn(state: &mut State, kill_switch: &mut bool, board: &mut Board) {
    match state.state_type {
        StateType::Configuration => {
            println!("Config Time\n");

            // Setting player types
            let mut i = 1;
            while i < 3 {
                let player_type = spawn_prompt(String::from(format!(
                    "Is player {} a human or an AI? \n 1. Human \n 2. AI",
                    i
                )));

                match player_type.as_ref() {
                    "2" => {
                        (*board).players[i - 1] = PlayerType::AI;
                        println!("Player {} set to: AI \n", i-1);
                        i += 1;
                    }
                    "1" =>
                    // "Human" is default on board construction
                    {
                        i += 1;
                        println!("Player {} set to: Human\n", i-1)
                    }
                    _ => {
                        // Redos prompt on failure
                        println!(
                            "\n'{}' is not a valid input value. Try again \n",
                            player_type
                        );
                    }
                };
            } // current_player is a boolean value
            println!(
                "The starting player has randomly been set to {}\n",
                board.current_player as u8 + 1
            );
            println!("Player 1 symbol is: x\n");
            println!("Player 2 symbol is: o\n");
            thread::sleep(time::Duration::from_millis(1000));
            println!(indoc! {"
            Token id is determined like this:
            
            [1][2][3]
            [4][5][6]
            [7][8][9]
            
            "});
            println!("All set!");
            thread::sleep(time::Duration::from_millis(1000));
            state.state_type = StateType::Game;
        }
        StateType::Game => {
            println!("Player {}'s turn \n", board.current_player as u8 + 1);

            match board.current_player_is_human() {
                true => human_turn(board),
                false => ai_turn(board),
            };

            board.display();

            thread::sleep(time::Duration::from_millis(1000));

            if board.check_winner_found() || board.check_tie() {
                *kill_switch = true
            }
        }
    }
}

fn human_turn(board: &mut Board) {
    let mut answered: bool = false;
    while answered == false {
        let answer = spawn_prompt(String::from(
            "\nPlace your token. Valid input values = 1-9\n",
        ));
        match answer.parse::<usize>() {
            Ok(v) => {
                if v != 0 && v < 10 {
                    if board.slot_is_available(v - 1) {
                        board.change_slot_owner(v - 1);
                        answered = true;
                        board.current_player = !board.current_player;
                    } else {
                        println!("{} is occupied. Try again", answer);
                    }
                } else {
                    println!("{} is not a valid input value. Try again", answer);
                }
            }
            Err(_) => println!("{} is not a valid input value. Try again", answer),
        }
    }
}

// Takes question, returns answer
fn spawn_prompt(question: String) -> String {
    println!("{}", question);
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Line could not be read");
    buffer.trim().to_owned()
}
