// View
// Responsible for drawing and polling for events.

use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::time::Duration;

use macroquad::prelude::*;

use crate::game::Coord;
use crate::controller::AppState;
use crate::controller::AppState::*;
use crate::message_sender::{Message, MessageSender};
use crate::piece::PieceKind;
use crate::piece::PieceKind::*;
use crate::sprite::*;
use crate::sprite::SpriteKind::*;
use crate::text::Text;

const BACKGROUND_COLOR: (u8, u8, u8) = (144, 144, 137);
const BOARD_CORNER: (f32, f32) = (240.0, 95.0);
const SQUARE_SIZE: f32 = 90.0; // matches the square.png size
const SQUARE_GAP: f32 = 5.0;
const RESERVE_0_CENTER: (f32, f32) = (715., 615.);
const RESERVE_1_CENTER: (f32, f32) = (95., 140.);
const TEXT_STATUS_CENTER: (f32, f32) = (400., 60.0);
const AI_PROGRESS_CORNER: (f32, f32) = (20., 770.);
const PIECE_SIZE: (f32, f32) = (70., 75.);

pub struct ViewGame {
    message_sender: MessageSender, // sends event messages to controller
    columns: usize,
    rows: usize,
    squares: HashMap<Coord, Sprite>,
    reserves: Vec<HashMap<Coord, Sprite>>,
    pieces: HashMap<usize, Sprite>, // usize is id matching model's Piece id
    pub selected_piece: Option<usize>,
    pub move_to_coords: Vec<Coord>,
    status_text: Text,
    ai_progress_text: Text,
}

impl ViewGame {
    pub async fn new(tx: Sender<Message>, columns: usize, rows: usize) -> Self {
        let mut ai_progress_text = Text::new(
            "".to_owned(), 
            AI_PROGRESS_CORNER,
            12,
            Some("Menlo.ttc"),
        ).await;
        ai_progress_text.centered = false;

        Self {
            message_sender: MessageSender::new(tx, None),
            columns, rows,
            squares: HashMap::new(),
            reserves: vec!(HashMap::new(), HashMap::new()),
            pieces: HashMap::new(),
            selected_piece: None,
            move_to_coords: Vec::new(),
            status_text: Text::new(
                "Welcome!".to_owned(), 
                TEXT_STATUS_CENTER,
                18,
                Some("Menlo.ttc"),
            ).await,
            ai_progress_text,
        }
    }

    pub async fn prepare(&mut self) {
        // Board
        let mut texture = Sprite::load_texture("square.png").await;
        for c in 0..self.columns {
            for r in 0..self.rows {
                let position = self.center_position_for(&Coord(c,r));
                let square = Sprite::new(Square, texture, position);
                self.squares.insert(Coord(c,r), square);
            }
        }
        // Reserve, player 0
        texture = Sprite::load_texture("reserve.png").await;
        let mut reserve = Sprite::new(Reserve, texture, RESERVE_0_CENTER);
        self.reserves[0].insert(Coord(0,0), reserve);
        // Reserve, player 1
        reserve = Sprite::new(Reserve, texture, RESERVE_1_CENTER);
        self.reserves[1].insert(Coord(0,0), reserve);
    }

    pub async fn add_piece(&mut self, coord: &Coord, id: usize, kind: PieceKind, player: usize) {
        let texture = match kind {
            King => Sprite::load_texture("king.png").await,
            Rook => Sprite::load_texture("rook.png").await,
            Bishop => Sprite::load_texture("bishop.png").await,
            Pawn => Sprite::load_texture("pawn.png").await,
        };    
        let position = self.center_position_for(coord);
        let mut piece = Sprite::new(Piece, texture, position);
        piece.set_size(Some(PIECE_SIZE));
        if player == 1 {
            piece.set_rotation(std::f32::consts::PI);
        }
        piece.id = Some(id);
        self.pieces.insert(id, piece);
    }

    fn corner_position_for(&self, coord: &Coord) -> (f32, f32) {
        // We want row 0 at the bottom of the board, not the top, so flip the row.
        let flip_r = self.rows - coord.1 - 1;
        let x = BOARD_CORNER.0 + SQUARE_GAP + (SQUARE_SIZE + SQUARE_GAP) * coord.0 as f32;
        let y = BOARD_CORNER.1 + SQUARE_GAP + (SQUARE_SIZE + SQUARE_GAP) * flip_r as f32;
        (x, y)
    }

    fn center_position_for(&self, coord: &Coord) -> (f32, f32) {
        let pos = self.corner_position_for(coord);
        let x = pos.0 + SQUARE_SIZE / 2.0;
        let y = pos.1 + SQUARE_SIZE / 2.0;
        (x, y)
    }

