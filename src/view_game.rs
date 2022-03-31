// View
// Responsible for drawing and polling for events.

use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::time::Duration;

use macroquad::prelude::*;

use crate::Game;
use crate::game::NONE;
use crate::controller::AppState;
use crate::controller::AppState::*;
use crate::message_sender::{Message, MessageSender};
use crate::Piece;
use crate::piece::PieceKind::{self, *};
use crate::sprite::*;
use crate::sprite::SpriteKind::*;
use crate::text::Text;

const BACKGROUND_COLOR: (u8, u8, u8) = (144, 144, 137);
const BOARD_CORNER: (f32, f32) = (165.0, 95.0);
const SQUARE_SIZE: f32 = 90.0; // matches the square.png size
const SQUARE_GAP: f32 = 5.0;
const PROMO_LINE_TOP: (f32, f32) = (405., 287.);
const PROMO_LINE_BOTTOM: (f32, f32) = (405., 477.);
const RESERVE_0_CENTER: (f32, f32) = (715., 615.);
const RESERVE_1_CENTER: (f32, f32) = (95., 140.);
const TEXT_STATUS_CENTER: (f32, f32) = (400., 60.0);
const AI_PROGRESS_CORNER: (f32, f32) = (20., 770.);
const PIECE_SIZE: (f32, f32) = (70., 75.);

