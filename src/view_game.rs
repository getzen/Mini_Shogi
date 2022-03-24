// View
// Responsible for drawing and polling for events.

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
use crate::sprite::SpriteKind;
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
    sprites: Vec<Sprite>,
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
            sprites: Vec::new(),
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
        // board
        let mut texture = Sprite::load_texture("square.png").await;
        for c in 0..self.columns {
            for r in 0..self.rows {
                let position = self.center_position_for(&Coord(c,r));
                let mut square = Sprite::new(Square, texture, position);
                square.coord = Coord(c,r);
                square.id = self.sprites.len();
                self.sprites.push(square);
            }
        }
        // Reserve, player 0
        texture = Sprite::load_texture("reserve.png").await;
        let mut reserve = Sprite::new(Reserve, texture, RESERVE_0_CENTER);
        reserve.id = self.sprites.len();
        self.sprites.push(reserve);
        // Reserve, player 1
        reserve = Sprite::new(Reserve, texture, RESERVE_1_CENTER);
        reserve.id = self.sprites.len();
        self.sprites.push(reserve);
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

    #[allow(dead_code)]
    fn sprites_for(&mut self, kind: SpriteKind) -> Vec<&mut Sprite> {
        self.sprites.iter_mut()
        .filter(|s| s.kind == kind)
        .collect()
    }

    fn sprite_ids_for(&self, kind: SpriteKind) -> Vec<usize> {
        self.sprites.iter()
        .enumerate()
        .filter(|s| s.1.kind == kind)
        .map(|s| s.0)
        .collect()
        // Old school:
        // let mut ids = Vec::new();
        // for (index, sprite) in self.sprites.iter().enumerate() {
        //     if sprite.kind == kind {
        //         ids.push(index);
        //     }
        // }
        // ids
    }

    #[allow(dead_code)]
    fn square_for(&mut self, coord: &Coord) -> &mut Sprite {
        self.sprites.iter_mut()
        .find(|s| s.kind == Square && s.coord == *coord)
        .unwrap()
    }

    #[allow(dead_code)]
    fn square_id_for(&self, coord: &Coord) -> usize {
        let sprite = self.sprites.iter()
        .find(|s| s.kind == Square && s.coord == *coord);
        sprite.unwrap().id
    }

    #[allow(dead_code)]
    fn piece_for(&mut self, coord: &Coord) -> &mut Sprite {
        self.sprites.iter_mut()
        .find(|s| s.kind == Piece && s.coord == *coord)
        .unwrap()
    }

    #[allow(dead_code)]
    fn piece_id_for(&self, coord: &Coord) -> usize {
        let sprite = self.sprites.iter().find(
            |s| s.kind == Piece && s.coord == *coord);
        sprite.unwrap().id
    }

    pub async fn add_piece_to(&mut self, coord: &Coord, kind: PieceKind, player: usize) {
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
        piece.coord = *coord;
        let id = self.sprites.len();
        piece.id = id;
        self.square_for(coord).contains_id = Some(id);
        self.sprites.push(piece);
    }

    pub fn move_piece(&mut self, from: &Coord, to: &Coord) {
        let id = self.piece_id_for(from);
        self.sprites[id].coord = *to;
        let to_position = self.center_position_for(to);
        self.sprites[id].animate_move(to_position, Duration::from_secs_f32(0.75));

        self.square_for(from).contains_id = None;
        self.square_for(to).contains_id = Some(id);
    }

    pub fn capture_piece(&mut self, coord: &Coord, capturing_player: usize) {
        let id = self.piece_id_for(coord);
        let mut to_position = RESERVE_0_CENTER;
        if capturing_player == 1 {
            to_position = RESERVE_1_CENTER;
        }
        self.sprites[id].animate_move(to_position, Duration::from_secs_f32(0.75));
    }

    pub fn handle_events(&mut self) {
        // Key presses.
        if is_key_down(KeyCode::Escape) {
            self.message_sender.send(Message::ShouldExit);
        }

        // Mouse position and buttons.
        let mouse_pos = mouse_position();
        let left_button = is_mouse_button_released(MouseButton::Left);

        for id in self.sprite_ids_for(Square) {
            if left_button && self.sprites[id].contains(mouse_pos) {
                let coord = self.sprites[id].coord;
                if self.sprites[id].contains_id.is_some() {
                    self.message_sender.send(Message::PieceSelected(coord));
                } else {
                    self.message_sender.send(Message::SquareSelected(coord));
                }
            }
        }
    }

    pub fn update(&mut self, time_delta: Duration) {
        for sprite in &mut self.sprites {
            sprite.update(time_delta);
        }
    }

    pub fn draw_board(&mut self) {
        clear_background(Color::from_rgba(
            BACKGROUND_COLOR.0, BACKGROUND_COLOR.1, BACKGROUND_COLOR.2, 255));
        for sprite in &mut self.sprites {
            sprite.draw();
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

    pub fn selected_piece_coord(&self) -> Option<Coord> {
        if let Some(selected) = self.selected_piece {
            return Some(self.sprites[selected].coord);
        }
        None
    }

    pub fn select_piece(&mut self, coord: &Coord) {
        if let Some(old_selected) = self.selected_piece {
            if old_selected == self.piece_id_for(coord) {
                return; // piece is already selected
            }
            self.unselect_piece();
        }
        for piece in self.sprites_for(Piece) {
            if piece.coord == *coord {
                piece.highlighted = true;
                self.selected_piece = Some(piece.id);
                break;
            }
        }
    }

    pub fn unselect_piece(&mut self) {
        if let Some(selected) = self.selected_piece {
            self.sprites[selected].highlighted = false;
            self.selected_piece = None;
        }
    }

    pub fn set_move_to_coords(&mut self, coords:Vec<Coord>) {
        for square in self.sprites_for(Square) {
            square.highlighted = coords.contains(&square.coord);
        }
        self.move_to_coords = coords;

        //     let id = self.square_id_for(coord);
        //     self.sprites[id].highlighted = coords.contains(coord);
        // }
    }

    pub fn is_move_to_coord(&self, coord: &Coord) -> bool {
        self.move_to_coords.contains(coord)
    }

    // /// Toggles highlighting for the piece with the given coord.
    // /// Turns off highlighting for all others, since only one piece
    // /// may be selected at a time.
    // pub fn toggle_piece_highlighting(&mut self, coord: &Coord) -> bool {
    //     let mut on = false;
    //     for piece in self.sprites_for(Piece) {
    //         if piece.coord == *coord {
    //             piece.highlighted = !piece.highlighted;
    //             on = piece.highlighted;
    //         } else {
    //             piece.highlighted = false;
    //         }
    //     }
    //     on
    // }

    // pub fn highlighted_piece_coord(&mut self) -> Option<Coord> {
    //     for piece in self.sprites_for(Piece) {
    //         if piece.highlighted {
    //             return Some(piece.coord);
    //         }
    //     }
    //     None
    // }

    /// Highlights the given square coords and turns it off for all others.
    pub fn highlight_squares(&mut self, coords: Vec<Coord>) {
        self.unhighlight_all_squares();

        // Highlight the new.
        for coord in coords {
            self.square_for(&coord).highlighted = true;
        }
    }

    /// Does what is says on the tin.
    pub fn unhighlight_all_squares(&mut self) {
        for square in self.sprites_for(Square) {
            square.highlighted = false;
        }
    }
}