// Game
// Primitives are used to keep Game instances on the stack and hashable.

//use slotmap::{DefaultKey, SlotMap};

use crate::Action;
use crate::action::ActionKind::*;
use crate::GameState::*;
use crate::Piece;
use crate::piece::PieceKind;
use crate::piece::PieceKind::*;

pub const COLS: usize = 3;
pub const ROWS: usize = 4;
const GRID_COUNT: usize = 12;
const PIECES_PER_PLAYER: usize = 4;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coord(pub usize, pub usize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GameState {
    Ongoing,
    Draw,
    WinPlayer0,
    WinPlayer1,
}

pub const NONE: usize = usize::MAX;
const STARTING_POSITION: &str = "BKR-P--p-rkb";

#[derive(Clone, Copy, Hash)]
pub struct Game {
    // This owns all the pieces. grid and reserves just hold the indices.
    pub pieces: [Piece; PIECES_PER_PLAYER * 2],
    pub grid: [usize; GRID_COUNT],
    reserves: [[usize; PIECES_PER_PLAYER]; 2],

    pub current_player: usize,
    pub state: GameState,
}

impl Game {
    pub fn new() -> Self {
        // This will be replaced by the proper pieces during 'prepare'.
        let default_piece = Piece::new(0, Pawn, 0);
        Self {
            pieces: [default_piece; PIECES_PER_PLAYER * 2],
            grid: [NONE; GRID_COUNT],
            reserves: [[NONE; PIECES_PER_PLAYER]; 2],
            current_player: 0,
            state: Ongoing,
        }
    }

    /// Convert coord to an array index.
    pub fn coord_to_index(coord: &Coord) -> usize {
        coord.1 * COLS + coord.0
    }

    /// Convert array index to coord.
    pub fn index_to_coord(index: usize) -> Coord {
        Coord(index % COLS, index / COLS)
    }

    // fn piece_id_for(&self, player: usize, kind: PieceKind) -> usize {
    //     let option_item = self.pieces.iter().enumerate().find(
    //         |(i, p)| p.kind == kind && p.player == player);
    //     option_item.unwrap().0
    // }

    pub fn piece_for(kind: char, id: usize) -> Piece {
        match kind {
            'K' => Piece::new(id, King, 0),
            'R' => Piece::new(id, Rook, 0),
            'B' => Piece::new(id, Bishop, 0),
            'P' => Piece::new(id, Pawn, 0),
            'k' => Piece::new(id, King, 1),
            'r' => Piece::new(id, Rook, 1),
            'b' => Piece::new(id, Bishop, 1),
            'p' => Piece::new(id, Pawn, 1),
            _ => panic!("piece kind not recognized"),
        }
    }

    /// Get the board ready for a new game.
    pub fn prepare(&mut self) {
        self.setup_position(STARTING_POSITION);
    }

    fn setup_position(&mut self, position: &str) {
        let mut piece_id = 0;
        let mut index = 0;
        for kind in position.chars() {
            if kind != '-' {
                // Add to pieces array.
                let piece = Game::piece_for(kind, piece_id);
                self.pieces[piece_id] = piece;
                // Set piece on grid.
                let coord = Game::index_to_coord(index);
                self.set_piece(piece_id, &coord);
                piece_id += 1;
            }
            index += 1;
        }
    }

    /// Advance to the next player.
    pub fn next_player(&mut self) {
        self.current_player = 1 - self.current_player;
    }

    #[allow(dead_code)]
    // Return to the previous player.
    pub fn previous_player(&mut self) {
        self.current_player = 1 - self.current_player;
    }

    #[allow(dead_code)]
    /// Returns a reference to the piece at the given coord.
    pub fn get_piece(&mut self, coord: &Coord) -> usize {
        self.grid[Game::coord_to_index(coord)]
    }

    /// Sets the piece at the given coord.
    pub fn set_piece(&mut self, piece_id: usize, coord: &Coord) {
        self.pieces[piece_id].coord = Some(*coord);
        self.grid[Game::coord_to_index(coord)] = piece_id;
    }

    /// Removes and returns the piece at the given coord.
    pub fn remove_piece(&mut self, coord: &Coord) -> usize {
        let i = Game::coord_to_index(coord);
        let piece_id = self.grid[i];
        self.pieces[piece_id].coord = None;
        self.grid[i] = NONE;
        piece_id
    }

    pub fn remove_reserve_piece(&mut self, piece_id: usize, player: usize) {
        let piece = NONE;
        for (index, id) in self.reserves[player].iter().enumerate() {
            if *id == piece_id {
                self.reserves[player][index] = NONE;
                break;
            }
        }
    }

    pub fn add_reserve_piece(&mut self, piece_id: usize, player: usize) {
        for (index, id) in self.reserves[player].iter().enumerate() {
            if *id == NONE {
                self.reserves[player][index] = piece_id;
                break;
            }
        }
    }

    /// Controller needs to know for piece selection logic.
    pub fn coord_has_current_player_piece(&mut self, coord: &Coord) -> bool {
        let id = self.get_piece(coord);
        self.pieces[id].player == self.current_player
    }

    // **** empty_indices instead, and avoid Coord2 conversion? ***
    /// Returns vector of coords that have no pieces.
    pub fn empty_coords(&self) -> Vec<Coord> {
        let mut empties = Vec::new();
        for i in self.grid {
            if i == NONE {
                empties.push(Game::index_to_coord(i));
            }
        }
        empties
    }

    /// Determines if the given player has won.
    fn is_win(&mut self, player: usize) -> bool {
        // If opponent king is capture: true.

        // If player king on back row and not in "check": true.

        // If 3-move repeat: true

        false
    }

    /// Updates and returns the 'status' field.
    pub fn update_state(&mut self) -> &GameState {
        self.state = GameState::Ongoing; // assume
        // Check for wins before checking for out-of-moves.
        if self.is_win(0) {
            self.state = GameState::WinPlayer0;
        }
        else if self.is_win(1) {
            self.state = GameState::WinPlayer1;
        }
        else if self.actions_available().is_empty() {
            self.state = GameState::Draw;
        }
        &self.state
    }

    pub fn actions_available(&mut self) -> Vec<Action> {
        let mut actions = Vec::new();
        // Get player's pieces. Optimization opportunity here.
        let mut player_pieces = Vec::new();
        for id in self.grid {
            if id == NONE { continue; }
            let piece = self.pieces[id];
            if piece.player == self.current_player {
                player_pieces.push(piece);
            }
        }
        for piece in &player_pieces {
            let vectors = piece.move_vectors();
            let pc_coord = piece.coord.unwrap();
            for (x, y) in vectors {
                let move_x = pc_coord.0 as i8 + x;
                let move_y = pc_coord.1 as i8 + y;

                // Is this move out of bounds?
                if move_x < 0 || move_x as usize  >= COLS || move_y < 0 || move_y as usize >= ROWS {
                    continue;
                }
                let to_coord = Coord(move_x as usize, move_y as usize);
                let action = Action::new(MoveNoCapture, piece.id, Some(pc_coord), to_coord );
                actions.push(action);
            }
        }
        actions
    }

    pub fn perform_action(&mut self, action: &Action, advance_player: bool) {
        match action.kind {
            MoveNoCapture => {
                self.remove_piece(&action.from.unwrap());
                self.set_piece(action.piece_id, &action.to);
            }
            MoveWithCapture => {
            }
            FromReserve => {
                let piece_id = self.remove_reserve_piece(action.piece_id, self.current_player);
                self.set_piece(action.piece_id, &action.to);
            },
            ToReserve => {
                let piece_id = self.remove_piece(&action.from.unwrap());
                self.add_reserve_piece(piece_id, self.current_player);
            },
        }
        if advance_player {
            self.next_player();
        }
    }

}