// Game
// Primitives are used to keep Game instances on the stack and hashable.

use std::collections::HashSet;

use crate::game::GameState::*;
use crate::game::GameLocation::*;
use crate::piece::Piece;
use crate::piece::PieceKind::*;

pub const COLS: usize = 5;
pub const ROWS: usize = 6;
const GRID_COUNT: usize = 30;
const PIECES_PER_PLAYER: usize = 8;

#[allow(dead_code)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
 //              piece.id, location_index, is_capture. See Game.last_move
pub struct Move (pub usize, pub usize, pub bool);

const STARTING_POSITION: &str = "SGKGS------PPP--ppp------sgkgs";

#[derive(Clone, Copy, Debug, Hash)]
pub struct Game {
    // This owns all the pieces. grid and reserves just hold the ids.
    pub pieces: [Piece; PIECES_PER_PLAYER * 2],
    pub grid: [usize; GRID_COUNT],
    pub reserves: [[usize; PIECES_PER_PLAYER * 2 - 1]; 2],
    pub current_player: usize,
    pub state: GameState,
    pub last_move: Option<Move>,
}

impl Game {
    pub fn new() -> Self {
        // This will be replaced by the proper pieces during 'prepare'.
        let default_piece = Piece::new(0, Pawn, 0);
        Self {
            pieces: [default_piece; PIECES_PER_PLAYER * 2],
            grid: [NONE; GRID_COUNT],
            reserves: [[NONE; PIECES_PER_PLAYER * 2 - 1]; 2],
            current_player: 0,
            state: Ongoing,

            last_move: None,
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
            let mut node = *self;
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
                let (x0, y0) = Game::index_to_column_row(piece.location_index);

                // Single-space moves.
                let short_vectors = piece.short_move_vectors();
                for (x, y) in short_vectors {
                    let move_x = x0 as i8 + x;
                    let move_y = y0 as i8 + y;
                    let result = self.validate_move_coord(piece.player, move_x, move_y);
                    if result != -1 {
                        let to_index = Game::column_row_to_index(move_x as usize, move_y as usize);
                        move_indices.push(to_index);
                    }
                }

                // Multiple-space (long) moves.
                let long_vectors = piece.long_move_vectors();
                for (x, y) in long_vectors {
                    let mut loop_x = x0 as i8;
                    let mut loop_y = y0 as i8;
                    loop {
                        loop_x += x;
                        loop_y += y;
                        let result = self.validate_move_coord(piece.player, loop_x, loop_y);
                        // Add if empty square or capture.
                        if result == 0 || result == 1 {
                            let to_index = Game::column_row_to_index(loop_x as usize, loop_y as usize);
                            move_indices.push(to_index);
                        }
                        // Break if illegal move or capture.
                        if result == -1 || result == 1 {
                            break;
                        }
                    }
                }
            },
            Reserve => {
                if piece.kind != Pawn {
                    move_indices.append(&mut self.empty_grid_indices());
                }
                else { // Pawn
                    // Per the rules, pawns cannot be placed on same column as another of
                    // the player's pawns.
                    // Get verboten columns. Optimization opportunity.
                    let mut columns = HashSet::<usize>::new();
                    for p in &self.pieces {
                        if p.player != piece.player || p.location != Board || p.kind != Pawn {
                            continue;
                        }
                        let (c, _) = Game::index_to_column_row(p.location_index);
                        columns.insert(c);
                    }

                    for empty in self.empty_grid_indices() {
                        // Need to skip the last row per the rules.
                        if piece.player == 0 && empty >= GRID_COUNT - COLS {
                            continue;
                        }
                        if piece.player == 1 && empty < COLS {
                            continue;
                        }
                        // Check if empty is in the same column as another pawn.
                        let (empty_c, _) = Game::index_to_column_row(empty);
                        if columns.contains(&empty_c) { continue; }
                        move_indices.push(empty);
                    }

                }
            },
            _ => {},
        }
        move_indices
    }

    /// Checks the given board move-to square and returns:
    ///   -1 if move is out of bounds or lands on own player,
    ///    0 if move is to empty square,
    ///   +1 if move is capture of enemy piece.
    fn validate_move_coord(&self, player: usize, x: i8, y: i8) -> i8 {
        // Is this move out of bounds?
        if x < 0 || x as usize  >= COLS || y < 0 || y as usize >= ROWS {
            return -1;
        }        
        // Does this land on own piece?
        let to_index = Game::column_row_to_index(x as usize, y as usize);
        let onto_player = self.player_for_location_index(to_index);
        if onto_player.is_none() {
            return 0;
        }
        // Land on own player?
        if onto_player.unwrap() == player {
            return -1;
        }
        // Must land on enemy piece.
        return 1;
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
        let mut capture = false; // for last_move
        if captured_id != NONE {
            capture = true;
            self.pieces[captured_id].player = player;
            self.pieces[captured_id].location = Reserve;
            // Demote?
            if let Some(demote_kind) = self.pieces[captured_id].demotion_kind() {
                self.pieces[captured_id].kind = demote_kind;
            }
            // Find a spot for the capture piece
            if let Some(available_index) = self.available_reserve_index(player) {
                self.pieces[captured_id].location_index = available_index;
                self.reserves[player][available_index] = captured_id;
            }
        }

        let mut check_for_promotion = true;

        // Move
        // First, remove from old location.
        let location_index = self.pieces[piece_id].location_index;
        match self.pieces[piece_id].location {
            Board => {
                self.grid[location_index] = NONE;
            },
            Reserve => {
                self.reserves[player][location_index] = NONE;
                check_for_promotion = false;
            },
            _ => panic!(""),
        }
        
        // Then, move to new.
        self.grid[to_index] = piece_id;
        self.pieces[piece_id].location = Board;
        self.pieces[piece_id].location_index = to_index;

        // Promote?
        if check_for_promotion && self.is_promotion_zone(player, to_index) {
            if let Some(promo_kind) = self.pieces[piece_id].promotion_kind() {
                self.pieces[piece_id].kind = promo_kind;
            }
        }

        self.last_move = Some(Move(piece_id, to_index, capture));

        self.next_player();
    }

    fn available_reserve_index(&self, player: usize) -> Option<usize> {
        for (index, id) in self.reserves[player].iter().enumerate() {
            if *id == NONE { return Some(index) }
        }
        None
    }

    fn is_promotion_zone(&self, player: usize, location_index: usize) -> bool {
        if player == 0 {
            return location_index >= GRID_COUNT - COLS * 2;
        }
        location_index < COLS * 2
    }

    pub fn create_piece(kind: char, id: usize) -> Piece {
        match kind {
            'K' => Piece::new(id, King, 0),
            'G' => Piece::new(id, Gold, 0),
            'S' => Piece::new(id, Silver, 0),
            'P' => Piece::new(id, Pawn, 0),
            'k' => Piece::new(id, King, 1),
            'g' => Piece::new(id, Gold, 1),
            's' => Piece::new(id, Silver, 1),
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
        for (index, kind) in position.chars().enumerate() {
            if kind != '-' {
                let mut piece = Game::create_piece(kind, piece_id);
                piece.location = Board;
                piece.location_index = index;
                self.pieces[piece_id] = piece;
                self.grid[index] = piece.id;
                piece_id += 1;
            }
        }
    }

    /// Advance to the next player.
    pub fn next_player(&mut self) {
        self.current_player = 1 - self.current_player;
    }

    /// Gets the id of the player's king.
    fn king_id_(&self, player: usize) -> Option<usize> {
        let opt_piece = self.pieces
        .iter()
        .find(|p| p.player == player && p.kind == King);
        if let Some(piece) = opt_piece {
            return Some(piece.id);
        }
        // Not found. King must be captured.
        None
    }

    /// Simple verion. If opponent's piece is captured, it's a win.
    fn is_win(&self, player: usize) -> bool {
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

    #[allow(dead_code)]
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