#[derive(Debug)]
pub struct Board(pub Vec<Vec<Tile>>);

#[derive(Debug)]
pub struct Tile(i32, Value);

#[derive(Debug)]
pub enum Value {
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
