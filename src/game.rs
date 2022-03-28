// Game
// Primitives are used to keep Game instances on the stack and hashable.

use crate::GameState::*;
use crate::GameLocation::*;
use crate::Piece;
use crate::piece::PieceKind::*;

pub const COLS: usize = 3;
pub const ROWS: usize = 4;
const GRID_COUNT: usize = 12;
const PIECES_PER_PLAYER: usize = 4;

// #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
// pub struct Coord(pub usize, pub usize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GameState {
    Ongoing,
    Draw,
    WinPlayer0,
    WinPlayer1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameLocation {
    OutOfGame,
    Board,
    Reserve,
}

pub const NONE: usize = usize::MAX;
const STARTING_POSITION: &str = "BKR-P--p-rkb";

#[derive(Clone, Copy, Debug, Hash)]
pub struct Game {
    // This owns all the pieces. grid and reserves just hold the indices.
    pub pieces: [Piece; PIECES_PER_PLAYER * 2],
    pub grid: [usize; GRID_COUNT],
    pub reserves: [[usize; PIECES_PER_PLAYER]; 2],

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

    pub fn column_row_to_index(x: usize, y: usize) -> usize {
        y * COLS + x
    }

    pub fn index_to_column_row(index: usize) -> (usize, usize) {
        (index % COLS, index / COLS) 
    }

    pub fn piece_for(&self, id: usize) -> &Piece {
        &self.pieces[id]
    }

    pub fn player_for_piece_id(&self, id: usize) -> usize {
        self.pieces[id].player
    }

    fn player_for_location_index(&self, index: usize) -> Option<usize> {
        let id = self.grid[index];
        if id != NONE {
            return Some(self.player_for_piece_id(id));
        }
        None
    }

    pub fn location_index_for(&self, id: usize) -> usize {
        self.pieces[id].location_index
    }

    fn pieces_ids_for(&self, player: usize) -> Vec<usize> {
        let mut ids = Vec::new();
        for piece in &self.pieces {
            if piece.player == player {
                ids.push(piece.id);
            }
        }
        ids
    }

    /// Returns all child nodes (possible Game states) for the given player.
    pub fn child_nodes(&self, player: usize) -> Vec<Game> {
        let mut nodes = Vec::new();
        for id in &self.pieces_ids_for(player) {
            nodes.append(&mut self.child_nodes_for_piece(*id));
        }
        nodes
    }

    /// Returns all the child nodes (possible Game states) for the given piece_id.
    pub fn child_nodes_for_piece(&self, id: usize) -> Vec<Game> {
        let mut nodes = Vec::new();
        let move_indices = self.move_indices_for_piece(id);
        for to_index in move_indices {
            let mut node = self.clone();
            node.make_move(id, to_index);
            nodes.push(node);
        }
        nodes
    }

    /// Returns all the position indices the given piece may move to, excluding
    /// positions occupied by the player's own pieces.
    pub fn move_indices_for_piece(&self, id: usize) -> Vec<usize> {
        let mut move_indices = Vec::new();
        let piece = self.piece_for(id);

        match piece.location {
            Board => {
                let vectors = piece.move_vectors();
                let (x0, y0) = Game::index_to_column_row(piece.location_index);
                for (x, y) in vectors {
                    let move_x = x0 as i8 + x;
                    let move_y = y0 as i8 + y;
                    // Is this move out of bounds?
                    if move_x < 0 || move_x as usize  >= COLS || move_y < 0 || move_y as usize >= ROWS {
                        continue;
                    }
                    let to_index = Game::column_row_to_index(move_x as usize, move_y as usize);
                    // Does this land on own piece?
                    if let Some(onto_player) = self.player_for_location_index(to_index) { 
                        if piece.player == onto_player {
                            continue;
                        }
                    }
                    move_indices.push(to_index);
                }
            },
            Reserve => {
                for empty in self.empty_grid_indices() {
                    if piece.kind == Pawn {
                        // Need to skip the last row per the rules.
                        if piece.player == 0 && empty >= GRID_COUNT - COLS {
                            continue;
                        }
                        if piece.player == 1 && empty < COLS {
                            continue;
                        }
                    }
                    move_indices.push(empty);
                }
            },
            _ => {},
        }
        move_indices
    }

    /// Returns a vector of empty spots in the grid.
    fn empty_grid_indices(&self) -> Vec<usize> {
        let mut empties = Vec::new();
        for (index, val) in self.grid.iter().enumerate() {
            if *val == NONE {
                empties.push(index);
            }
        }
        empties
    }

    fn make_move(&mut self, piece_id: usize, to_index: usize) {
        let player = self.player_for_piece_id(piece_id);
        let captured_id = self.grid[to_index];

        // Capture?
        if captured_id != NONE {
            self.pieces[captured_id].player = player;
            self.pieces[captured_id].location = Reserve;
            if let Some(available_index) = self.available_reserve_index(player) {
                self.pieces[captured_id].location_index = available_index;
                self.reserves[player][available_index] = captured_id;
            }
        }

        // Move
        // First, remove from old location.
        let location_index = self.pieces[piece_id].location_index;
        match self.pieces[piece_id].location {
            Board => {
                self.grid[location_index] = NONE;
            },
            Reserve => {
                self.reserves[player][location_index] = NONE;
            },
            _ => panic!(""),
        }
        // Then, move to new.
        self.grid[to_index] = piece_id;
        self.pieces[piece_id].location = Board;
        self.pieces[piece_id].location_index = to_index;

        self.next_player();
    }

    fn available_reserve_index(&self, player: usize) -> Option<usize> {
        for (index, id) in self.reserves[player].iter().enumerate() {
            if *id == NONE { return Some(index) }
        }
        None
    }

    pub fn create_piece(kind: char, id: usize) -> Piece {
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
                let mut piece = Game::create_piece(kind, piece_id);
                piece.location = Board;
                piece.location_index = index;
                self.pieces[piece_id] = piece;
                self.grid[index] = piece.id;
                piece_id += 1;
            }
            index += 1;
        }
    }

    /// Advance to the next player.
    pub fn next_player(&mut self) {
        self.current_player = 1 - self.current_player;
    }

    /// Gets the id of the player's king.
    fn king_id_(&self, player: usize) -> Option<usize> {
        let piece = self.pieces
        .iter()
        .find(|p| p.player == player && p.kind == King);
        if piece.is_some() {
            return Some(piece.unwrap().id)
        }
        // Not found. King must be captured.
        None
    }

    /// Simple verion. If opponent's piece is captured, it's a win.
    fn is_win(&mut self, player: usize) -> bool {
        let king_id = self.king_id_(1 - player);
        if king_id.is_some() {return false;}
        true
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
        // else if self.actions_available().is_empty() {
        //     self.state = GameState::Draw;
        // }
        &self.state
    }

    pub fn debug(&self) {
        println!("--- debug ---");
        for (index, id) in self.grid.iter().enumerate() {
            if *id == NONE { continue;}
            println!("index: {}, id: {}", index, id);
        }
        println!("reserve 0");
        for (index, id) in self.reserves[0].iter().enumerate() {
            if *id == NONE { continue;}
            println!("index: {}, id: {}", index, id);
        }
        println!("reserve 1");
        for (index, id) in self.reserves[1].iter().enumerate() {
            if *id == NONE { continue;}
            println!("index: {}, id: {}", index, id);
        }
    }
}