use rand::random;
use crate::ai::{Node, Board};

const TURN_TIME: u16 = 1000;

#[derive(Clone, PartialEq)]
pub struct Piece {
    pub x:u8,
    pub y:u8,
}


// Considering how there are only two players, booleans as identifiers are enough.
#[derive(Clone)]
pub struct Participant {
    id: bool,
    pieces: Vec<Piece>,
}

impl Participant {
    fn new(id: bool) -> Self {
        Self {
            id: id,
            pieces: Vec::new(),
        }
    }
}

pub struct Game {
    turn_count: u8,
    current_participant_id: bool,
    participants: [Participant; 2],
}

impl Game {
    fn new() -> Self {
        let participants: [Participant;2] = [Participant::new(true), Participant::new(false)];
        Self {
            turn_count:0,
            current_participant_id: determine_starting_player(),
            participants: participants
        }
    }
    fn rotate_turn(&mut self){
        match self.current_participant_id {
            true => self.current_participant_id = false,
            false => self.current_participant_id = true
        }
    }
}

pub fn display_board(node: Node){
    for i in 0..3{
        println!("[ ] [ ] [ ]\n")
    }
}

pub fn resolve_everything(){
    let board = Board::new();
}



pub fn determine_starting_player() -> bool {
    random()
}




