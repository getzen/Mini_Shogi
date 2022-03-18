// View
// Responsible for drawing and polling for events.

use std::sync::mpsc::Sender;

use macroquad::prelude::*;

use crate::game::Coord;
use crate::controller::{Message, AppState};
use crate::controller::AppState::*;
use crate::message_sender::MessageSender;
use crate::sprite::Sprite;
use crate::text::Text;

const BOARD_POS: (f32, f32) = (100.0, 100.0);
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
    // reserve0, // Player 0 piece reserve area.
    // reserve1,
    // text_status,  // Your turn. Game over. Etc.
    // text_ai_thinking, // nodes/sec, pv
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

    pub async fn add_piece_to(&mut self, coord: &Coord, player: usize) {
        let texture = match player {
            0 => Sprite::load_texture("x.png").await,
            1 => Sprite::load_texture("o.png").await,
            _ => panic!("add_piece_to: player not found")
        };    
        let position = self.center_position_for(coord);
        let piece = Sprite::new(texture, position);
        self.pieces.push(piece);
    }

    pub fn handle_events(&mut self) {
        // Key presses.
        if is_key_down(KeyCode::Escape) {
            self.message_sender.send(Message::ShouldExit);
        }

        // Mouse position and buttons.
        let left_button_released = is_mouse_button_released(MouseButton::Left);
        
        for square in &mut self.squares {
            let on_square = square.highlight_on_mouse_over();
            if on_square && left_button_released {
                self.message_sender.send(Message::CoordSelected(square.coord));
            }
        }
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