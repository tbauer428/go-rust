pub mod data;
pub mod game;

use data::Board;
use data::Value;
use game::initialize_window;
use std::*;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self)
    }
}

fn main() {
    let board = Board::new(13);

    // let board_rows = &board.0;

    // for x in 0..board_rows.len() {
    //     println!("Row Number: {}: {}", x + 1, format!("{:?}", board_rows[x]));
    // }

    initialize_window(&board)
}
