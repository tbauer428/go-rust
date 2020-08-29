pub mod game;

use game::initialize_window;
use std::*;

#[derive(Debug)]
struct Board(Vec<Vec<Tile>>);

#[derive(Debug)]
struct Tile(i32, Value);

#[derive(Debug)]
enum Value {
    White,
    Black,
    Empty,
}

impl Board {
    pub fn new(size: i32) -> Self {
        let mut row_number: i32 = 1;

        let mut board_vec: Vec<Vec<Tile>> = vec![];

        while row_number <= size {
            let mut row: Vec<Tile> = vec![];

            let mut tile_number: i32 = 1;
            while tile_number <= size {
                let tile = Tile(tile_number, Value::Empty);

                row.push(tile);

                tile_number += 1
            }

            board_vec.push(row);

            row_number += 1
        }

        Board(board_vec)
    }
}

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
    let board = Board::new(3);

    let board_rows = &board.0;

    for x in 0..board_rows.len() {
        println!("Row Number: {}: {}", x + 1, format!("{:?}", board_rows[x]));
    }

    initialize_window()
}
