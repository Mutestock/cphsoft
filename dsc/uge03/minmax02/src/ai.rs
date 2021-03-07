/*

    position = tiles?
*/

use crate::tic_tac_toe::{check_winner_found, Board, SlotOccupied};
use rand::Rng;

pub fn ai_turn(board: &mut Board) {
    // Place token in random place if empty board (starting player)
    if !board.slots.iter().any(|v| v != &SlotOccupied::Empty) {
        board.change_slot_owner(rand::thread_rng().gen_range(0..9));
    } else {
        minimax(
            board,
            &mut board.slots.clone(),
            0.0,
            0.0,
            9,
            board.current_player,
            &mut None,
        );
    }

    board.tile_changed_this_turn = false;
    board.current_player = !board.current_player;
}

fn minimax(
    board: &mut Board,
    intermediate_board: &mut [SlotOccupied; 9],
    mut alpha: f64,
    mut beta: f64,
    depth: usize,
    maximizing_player: bool,
    first_move: &mut Option<usize>,
) -> f64 {
    // First dig to the buttom (winner found)
    // Need to check opponent as well.


    if depth == 0 || check_winner_found(&intermediate_board) {
        // Static evaluation of position?
        board.change_slot_owner(match first_move {
            Some(x) => x.clone(),
            None => panic!("How could this happen to meeeeeee"),
        });
        return 0.0;
    }

    // Player 1
    if maximizing_player {
        let mut max_eval = f64::NEG_INFINITY;
        for (i, slot) in intermediate_board.clone().iter().enumerate() {
            let eval = create_eval(
                slot,
                intermediate_board,
                i,
                board,
                alpha,
                beta,
                depth,
                maximizing_player,
                first_move,
            );

            max_eval = match max_eval > eval {
                true => max_eval,
                false => eval,
            };
            alpha = match alpha > eval {
                true => alpha,
                false => eval,
            };
            if beta <= alpha {
                break;
            }
        }
        return max_eval;
    } else {
        let mut min_eval = f64::INFINITY;
        for (i, slot) in intermediate_board.clone().iter().enumerate() {
            let eval = create_eval(
                slot,
                intermediate_board,
                i,
                board,
                alpha,
                beta,
                depth,
                maximizing_player,
                first_move,
            );
            min_eval = match min_eval < eval {
                true => min_eval,
                false => eval,
            };
            beta = match beta < eval {
                true => beta,
                false => eval,
            };
            if beta <= alpha {
                break;
            }
        }
        return min_eval;
    }
}

fn create_eval(
    slot: &SlotOccupied,
    intermediate_board: &mut [SlotOccupied; 9],
    mut i: usize,
    board: &mut Board,
    alpha: f64,
    beta: f64,
    depth: usize,
    maximizing_player: bool,
    first_move: &mut Option<usize>,
) -> f64 {
    if slot != &SlotOccupied::Empty {
        intermediate_board[i] = *slot;

        let thing = match first_move {
            Some(x) => x,
            None => &mut i,
        };
        minimax(
            board,
            intermediate_board,
            alpha,
            beta,
            depth - 1,
            !maximizing_player,
            &mut Some(*thing),
        )
    } else {
        minimax(
            board,
            intermediate_board,
            alpha,
            beta,
            depth - 1,
            !maximizing_player,
            &mut None,
        )
    }
}