    pub fn move_piece(&mut self, id: usize, to: &Coord) {
        let to_position = self.center_position_for(to);
        if let Some(piece) = self.pieces.get_mut(&id) {
            piece.animate_move(to_position, Duration::from_secs_f32(0.75));
        }
    }

    pub fn capture_piece(&mut self, id: usize, capturing_player: usize) {
        if let Some(piece) = self.pieces.get_mut(&id) {
            let mut to_position = RESERVE_0_CENTER;
            if capturing_player == 1 {
                to_position = RESERVE_1_CENTER;
            }
            let mut theta: f32 = 0.0;
            if capturing_player == 1 {
                theta = std::f32::consts::PI
            }
            piece.set_rotation(theta);
            piece.animate_move(to_position, Duration::from_secs_f32(0.75));
        }
    }

    pub fn handle_events(&mut self) {
        // Key presses.
        if is_key_down(KeyCode::Escape) {
            self.message_sender.send(Message::ShouldExit);
        }

        // Mouse position and buttons.
        let mouse_pos = mouse_position();
        let left_button = is_mouse_button_released(MouseButton::Left);

        let mut clicked_handled = false;

        // Detect piece hits first
        for (id, piece) in &self.pieces {
            if left_button && piece.contains(mouse_pos) {
                self.message_sender.send(Message::PieceSelected(*id));
                clicked_handled = true;
            }
        }

        if !clicked_handled {
            // Squares
            for (coord, square) in &self.squares {
                if left_button && square.contains(mouse_pos) {
                    self.message_sender.send(Message::SquareSelected(*coord));
                    clicked_handled = true;
                }
            }
        }
        
        if !clicked_handled {
            // Reserves
            for i in 0..2 {
                for (coord, reserve) in &self.reserves[i] {
                    if left_button && reserve.contains(mouse_pos) {
                        self.message_sender.send(Message::ReserveSelected((i, *coord)));
                    }
                }
            }
        }
    }

    pub fn update(&mut self, time_delta: Duration) {
        for (_, piece) in &mut self.pieces {
            piece.update(time_delta);
        }
    }

    pub fn draw_board(&mut self) {
        clear_background(Color::from_rgba(
            BACKGROUND_COLOR.0, BACKGROUND_COLOR.1, BACKGROUND_COLOR.2, 255));
        // Squares
        for (_, square) in &mut self.squares {
            square.draw();
        }
        // Reserves
        for i in 0..2 {
            for (_, reserve) in &mut self.reserves[i] {
                reserve.draw();
            }
        }
        // Pieces
        for (_, piece) in &mut self.pieces {
            piece.draw();
        }
    }

    pub fn draw_ui(&mut self, state: &AppState, other_text: &str) {
        // Status text
        let text = match state {
            HumanTurn => "Make move.",
            AIThinking => "AI thinking...",
            Player0Won => "Player 1 wins!",
            Player1Won => "Player 2 wins!",
            Draw => "The game is a draw.",
            _ => {"Undefined state."},
        };

        self.status_text.text = text.to_owned();
        self.status_text.draw();

        self.ai_progress_text.text = other_text.to_owned();
        self.ai_progress_text.draw();
    }

    pub async fn end_frame(&self) {
        next_frame().await;
    }

    pub fn selected_piece_id(&self) -> Option<usize> {
        if let Some(selected) = self.selected_piece {
            return self.pieces[&selected].id;
        }
        None
    }

    pub fn select_piece(&mut self, id: usize) {
        if let Some(old_id) = self.selected_piece {
            if old_id == id {
                return; // piece is already selected
            }
            self.unselect_piece();
        }
        if let Some(piece) = self.pieces.get_mut(&id) {
            piece.highlighted = true;
            self.selected_piece = Some(id);
        }
    }

    pub fn unselect_piece(&mut self) {
        if let Some(id) = self.selected_piece {
            if let Some(piece) = self.pieces.get_mut(&id) {
                piece.highlighted = false;
            }
            self.selected_piece = None;
        }
    }

    pub fn set_move_to_coords(&mut self, coords:Vec<Coord>) {
        for (coord, square) in &mut self.squares {
            square.highlighted = coords.contains(coord);
        }
        self.move_to_coords = coords;
    }

    pub fn is_move_to_coord(&self, coord: &Coord) -> bool {
        self.move_to_coords.contains(coord)
    }

    /// Highlights the given square coords and turns it off for all others.
    pub fn highlight_squares(&mut self, coords: Vec<Coord>) {
        self.unhighlight_all_squares();

        // Highlight the new.
        for coord in &coords {
            if let Some(square) = self.squares.get_mut(coord) {
                square.highlighted = true;
            }
        }
    }

    /// Does what is says on the tin.
    pub fn unhighlight_all_squares(&mut self) {
        for (_, square) in &mut self.squares {
            square.highlighted = false;
        }
    }
}