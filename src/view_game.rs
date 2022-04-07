// View
// Responsible for drawing and polling for events.

use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::time::Duration;

use macroquad::audio::{Sound, play_sound_once};
use macroquad::prelude::*;

use crate::asset_loader::AssetLoader;
use crate::Game;
use crate::game::NONE;
use crate::controller::AppState;
use crate::controller::AppState::*;
use crate::message_sender::{Message, MessageSender};
use crate::Piece;
use crate::piece::PieceKind::{self, *};
use crate::sprite::*;
use crate::text::Text;

const BACKGROUND_COLOR: (u8, u8, u8) = (144, 144, 137);
const BOARD_CORNER: (f32, f32) = (165.0, 95.0);
const SQUARE_SIZE: f32 = 90.0; // matches the square.png size
const SQUARE_GAP: f32 = 5.0;
const PROMO_LINE_TOP: (f32, f32) = (405., 287.);
const PROMO_LINE_BOTTOM: (f32, f32) = (405., 477.);
const RESERVE_0_CENTER: (f32, f32) = (715., 615.);
const RESERVE_1_CENTER: (f32, f32) = (95., 140.);
const RESERVE_BOX_OFFSET: f32 = 20.;
const RESERVE_PIECE_OFFSET: f32 = 12.;
const TEXT_STATUS_CENTER: (f32, f32) = (400., 60.0);
const AI_PROGRESS_CORNER: (f32, f32) = (20., 770.);
const PIECE_SIZE: (f32, f32) = (70., 75.);
const MOVE_DURATION: f32 = 0.25;

pub struct ViewGame {
    message_sender: MessageSender, // sends event messages to controller
    columns: usize,
    rows: usize,
    squares: HashMap<usize, Sprite>, // key: location index
    promotion_lines: Vec<Sprite>,
    reserve_boxes: Vec<HashMap<usize, Sprite>>, // *************** why is this a hash map?
    pieces: Vec<Sprite>, // a vec so it can be sorted by z_order
    pub selected_piece: Option<usize>,
    pub move_indices: Vec<usize>, // all the spots the currently selected piece can move to
    status_text: Text,
    ai_progress_text: Text,
    piece_move: Sound,
    piece_capture: Sound,
}

impl ViewGame {
    pub async fn new(tx: Sender<Message>, columns: usize, rows: usize) -> Self {
        Self {
            message_sender: MessageSender::new(tx, None),
            columns, rows,
            squares: HashMap::new(),
            promotion_lines: Vec::new(),
            reserve_boxes: vec!(HashMap::new(), HashMap::new()),
            pieces: Vec::new(),
            selected_piece: None,
            move_indices: Vec::new(),
            status_text: Text::new(
                TEXT_STATUS_CENTER,
                "Welcome!".to_owned(),
                18,
                Some("Menlo"),
            ).await,
            ai_progress_text: Text::new(
                AI_PROGRESS_CORNER,
                "".to_owned(), 
                12,
                Some("Menlo"),
            ).await,
            piece_move: AssetLoader::get_sound("piece_move").await,
            piece_capture: AssetLoader::get_sound("piece_capture").await,
        }
    }

    pub async fn prepare(&mut self) {
        self.status_text.centered = true;

        // Board
        let mut texture = AssetLoader::get_texture("square");
        for c in 0..self.columns {
            for r in 0..self.rows {
                let index = Game::column_row_to_index(c, r);
                let position = self.center_position_for(index);
                let mut square = Sprite::new(position, texture);
                square.alt_color = Some(LIGHTGRAY);
                self.squares.insert(index, square);
            }
        }

        // Promotion lines
        texture = AssetLoader::get_texture("line");
        let line_top = Sprite::new(PROMO_LINE_TOP, texture);
        self.promotion_lines.push(line_top);
        let line_bottom = Sprite::new(PROMO_LINE_BOTTOM, texture);
        self.promotion_lines.push(line_bottom);

        // Reserves
        texture = AssetLoader::get_texture("reserve");
        for i in 0..4 {
            // Reserve, player 0
            let mut pos_x = RESERVE_0_CENTER.0;
            let mut pos_y = RESERVE_0_CENTER.1 - i as f32 * (SQUARE_SIZE + RESERVE_BOX_OFFSET); 
            let mut reserve = Sprite::new((pos_x, pos_y), texture);
            self.reserve_boxes[0].insert(i, reserve);
            // Reserve, player 1
            pos_x = RESERVE_1_CENTER.0;
            pos_y = RESERVE_1_CENTER.1 + i as f32 * (SQUARE_SIZE + RESERVE_BOX_OFFSET);
            reserve = Sprite::new((pos_x, pos_y), texture);
            self.reserve_boxes[1].insert(i, reserve);
        }
    }

