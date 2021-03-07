use crate::game::{
    Participant, 
    Piece,
    determine_starting_player
};

/*
    The board options must represent this. 



            o
           / \
          x   x
         / \ / \
        o  oo  o

    What defines a "high" and a "low" value?
    Will there be a nested interest for the players to interrupt each other?
    Queue?


    Supposedly something like:


                    X - Vision(pos)        Y - Vision(pos)      X - Vision(Conclusion(x(pos) + y(pos)))
    [x] [ ] [ ]     [-] [1] [1]            [-] [1] [1]          [-] [2] [2]
    [ ] [o] [ ]     [1] [ ] [0]            [1] [ ] [1]          [2] [ ] [1]
    [ ] [ ] [ ]     [1] [0] [0]            [1] [1] [0]          [2] [1] [0]

    Whenever a player takes an action, certain branches must be disconnected from the minmax tree. (pruning) Vector tree.

    Each node must contain:
    Coordinates (x,y)
    Motivator
    Child nodes.
   
*/

//Granted this will always be 9 to conform with the definition of tic tac toe.
const DEPTH: u8 = 9;

fn assign_new_piece(mut node: Option<Node>) -> Piece {
    match node {
        Some(node) => {
            create_piece_with_new_coordinates(&node)
        }
        None => panic!("Node was none")
    }
}

fn create_piece_with_new_coordinates(mut node: &Node) -> Piece {
    match &node.child_nodes.last(){
        Some(node) => {
            let piece = &node.coordinate;
            if piece.x == 2 && piece.y == 2 {
                Piece {
                    x:0,
                    y:0,
                }
            }
            else if piece.y == 2{
                Piece {
                    x: piece.x+1,
                    y: 0,
                }
            }
            else {
                Piece{
                    x: piece.x,
                    y: piece.y+1,
                }
            }
        }
        None => {
            Piece {
                x:0,
                y:0,
            }
        }
    }
}

fn assign_child_nodes(mut node: Option<Node>) -> Vec<Node> {
    match node {
        Some(node) => node.child_nodes,
        None => Vec::new(),
    }
}

#[derive(Clone)]
pub struct Node {
    coordinate: Piece,
    owner: Option<bool>,
    child_nodes: Vec<Node>,
}

impl Node {
    fn new(previous_node: Option<Node>, piece: Piece) -> Self{
        Self {
            coordinate: piece,
            owner: None,
            child_nodes: assign_child_nodes(previous_node),
        }
    }

    // fn check_pieces(&self, incoming_piece: &Piece) -> bool {
    //     self.coordinates.iter().any(|v| v.x == incoming_piece.x && v.y == incoming_piece.y)
    // }
}

pub struct Board {
    board_squares: Vec<Node>
}

fn swap_players(participant_id: bool) -> bool{
    if participant_id{
        false
    }
    else {
        true
    }
}

fn populate_available_squares() -> Vec<Piece>{
    let mut available_squares: Vec<Piece> = Vec::new();

    (0..3).for_each(|x| {
        (0..3).for_each(|y|{
            available_squares.push(Piece{x,y})
        })
    });
    
    available_squares
}



impl Board {
    pub fn new() -> Self {
        let mut new_board =  Self {
            board_squares: Vec::new(),
        };
        new_board.populate_options(determine_starting_player(), Node::new(None, Piece{x:0,y:0}), populate_available_squares());
        new_board
    }

    fn populate_options(&mut self, participant_id: bool, mut node: Node, available_squares: Vec<Piece>) {
        // Need a disqualifier. Think x*y = i. Vector with disqualified pieces? Depth unnecessary? 
        // Need way to separate coordinates. 3*3. Depth of 9. 
        if !&available_squares.is_empty() {
            //let x = depth % 3;
            //let mut y = 0;
            //if depth !=0 {
            //    y = (depth as f64/3.0 ).floor() as u8;
            //}

            //let spot_is_occupied = available_squares.iter().any(|v| v.x == x && v.y == y);

            &available_squares.iter().for_each(|v| {
                if !(v.x == node.coordinate.x && v.y == node.coordinate.y) {
                    let mut available_squares_cloned = available_squares.clone();
                    println!("{} {}", v.x, v.y);
                    available_squares_cloned.retain( |element| element != v);
                    node.child_nodes.push(node.clone());

                    self.populate_options(participant_id, Node::new(Some(node.clone()), create_piece_with_new_coordinates(&node)), available_squares_cloned);
                }
            });

            swap_players(participant_id);
        }
    }
}
