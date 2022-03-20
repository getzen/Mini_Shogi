// View
// Responsible for drawing and polling for events.

use std::sync::mpsc::Sender;

use macroquad::prelude::*;

use crate::game::{Coord, NONE};
use crate::controller::{Message, AppState};
use crate::controller::AppState::*;
use crate::message_sender::MessageSender;
use crate::piece::PieceKind;
use crate::piece::PieceKind::*;
use crate::sprite::*;
use crate::sprite::SpriteKind;
use crate::sprite::SpriteKind::*;
use crate::text::Text;

const BOARD_POS: (f32, f32) = (240.0, 40.0);
const SQUARE_SIZE: f32 = 100.0; // matches the square.png size
const SQUARE_GAP: f32 = 5.0;
// const RESERVE_0_ORIGIN
// const RESERVE_1_ORIGIN
const TEXT_STATUS_POS: (f32, f32) = (400., 500.0);
const AI_PROGRESS_POS: (f32, f32) = (20., 570.);

pub struct ViewGame {
    message_sender: MessageSender, // sends event messages to controller
    columns: usize,
    rows: usize,
    sprites: Vec<Sprite>,
    //squares: Vec<usize>, // indices to sprites
    //pieces: Vec<usize>, // indices to sprites
    pub selected_piece: Option<usize>,
    status_text: Text,
    ai_progress_text: Text,
}

impl ViewGame {
    pub async fn new(tx: Sender<Message>, columns: usize, rows: usize) -> Self {
        let mut ai_progress_text = Text::new(
            "".to_owned(), 
            AI_PROGRESS_POS,
            12,
            Some("Menlo.ttc"),
        ).await;
        ai_progress_text.centered = false;

        Self {
            message_sender: MessageSender::new(tx, None),
            columns, rows,
            sprites: Vec::new(),
            //squares: Vec::new(),
            //pieces: Vec::new(),
            selected_piece: None,
            status_text: Text::new(
                "Welcome!".to_owned(), 
                TEXT_STATUS_POS,
                18,
                Some("Menlo.ttc"),
            ).await,
            ai_progress_text,
        }
    }

    pub async fn prepare(&mut self) {
        let texture = Sprite::load_texture("square.png").await;
        for c in 0..self.columns {
            for r in 0..self.rows {
                let position = self.center_position_for(&Coord(c,r));
                let mut square = Sprite::new(Square, texture, position);
                square.coord = Coord(c,r);
                square.id = self.sprites.len();
                self.sprites.push(square);
            }
        }        
    }

    fn corner_position_for(&self, coord: &Coord) -> (f32, f32) {
        // We want row 0 at the bottom of the board, not the top, so flip the row.
        let flip_r = self.rows - coord.1 - 1;
        let x = BOARD_POS.0 + SQUARE_GAP + (SQUARE_SIZE + SQUARE_GAP) * coord.0 as f32;
        let y = BOARD_POS.1 + SQUARE_GAP + (SQUARE_SIZE + SQUARE_GAP) * flip_r as f32;
        (x, y)
    }

    fn center_position_for(&self, coord: &Coord) -> (f32, f32) {
        let pos = self.corner_position_for(coord);
        let x = pos.0 + SQUARE_SIZE / 2.0;
        let y = pos.1 + SQUARE_SIZE / 2.0;
        (x, y)
    }

    // fn sprites_with(&self, kind: SpriteKind) -> Vec<&Sprite> {
    //     self.sprites.iter().filter(|s| s.kind == kind).collect()
    // }

    fn sprite_ids_for(&self, kind: SpriteKind) -> Vec<usize> {
        let mut ids = Vec::new();
        for (index, sprite) in self.sprites.iter().enumerate() {
            if sprite.kind == kind {
                ids.push(index);
            }
        }
        ids
    }

    fn square_id_for(&self, coord: &Coord) -> usize {
        let sprite = self.sprites.iter().find(
            |s| s.kind == Square && s.coord == *coord);
        sprite.unwrap().id
    }

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
        piece.set_size(Some((80., 80.)));
        if player == 1 {
            piece.set_rotation(std::f32::consts::PI);
        }
        let id = self.sprites.len();
        piece.id = id;
        let square_id = self.square_id_for(coord);
        self.sprites[square_id].contains_id = Some(id);
        self.sprites.push(piece);
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

    pub fn draw_board(&mut self) {
        clear_background(Color::from_rgba(81, 81, 81, 255));

        for sprite in &mut self.sprites {
            sprite.draw();
        }
        
        // Reserves
        // self.reserve0.draw();
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

    #[allow(dead_code)]
    pub fn highlight_square(&mut self, coord: &Coord) {
        //self.square_with(coord).highlight = true;
    }
}