    fn texture_for(&self, piece_kind: PieceKind) -> Texture2D {
        match piece_kind {
            King => AssetLoader::get_texture("king"),
            Gold => AssetLoader::get_texture("gold"),
            Silver => AssetLoader::get_texture("silver"),
            SilverPro => AssetLoader::get_texture("silver_pro"),
            Pawn => AssetLoader::get_texture("pawn"),
            PawnPro => AssetLoader::get_texture("pawn_pro"),
        }
    }

    fn piece_for_id(&mut self, id: usize) -> Option<&mut Sprite> {
        self.pieces.iter_mut().find(|p| p.id == Some(id))
    }

    pub fn add_piece(&mut self, piece: &Piece) {
        let texture = self.texture_for(piece.kind);
        let position = self.center_position_for(piece.location_index);
        let mut sprite = Sprite::new(position, texture);
        sprite.set_size(Some(PIECE_SIZE));
        if piece.player == 1 {
            sprite.rotation = std::f32::consts::PI;
        }
        sprite.alt_color = Some(LIGHTGRAY);
        sprite.id = Some(piece.id);
        self.pieces.insert(piece.id, sprite);
    }

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
        if let Some(piece) = self.piece_for_id(id) {
            if to_position != piece.position {
                piece.animate_move(to_position, Duration::from_secs_f32(MOVE_DURATION));
                play_sound_once(self.piece_move);
            }
        }
    }

    fn move_piece_to_reserve(&mut self, player: usize, id: usize, reserve_index: usize, count_index: usize) {
        let reserve_pos = self.reserve_boxes[player].get(&reserve_index).unwrap().position;
        if let Some(piece) = self.piece_for_id(id) {
                let mut to_position = reserve_pos;
                if player == 0 {
                    to_position.1 -= RESERVE_PIECE_OFFSET * count_index as f32;
                } else {
                    to_position.1 += RESERVE_PIECE_OFFSET * count_index as f32;
                }
                let mut theta: f32 = 0.0;
                if player == 1 {
                    theta = std::f32::consts::PI
                }
                piece.rotation = theta;
                if to_position != piece.position {
                    piece.animate_move(to_position, Duration::from_secs_f32(MOVE_DURATION));
                    play_sound_once(self.piece_capture);
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
                if let Some(piece) = self.piece_for_id(*id) {
                    piece.z_order = count_index; // position on top of previous pieces
                }
                self.move_piece_to_reserve(player, *id, reserve_index, count_index);
            }
        }
        // Sort by z_order so the overlap is correct.
        self.pieces.sort_by(|a, b| a.z_order.cmp(&b.z_order));
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
        if let Some(sprite) = self.piece_for_id(id) {
            sprite.texture = texture;
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
        for piece in &self.pieces {
        //for (id, piece) in &self.pieces {
            if left_button && piece.contains(mouse_pos) {
                self.message_sender.send(Message::PieceSelected(piece.id.unwrap()));
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
                for (_index, reserve) in &self.reserve_boxes[i] {
                    if left_button && reserve.contains(mouse_pos) {
                        self.message_sender.send(Message::ReserveSelected(i));
                    }
                }
            }
        }
    }

    pub fn update(&mut self, time_delta: Duration) -> bool {
        let mut update_active = false;
        for piece in &mut self.pieces {
            let updated = piece.update(time_delta);
            if updated {
                update_active = true;
            }
        }
        update_active
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

        // Reserve boxes - uncomment for debugging
        // for i in 0..2 {
        //     for reserve in &mut self.reserve_boxes[i].values_mut() {
        //         reserve.draw();
        //     }
        // }
        
        // Pieces
        for piece in &mut self.pieces {
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
            _ => {""},
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
        self.selected_piece
    }

    pub fn select_piece(&mut self, id: usize) {
        if let Some(old_id) = self.selected_piece {
            if old_id == id {
                return; // piece is already selected
            }
            self.unselect_piece();
        }
        if let Some(piece) = self.piece_for_id(id) {
            //piece.highlighted = true;
            piece.use_alt_color = true;
            self.selected_piece = Some(id);
        }
    }

    pub fn unselect_piece(&mut self) {
        if let Some(id) = self.selected_piece {
            if let Some(piece) = self.piece_for_id(id) {
                //piece.highlighted = false;
                piece.use_alt_color = false;
            }
            self.selected_piece = None;
        }
    }

    pub fn set_move_indicies(&mut self, indicies:Vec<usize>) {
        for (index, square) in &mut self.squares {
            //square.highlighted = indicies.contains(index);
            square.use_alt_color = indicies.contains(index);
        }
        self.move_indices = indicies;
    }

    pub fn is_move_index(&self, index: usize) -> bool {
        self.move_indices.contains(&index)
    }

    /// Does what is says on the tin.
    pub fn unhighlight_all_squares(&mut self) {
        for square in &mut self.squares.values_mut() {
            //square.highlighted = false;
            square.use_alt_color = false;
        }
    }
}