pub struct ViewGame {
    message_sender: MessageSender, // sends event messages to controller
    columns: usize,
    rows: usize,
    squares: HashMap<usize, Sprite>, // key: location index
    promotion_lines: Vec<Sprite>,
    reserves: Vec<HashMap<usize, Sprite>>, //
    pieces: HashMap<usize, Sprite>, // key: model's Piece.id
    pub selected_piece: Option<usize>,
    pub move_indices: Vec<usize>, // all the spots the currently selected piece can move to
    status_text: Text,
    ai_progress_text: Text,
    piece_textures: HashMap<String, Texture2D>,
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
            promotion_lines: Vec::new(),
            reserves: vec!(HashMap::new(), HashMap::new()),
            pieces: HashMap::new(),
            selected_piece: None,
            move_indices: Vec::new(),
            status_text: Text::new(
                "Welcome!".to_owned(), 
                TEXT_STATUS_CENTER,
                18,
                Some("Menlo.ttc"),
            ).await,
            ai_progress_text,
            piece_textures: HashMap::new(),
        }
    }

    pub async fn prepare(&mut self) {
        // Load textures.
        let names = ["king.png", "gold.png", "silver.png", "silver_pro.png", "pawn.png", "pawn_pro.png"];
        for name in names {
            let texture = Sprite::load_texture(name).await;
            self.piece_textures.insert(name.to_owned(), texture);
        }

        // Board
        let mut texture = Sprite::load_texture("square.png").await;
        for c in 0..self.columns {
            for r in 0..self.rows {
                let index = Game::column_row_to_index(c, r);
                let position = self.center_position_for(index);
                let square = Sprite::new(Square, texture, position);
                self.squares.insert(index, square);
            }
        }

        // Promotion lines
        texture = Sprite::load_texture("line.png").await;
        let line_top = Sprite::new(Default, texture, PROMO_LINE_TOP);
        self.promotion_lines.push(line_top);
        let line_bottom = Sprite::new(Default, texture, PROMO_LINE_BOTTOM);
        self.promotion_lines.push(line_bottom);

        // Reserves
        texture = Sprite::load_texture("reserve.png").await;
        for i in 0..3 {
            // Reserve, player 0
            let mut pos_x = RESERVE_0_CENTER.0;
            let mut pos_y = RESERVE_0_CENTER.1 - i as f32 * (SQUARE_SIZE + SQUARE_GAP); 
            let mut reserve = Sprite::new(Reserve, texture, (pos_x, pos_y));
            self.reserves[0].insert(i, reserve);
            // Reserve, player 1
            pos_x = RESERVE_1_CENTER.0;
            pos_y = RESERVE_1_CENTER.1 + i as f32 * (SQUARE_SIZE + SQUARE_GAP);
            reserve = Sprite::new(Reserve, texture, (pos_x, pos_y));
            self.reserves[1].insert(i, reserve);
        }
    }

    fn texture_for(&self, piece_kind: PieceKind) -> Texture2D {
        let key = match piece_kind {
            King => "king.png",
            Gold => "gold.png",
            Silver => "silver.png",
            SilverPro => "silver_pro.png",
            Pawn => "pawn.png",
            PawnPro => "pawn_pro.png",
        };
        *self.piece_textures.get(&key.to_owned()).unwrap()
    }

    pub fn add_piece(&mut self, piece: &Piece) {
        let texture = self.texture_for(piece.kind);
        let position = self.center_position_for(piece.location_index);
        let mut sprite = Sprite::new(Piece, texture, position);
        sprite.set_size(Some(PIECE_SIZE));
        if piece.player == 1 {
            sprite.set_rotation(std::f32::consts::PI);
        }
        sprite.id = Some(piece.id);
        self.pieces.insert(piece.id, sprite);
    }

    // pub fn remove_piece(&mut self, piece: &Piece) {
    //     self.pieces.remove_entry(&piece.id);
    // }

    fn corner_position_for(&self, index: usize) -> (f32, f32) {
        let (x0, y0) = Game::index_to_column_row(index);
        // We want row 0 at the bottom of the board, not the top, so flip the row.
        let flip_r = self.rows - y0 - 1;
        let x = BOARD_CORNER.0 + SQUARE_GAP + (SQUARE_SIZE + SQUARE_GAP) * x0 as f32;
        let y = BOARD_CORNER.1 + SQUARE_GAP + (SQUARE_SIZE + SQUARE_GAP) * flip_r as f32;
        (x, y)
    }

    fn center_position_for(&self, index: usize) -> (f32, f32) {
        let pos = self.corner_position_for(index);
        let x = pos.0 + SQUARE_SIZE / 2.0;
        let y = pos.1 + SQUARE_SIZE / 2.0;
        (x, y)
    }

    pub fn move_piece_on_grid(&mut self, id: usize, to_index: usize) {
        let to_position = self.center_position_for(to_index);
        if let Some(piece) = self.pieces.get_mut(&id) {
            piece.animate_move(to_position, Duration::from_secs_f32(0.75));
        }
    }

    fn move_piece_to_reserve(&mut self, player: usize, id: usize, reserve_index: usize, count_index: usize) {
        if let Some(piece) = self.pieces.get_mut(&id) {

            // Get reserve position.
            let reserve_val = self.reserves[player].get(&reserve_index);
            if let Some(reserve) = reserve_val {
                let mut to_position = reserve.position;
                to_position.0 += 15.0 * count_index as f32;
                let mut theta: f32 = 0.0;
                if player == 1 {
                    theta = std::f32::consts::PI
                }
                piece.set_rotation(theta);
                piece.animate_move(to_position, Duration::from_secs_f32(0.75));
            }   
        }
    }

    /// Position the reserve pieces for the player, grouping by PieceKind.
    fn update_reserve_pieces(&mut self, game: &Game, player: usize) {
        // First, get all the piece ids and group them into a vec and store them by kind.
        let mut reserve_hash = HashMap::<PieceKind, Vec<usize>>::new();
        
        for id in game.reserves[player] {
            if id == NONE { continue }
            let new_kind = game.piece_for(id).kind;
            self.update_piece_kind(id, new_kind); // piece may be demoted

            if let Some(id_vec) = reserve_hash.get_mut(&new_kind) {
                id_vec.push(id);
            } else {
                reserve_hash.insert(new_kind, vec![id]);
            }
        }

        // Now, move the pieces into the appropriate spot.
        for (kind, id_vec) in reserve_hash {
            // Could match PieceKind here to specific reserve index. Pawns = 0, etc
            let reserve_index = match kind {
                Gold => 2,
                Silver => 1,
                Pawn => 0,
                _ => 3,
            };
            for (count_index, id) in id_vec.iter().enumerate() {
                self.move_piece_to_reserve(player, *id, reserve_index, count_index);
            }
        }
    }

    pub fn update_with_game(&mut self, game: &Game) {
        // Board move
        for (index, id) in game.grid.iter().enumerate() {
            if *id == NONE { continue }
            self.move_piece_on_grid(*id, index);
            let new_kind = game.piece_for(*id).kind;
            self.update_piece_kind(*id, new_kind);
        }
        // Reserves
        for player in 0..2 {
            self.update_reserve_pieces(game, player);
        }
    }

    fn update_piece_kind(&mut self, id: usize, new_kind: PieceKind) {
        let texture = self.texture_for(new_kind);
        if let Some(sprite) = self.pieces.get_mut(&id) {
            sprite.update_texture(texture);
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

        // Detect piece hits first.
        for (id, piece) in &self.pieces {
            if left_button && piece.contains(mouse_pos) {
                self.message_sender.send(Message::PieceSelected(*id));
                clicked_handled = true;
            }
        }

        if !clicked_handled {
            // Squares
            for (index, square) in &self.squares {
                if left_button && square.contains(mouse_pos) {
                    self.message_sender.send(Message::SquareSelected(*index));
                    clicked_handled = true;
                }
            }
        }
        
        if !clicked_handled {
            // Reserves
            for i in 0..2 {
                for (_index, reserve) in &self.reserves[i] {
                    if left_button && reserve.contains(mouse_pos) {
                        self.message_sender.send(Message::ReserveSelected(i));
                    }
                }
            }
        }
    }

    pub fn update(&mut self, time_delta: Duration) {
        for piece in &mut self.pieces.values_mut() {
            piece.update(time_delta);
        }
    }

    pub fn draw_board(&mut self) {
        clear_background(Color::from_rgba(
            BACKGROUND_COLOR.0, BACKGROUND_COLOR.1, BACKGROUND_COLOR.2, 255));
        // Squares
        for square in &mut self.squares.values_mut() {
            square.draw();
        }
        // Promotion lines
        for line in &mut self.promotion_lines {
            line.draw();
        }
        // Reserves
        for i in 0..2 {
            for reserve in &mut self.reserves[i].values_mut() {
                reserve.draw();
            }
        }
        // Pieces
        for piece in &mut self.pieces.values_mut() {
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

    pub fn set_move_indicies(&mut self, indicies:Vec<usize>) {
        for (index, square) in &mut self.squares {
            square.highlighted = indicies.contains(index);
        }
        self.move_indices = indicies;
    }

    pub fn is_move_index(&self, index: usize) -> bool {
        self.move_indices.contains(&index)
    }

    /// Does what is says on the tin.
    pub fn unhighlight_all_squares(&mut self) {
        for square in &mut self.squares.values_mut() {
            square.highlighted = false;
        }
    }
}