// View
// Responsible for drawing and polling for events.

use std::sync::mpsc::Sender;

use macroquad::prelude::*;

use crate::game::Coord;
use crate::controller::{Message, AppState};
use crate::controller::AppState::*;
use crate::message_sender::MessageSender;
use crate::piece::PieceKind;
use crate::piece::PieceKind::*;
use crate::sprite::Sprite;
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
    squares: Vec<Sprite>,
    pieces: Vec<Sprite>,
    status_text: Text,
    ai_progress_text: Text,
    selected_piece: Option<usize>,
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
            squares: Vec::<Sprite>::new(),
            pieces: Vec::<Sprite>::new(),
            status_text: Text::new(
                "Welcome!".to_owned(), 
                TEXT_STATUS_POS,
                18,
                Some("Menlo.ttc"),
            ).await,
            ai_progress_text,
            selected_piece: None,
        }
    }

    pub async fn prepare(&mut self) {
        let texture = Sprite::load_texture("square.png").await;
        for c in 0..self.columns {
            for r in 0..self.rows {
                let position = self.center_position_for(&Coord(c,r));
                let mut square = Sprite::new(texture, position);
                square.coord = Coord(c,r);
                self.squares.push(square);
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

    pub async fn add_piece_to(&mut self, coord: &Coord, kind: PieceKind, player: usize) {
        let texture = match kind {
            King => Sprite::load_texture("king.png").await,
            Rook => Sprite::load_texture("rook.png").await,
            Bishop => Sprite::load_texture("bishop.png").await,
            Pawn => Sprite::load_texture("pawn.png").await,
        };    
        let position = self.center_position_for(coord);
        let mut piece = Sprite::new(texture, position);
        piece.set_size(Some((80., 80.)));
        if player == 1 {
            piece.set_rotation(std::f32::consts::PI);
        }
        self.pieces.push(piece);
    }

    pub fn handle_events(&mut self) {
        // Key presses.
        if is_key_down(KeyCode::Escape) {
            self.message_sender.send(Message::ShouldExit);
        }

        // Mouse position and buttons.
        let mouse_pos = mouse_position();
        let left_button_released = is_mouse_button_released(MouseButton::Left);
        
        for square in &mut self.squares {
            let on_square = square.highlight_on_mouse_over();
            if on_square && left_button_released {
                //self.message_sender.send(Message::SquareSelected(square.coord));
            }
        }

        // // Unselected current piece
        // if let Some(p) = self.selected_piece {
        //     let mut old = self.piece_sprite_with(p);
        //     if old.is_some() {
        //        old.unwrap().highlighted = false;
        //     }
        // }

        for piece in &mut self.pieces {
            //piece.highlighted = false; // assume no selection
            let on_piece = piece.contains(mouse_pos);
            if on_piece && left_button_released {
                // Select new piece
                self.selected_piece = Some(piece.id);
                piece.highlighted = true;
                //self.message_sender.send(Message::PieceSelected(piece.coord));
                
                    
            }
                
            
        }
    }

    fn piece_sprite_with(&self, id: usize) -> Option<&Sprite> {
        for piece in &self.pieces {
            if piece.id == id {
                return Some(piece);
            }
        }
        None
    }

    pub fn draw_board(&mut self) {
        clear_background(Color::from_rgba(81, 81, 81, 255));

        // Squares
        for square in &mut self.squares {
            square.draw();
        }

        // Square coord labels

        // Pieces
        for piece in &mut self.pieces {
            piece.draw();
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
        for square in &mut self.squares {
            if square.coord == *coord {
                square.highlighted = true;
                break;
            }
        }
    }
}