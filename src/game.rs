// Game
// Primitives are used to keep Game instances on the stack and hashable.

//use slotmap::{DefaultKey, SlotMap};

use crate::Action;
use crate::action::ActionKind::*;
use crate::GameState::*;
use crate::Piece;
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

    /// Gets the id of the player's king.
    fn king_id_(&self, player: usize) -> Option<usize> {
        let piece = self.pieces
        .iter().find(|p| p.player == player && p.kind == King);
        if piece.is_some() {
            return Some(piece.unwrap().id)
        }
        // Not found. King was captured.
        None
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

    pub fn player_for(&self, piece_id: usize) -> usize {
        self.pieces[piece_id].player
    }

    pub fn coord_for(&self, piece_id: usize) -> Option<Coord> {
        self.pieces[piece_id].coord
    }

    // Parachuting rules:
    // 1. A pawns can never be parachuted into the last row since it could not move.
    // 2. Two pawns belonging to the same player can never be positioned in the same column.
    // However, having a pawn and a upgraded pawn in the same column is allowed.
    // 3. It is forbidden to put a parachuting pawn in front of the opponent king if it
    // creates “checkmate”.
    pub fn parachute_coords(&self, for_pawn: bool) -> Vec<Coord> {
        let mut empties = Vec::new();
        for (i, piece_id) in self.grid.iter().enumerate() {
            if *piece_id != NONE { continue };
            if for_pawn {
                // Rule 1. Need to skip the last row per the rules.
                if self.current_player == 0 && i >= GRID_COUNT - COLS {
                    continue;
                }
                if self.current_player == 1 && i < COLS {
                    continue;
                }
            }

            // need fns for attacked coords, piece move coords, is_checkmate

            // All clear. Add to empties.
            empties.push(Game::index_to_coord(i));
            
        }
        empties
    }

    /// Returns a tuple containing a vec of the move coords for the
    /// given piece and a vec of the capture coords. This is not for
    /// parachuting pieces. If capture_own is true, then the attack
    /// coords will include attacked coords with own pieces on them.
    /// This is needed for checkmate determination.
    fn move_and_attack_coords(&mut self, piece_id: usize, capture_own: bool) -> (Vec<Coord>, Vec<Coord>) {
        let mut move_coords = Vec::new();
        let mut attack_coords = Vec::new();
        let piece = &self.pieces[piece_id];
        let player = piece.player;
        let vectors = piece.move_vectors();
        let coord = piece.coord.unwrap();

        for (x, y) in vectors {
            let move_x = coord.0 as i8 + x;
            let move_y = coord.1 as i8 + y;
            // Is this move out of bounds?
            if move_x < 0 || move_x as usize  >= COLS || move_y < 0 || move_y as usize >= ROWS {
                continue;
            }
            let to_coord = Coord(move_x as usize, move_y as usize);
            let onto_id = self.get_piece(&to_coord);
            // Is the square empty?
            if onto_id == NONE {
                move_coords.push(to_coord);
                continue;
            }
            let onto_player = self.pieces[onto_id].player;
            // Does this land on own piece?
            if !capture_own && onto_player == player { 
                continue;
            }
            attack_coords.push(to_coord);
        }
        (move_coords, attack_coords)
    }

    /// Simple verion. If opponent's piece is captured, it's a win.
    fn is_win(&mut self, player: usize) -> bool {
        let king_id = self.king_id_(1 - player);
        if king_id.is_some() {return false;}
        true
    }

    /// Determines if the given player has won.
    fn is_win_real(&mut self, player: usize) -> bool {

        let king_id = self.king_id_(1 - player).unwrap();
        let king_coord = self.pieces[king_id].coord.unwrap();

        // Get the player's pieces. *** OPTIMIZE ***
        let mut player_ids = Vec::new();
        for id in self.grid {
            if id == NONE { continue; }
            if self.pieces[id].player == self.current_player {
                player_ids.push(id);
            }
        }

        // Get the moves/captures for each player piece.
        for id in player_ids {
            let from_coord = self.pieces[id].coord;
            let (move_coords, attack_coords) = self.move_and_attack_coords(id, true);

             // If this piece doesn't attack king, continue.
            if !attack_coords.contains(&king_coord) {
                continue;
            }

            // King is under attack. Can he escape or capture?
            let (king_move_coords, king_capture_coords) = self.move_and_attack_coords(king_id, false);
            // Check escape first.
            let mut can_escape = false;
            for escape_coord in king_move_coords {
                if !move_coords.contains(&escape_coord) && !attack_coords.contains(&escape_coord) {
                    // He can escape.
                    can_escape = true;
                    println!("king can escape to {:?}", escape_coord);
                    return false;
                }
                // Must be checkmate, unless other opponent piece can capture. 
            }
            println!("checkmate?");

        }

        // Novice game: if player king on back row and not in "check": true.

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
        // Get player's grid pieces. Optimization opportunity here.
        let mut grid_ids = Vec::new();
        for id in self.grid {
            if id == NONE { continue; }
            if self.pieces[id].player == self.current_player {
                grid_ids.push(id);
            }
        }
        // Get the moves for each grid piece.
        for id in grid_ids {
            let from_coord = self.pieces[id].coord;
            let (move_coords, capture_coords) = self.move_and_attack_coords(id, false);
            for move_coord in &move_coords {
                let action = Action::new(
                    MoveNoCapture, 
                    id, 
                    from_coord, 
                    *move_coord, 
                    None);
                actions.push(action);
            }
            for capture_coord in &capture_coords {
                let capture_id = self.get_piece(capture_coord);
                let action = Action::new(
                    MoveWithCapture, id,
                    from_coord, 
                    *capture_coord, 
                    Some(capture_id));
                    actions.push(action);
            }   
        }
        
        // Parachute actions.
        // Get pieces in player's reserve.
        let mut reserve_ids = Vec::new();
        for id in self.reserves[self.current_player]  {
            if id == NONE { continue; }
            reserve_ids.push(id);
        }

        for id in reserve_ids {
            // Identical pieces are not filtered out even though actions would be the same.
            // Possibly create a HashSet to store piece kinds and 'continue' when match found.

            let from_coord = self.pieces[id].coord;

            // Parachute coords checks for rules 1, 2, 3
            let to_coords: Vec<Coord>;
            if self.pieces[id].kind == Pawn {
                to_coords = self.parachute_coords(true);
            } else {
                to_coords = self.parachute_coords(false);
            }
            for to_coord in to_coords {
                let action = Action::new(
                    FromReserve, id, from_coord, to_coord, None);
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
                // Move captured piece to player reserve.
                let captured_id = self.remove_piece(&action.to);
                self.pieces[captured_id].player = self.current_player;
                self.add_reserve_piece(captured_id, self.current_player);
                // Move player piece.
                self.remove_piece(&action.from.unwrap());
                self.set_piece(action.piece_id, &action.to);
            }
            FromReserve => {
                let _ = self.remove_reserve_piece(action.piece_id, self.current_player);
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