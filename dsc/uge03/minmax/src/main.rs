pub mod game;
pub mod ai;

use game::{ display_board };

/*

Victory condition - three in a row

Board coordinates X,Y tuple or struct?

Options - 9,8,7,6,5,4,3,2,1

List with coordinates. Detach with pop.

Abstract coordiante representation
(0,2)(1,2)(2,2)
(0,1)(1,1)(2,1)
(0,0)(1,0)(2,0)

Console display example. Change of text color to display final victory formation?
[x] [o] [x]
[o] [x] [ ]
[x] [ ] [o]


minmax - black (min vals) vs white (max vals)

Victory case examples:

# Horizontal - Constant y axis
(0,2)(1,2)(2,2)

# Diagonal
(0.2)(1.1)(2.0)

# Vertical - Constant x axis
(0.2)(0.1)(0.0)

board square definition? Square(Owner, (x,y))

*/





fn main() {
    game::resolve_everything();
}
