use rand;




// Contains basic information about the current state of the game.
#[derive(Clone)]
pub struct Board {
    pub players: [PlayerType; 2],
    pub slots: [SlotOccupied; 9],
    // Where player 1 = false, player 2 = true
    pub current_player: bool,
    pub tile_changed_this_turn: bool,
}

impl Board {
    pub fn new() -> Self {
        Self {
            players: [PlayerType::Human, PlayerType::Human],
            slots: [
                SlotOccupied::Empty,
                SlotOccupied::Empty,
                SlotOccupied::Empty,
                SlotOccupied::Empty,
                SlotOccupied::Empty,
                SlotOccupied::Empty,
                SlotOccupied::Empty,
                SlotOccupied::Empty,
                SlotOccupied::Empty,
            ],
            current_player: rand::random(),
            tile_changed_this_turn: false,
        }
    }

    pub fn slot_is_available(&self, slot_id: usize) -> bool {
        self.slots[slot_id] == SlotOccupied::Empty
    }

    pub fn change_slot_owner(&mut self, slot_id: usize) {
        self.slots[slot_id] = match self.current_player {
            false => SlotOccupied::Cross,
            true => SlotOccupied::Circle,
        };
    }

    pub fn current_player_is_human(&mut self) -> bool {
        self.players[(self.current_player as usize)] == PlayerType::Human
    }

    pub fn check_tie(&self) -> bool {
        let res = !self.slots.iter().any(|v| match v {
            SlotOccupied::Empty => false,
            SlotOccupied::Cross => true,
            SlotOccupied::Circle => true,
        });
        if res {
            println!("\nDraw")
        }
        res
    }
    // Board dependant win check.
    pub fn check_winner_found(&self) -> bool {
        let slots_arr = &self.slots;

        // Check diagonal
        if self.check_winner_diagonal() {
            return true;
        }

        for i in 0..3 {
            // Check horizontal
            if slots_arr[i * 3] != SlotOccupied::Empty
                && slots_arr[i * 3] == slots_arr[i * 3 + 1]
                && slots_arr[i * 3 + 1] == slots_arr[i * 3 + 2]
            {
                println!(
                    "\nPlayer {} wins by horizontal",
                    self.current_player as u8 + 1
                );
                return true;
            }
            // Check vertical
            else if slots_arr[i] != SlotOccupied::Empty
                && slots_arr[i] == slots_arr[i + 3]
                && slots_arr[i + 3] == slots_arr[i + 6]
            {
                println!(
                    "\nPlayer {} wins by vertical",
                    self.current_player as u8 + 1
                );
                return true;
            }
        }

        false
    }

    fn check_winner_diagonal(&self) -> bool {
        let slots_arr = &self.slots;
        if slots_arr[0] != SlotOccupied::Empty
            && slots_arr[0] == slots_arr[4]
            && slots_arr[0] == slots_arr[8]
        {
            println!("\nPlayer {} wins by diagonal", self.current_player as u8);
            true
        } else if slots_arr[2] != SlotOccupied::Empty
            && slots_arr[2] == slots_arr[4]
            && slots_arr[4] == slots_arr[6]
        {
            println!("\nPlayer {} wins by diagonal", self.current_player as u8);
            true
        } else {
            false
        }
    }

    pub fn display(&self) {
        for i in 0..9 {
            if i % 3 == 0 {
                println!();
            }
            print!("{}", visualize_token(&self.slots[i]));
        }
        println!();
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum PlayerType {
    Human,
    AI,
}

#[derive(PartialEq, Copy, Clone)]
pub enum SlotOccupied {
    Cross,
    Circle,
    Empty,
}

fn visualize_token(slot_occupied: &SlotOccupied) -> String {
    let res = match slot_occupied {
        SlotOccupied::Cross => "[x]".to_owned(),
        SlotOccupied::Circle => "[o]".to_owned(),
        SlotOccupied::Empty => "[ ]".to_owned(),
    };
    res
}

// Board independant win check
pub fn check_winner_found(slots_arr: &[SlotOccupied; 9]) -> bool {
    // Check diagonal
    if check_winner_diagonal(slots_arr) {
        return true;
    }

    for i in 0..3 {
        // Check horizontal
        if slots_arr[i * 3] != SlotOccupied::Empty
            && slots_arr[i * 3] == slots_arr[i * 3 + 1]
            && slots_arr[i * 3 + 1] == slots_arr[i * 3 + 2]
        {
            return true;
        }
        // Check vertical
        else if slots_arr[i] != SlotOccupied::Empty
            && slots_arr[i] == slots_arr[i + 1]
            && slots_arr[i + 1] == slots_arr[i + 2]
        {
            return true;
        }
    }

    false
}

fn check_winner_diagonal(slots_arr: &[SlotOccupied; 9]) -> bool {
    if slots_arr[0] != SlotOccupied::Empty
        && slots_arr[0] == slots_arr[4]
        && slots_arr[0] == slots_arr[8]
    {
        true
    } else if slots_arr[2] != SlotOccupied::Empty
        && slots_arr[2] == slots_arr[4]
        && slots_arr[4] == slots_arr[6]
    {
        true
    } else {
        false
    }
}
