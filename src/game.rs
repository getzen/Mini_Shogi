// Board2

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coord(pub usize, pub usize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GameState {
    Ongoing,
    Draw,
    WinPlayer0,
    WinPlayer1,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Game {
    grid: [Option<Piece>; GRID_COUNT],
    reserves: [[Option<Piece>; PIECES_PER_PLAYER]; 2],
    pub current_player: usize,
    pub state: GameState,

    ////////
    // pieces: SlotMap<>?
    // reserves: [u64; 8]; 2] // use 'swap' to make it like a stack!
    // reserves_len
    // board_pieces: [[Option<Piece>; 9] ; 2]
    // board_pieces_len
    ///////
}

impl Game {
    pub fn new() -> Self {
        Self {
            grid: [None; GRID_COUNT],
            reserves: [[None; PIECES_PER_PLAYER]; 2],
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

    /// Get the board ready for a new game.
    pub fn prepare(&mut self) {
        // Put pieces on board.
        // Player 0.
        self.set_piece(Piece::new(0, Bishop, 0), &Coord(0,0));
        self.set_piece(Piece::new(0, King, 0), &Coord(1,0));
        self.set_piece(Piece::new(0, Rook, 0), &Coord(2,0));
        self.set_piece(Piece::new(0, Pawn, 0), &Coord(1,1));
        // Player 1.
        self.set_piece(Piece::new(0, Bishop, 1), &Coord(0,0));
        self.set_piece(Piece::new(0, King, 1), &Coord(1,0));
        self.set_piece(Piece::new(0, Rook, 1), &Coord(2,0));
        self.set_piece(Piece::new(0, Pawn, 1), &Coord(1,1));
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
    pub fn get_piece(&mut self, coord: &Coord) -> &Option<Piece> {
        &self.grid[Game::coord_to_index(coord)]
    }

    /// Sets the piece at the given coord.
    pub fn set_piece(&mut self, piece: Piece, coord: &Coord) {
        // warn if space not empty?
        self.grid[Game::coord_to_index(coord)] = Some(piece);
    }

    /// Removes and returns the piece at the given coord.
    pub fn remove_piece(&mut self, coord: &Coord) -> Piece {
        let i = Game::coord_to_index(coord);
        let piece = self.grid[i]; // assume this makes a copy..
        self.grid[i] = None; // ...so we need to explicity set to None.
        piece.unwrap() // we want this to panic if None
    }

    pub fn remove_reserve_piece(&mut self, kind: PieceKind, player: usize) -> Option<Piece> {
        let mut piece: Option<Piece> = None;
        for (i, p) in self.reserves[player as usize].iter().enumerate() {
            if p.is_some() && p.unwrap().kind == kind {
                piece = *p;
                self.reserves[player as usize][i] = None;
                break;
            }
        }
        piece
    }

    pub fn add_reserve_piece(&mut self, piece: Piece) {
        let player = piece.player;
        for (i, p) in self.reserves[player as usize].iter().enumerate() {
            if p.is_none() {
                self.reserves[player as usize][i] = Some(piece);
                break;
            }
        }
    }

    // **** empty_indices instead, and avoid Coord2 conversion? ***
    /// Returns vector of coords that have no pieces.
    pub fn empty_coords(&self) -> Vec<Coord> {
        let mut empties = Vec::new();
        for (i, p) in self.grid.iter().enumerate() {
            if p.is_none() {
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
        

        let available_coords = self.empty_coords();
        let mut piece_kind = Pawn;
        if self.current_player == 1 {
            piece_kind = Pawn;
        }
        for coord in available_coords {
            let action = Action::new(FromReserve, piece_kind, coord);
            actions.push(action);
        }
        actions
    }

    pub fn perform_action(&mut self, action: &Action, advance_player: bool) {
        match action.action_kind {
            FromReserve => {
                let piece = self.remove_reserve_piece(action.piece_kind, self.current_player);
                self.set_piece(piece.unwrap(), &action.coord);
            },
            ToReserve => {
                let piece = self.remove_piece(&action.coord);
                self.add_reserve_piece(piece);
            },
        }
        if advance_player {
            self.next_player();
        }
    }

}