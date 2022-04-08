// Board
// Holds an array of Squares that can be accessed with 2D x,y coordinates.

use crate::{Coord, Piece};

#[derive(Debug, Clone)]
pub struct Board {    
    pub columns: usize,
    pub rows: usize,
    pub piece_ids: Vec<Option<usize>>,
}

impl Board {
    pub fn new(columns: usize, rows: usize) -> Self {
        Self {
            columns,
            rows,
            piece_ids: Vec::<Option<usize>>::with_capacity(columns * rows),
        }
    }

    /// Set up the board for use.
    pub fn prepare(&mut self) {
        for _ in 0..self.columns * self.rows {
            self.piece_ids.push(None);
        }
    }

    #[allow(dead_code)]
    /// Prints the board.
    pub fn display(&mut self, pieces: &[Piece]) {
        let x_labels = "abcdefghijklmnopqrstuvwxyz";
        let mut horiz_separator = String::from("  +");
        let mut x_label_row = String::from("    ");
        for x in 0..self.columns {
            horiz_separator.push_str("---+");
            x_label_row.push_str(&x_labels[x..x+1]);
            x_label_row.push_str("   ");
        }
        println!("{}", horiz_separator);

        for y in (0..self.rows).rev() {
            print!("{} |", y + 1); // to match chess coordinates that start at 1, not 0
            for x in 0..self.columns {
                print!(" ");
                if let Some(id) = self.piece_id_at(&Coord {x, y}) {
                    print!("{}", pieces[id].string_rep());
                }
                else {
                    print!(" ");
                }
                print!(" |");
            }
            println!();
            println!("{}", horiz_separator);
        }
        println!("{}", x_label_row);
    }

    /// Returns vector of coords that have no pieces.
    pub fn empty_coords(&self) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        for (index, id) in self.piece_ids.iter().enumerate() {
            if id.is_none() {
                coords.push(Coord::from_index(index, self.columns));
            }
        }
        coords
    }

    /// Returns the piece_id at the given coord.
    pub fn piece_id_at(&mut self, coord: &Coord) -> Option<usize> {
        self.piece_ids[coord.to_index(self.columns)]
    }

    /// Adds the piece_id at given coord. Panics if coord is not empty.
    pub fn add_piece_id(&mut self, id: usize, coord: &Coord) {
        let index = coord.to_index(self.columns);
        if self.piece_ids[index].is_some() {
            panic!("Board.add_piece: coord is not empty!");
        }
        self.piece_ids[index] = Some(id);
    }

    /// Sets the coord to EMPTY_ID and returns the previous id.
    pub fn remove_piece_id(&mut self, coord: &Coord) -> Option<usize> {
        let index = coord.to_index(self.columns);
        // 'take' removes the option value and leaves None in its place.
        self.piece_ids[index].take()
    }

    // Is this used?
    pub fn is_player_piece_at(&mut self, coord: &Coord, player: usize, pieces: &[Piece]) -> bool {
        let id = self.piece_id_at(coord);
        id.is_some() && pieces[id.unwrap()].player == player
    }
}