// Controller
// Handles the app flow and is the intermediary between the view and model.

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use macroquad::prelude::get_frame_time;
use num_format::{Locale, ToFormattedString};

use crate::ai::AI;
use crate::ai::AIProgress;
use crate::message_sender::{Message, MessageSender};
use crate::game::*;
use crate::GameState;
use crate::controller::AppState::*;
use crate::controller::PlayerKind::*;
use crate::view_game::ViewGame;
use crate::view_intro::ViewIntro;

#[derive(PartialEq, Clone, Copy)]
pub enum AppState {
    Intro,
    HumanTurn,
    AITurnBegin,
    AIThinking,
    NextPlayer,
    Player0Won,
    Player1Won,
    Draw,
    Exit,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlayerKind {
    Human,
    AIRandom,
    AIMinimax,
    AIMonteCarlo,
    AIMonteCarloTree,
}

pub struct Controller {
    player_kinds: Vec<PlayerKind>,
    game: Game,
    view_intro: ViewIntro,
    view_game: ViewGame,
    pub state: AppState,
    node_history: Vec<Game>,
    rx: Receiver<Message>,
    tx: Sender<Message>,
    pv_text: String,
}

impl Controller {
    pub async fn new() -> Self {
        // Create message passing transmitter for View and Game to use to communicate
        // with Controller as receiver.
        let (tx, rx) = mpsc::channel();
        Self {
            player_kinds: Vec::new(),
            game: Game::new(),
            view_intro: ViewIntro::new(tx.clone()).await,
            view_game: ViewGame::new(tx.clone(), COLS, ROWS, ).await,
            state: Intro,
            node_history: Vec::new(),
            rx, tx,
            pv_text: String::from(""),
        }
    }

    pub async fn prepare(&mut self) {
        self.player_kinds.push(PlayerKind::Human);
        self.player_kinds.push(PlayerKind::AIMinimax);
        self.game.prepare();
        self.view_intro.prepare();
        self.view_game.prepare().await;

        // Add the game's pieces to the view.
        for piece in &self.game.pieces {
            if let Some(location_id) = piece.location_index {
                let coord = Game::index_to_coord(location_id);
                self.view_game.add_piece(&coord, piece.id, piece.kind, piece.player).await;
            }   
        }
    }

    /// The main control loop.
    pub async fn go(&mut self) {
        loop {
            // Handle events
            match self.state {
                Intro => self.view_intro.handle_events(),
                _ => self.view_game.handle_events(),
            }

            // Check messages
            self.check_messages().await;

            // Take action
            match self.state {
                AITurnBegin => self.begin_ai_turn(),
                NextPlayer => self.next_player(),
                Exit => break,
                _ => {},
            }

            // Update view
            self.view_game.update(Duration::from_secs_f32(get_frame_time()));

            // Draw view
            match self.state {
                Intro => {
                    self.view_intro.draw();
                    self.view_intro.end_frame().await;
                }
                _ => {
                    self.view_game.draw_board();
                    self.view_game.draw_ui(&self.state, &self.pv_text);
                    self.view_game.end_frame().await;
                },
            }
        }
    }

    async fn check_messages(&mut self) {
        let received = self.rx.try_recv();
        if received.is_ok() {
            match received.unwrap() {
                Message::IntroEnded => {
                    self.next_player();
                },
                Message::PieceSelected(piece_id) => {
                    self.piece_selected(piece_id);
                },
                Message::SquareSelected(coord) => {
                    self.square_selected(&coord);
                },
                Message::ReserveSelected(player) => {
                    self.reserve_selected(player);
                },
                Message::AIUpdate(progress) => {
                    if self.state == AIThinking {
                        self.pv_text = self.format_ai_progress(&progress);
                    }
                }
                Message::SearchCompleted(progress) => {
                    let action = progress.pv.first().unwrap();
                    
                    match action.kind {
                        MoveNoCapture => {
                            self.perform_move(action.piece_id, &action.to);
                        },
                        MoveWithCapture => {
                            self.perform_move_with_capture(
                                action.piece_id, 
                                action.captured_id.unwrap(), 
                                &action.to);
                        },
                        FromReserve => {
                            self.perform_move(action.piece_id, &action.to);
                        },
                        ToReserve => {println!("AI ToReserve action?");}
                    }
                    self.pv_text = self.format_ai_progress(&progress);
                    self.state = NextPlayer;
                },
                Message::ShouldExit => self.state = Exit,
            }
        }
    }

    fn piece_selected(&mut self, piece_id: usize) {
        if self.state != HumanTurn { return; }
        // Own piece?
        if self.game.player_for(piece_id) == self.game.current_player {
            // Select it.
            self.view_game.select_piece(piece_id);
            // Highlight move-to squares.
            let mut coords = Vec::new();
            for action in self.game.actions_available() {
                if action.piece_id == piece_id {
                    coords.push(action.to);
                }
            }
            self.view_game.set_move_to_coords(coords);
        } 
        else {
            // Opponent's piece. Is it on a move-to square?
            if let Some(coord) = self.game.coord_for(piece_id) {
                if self.view_game.is_move_to_coord(&coord) {
                    // Capture
                    if let Some(move_id) = self.view_game.selected_piece_id() {
                        self.perform_move_with_capture(move_id, piece_id, &coord);
                        self.state = NextPlayer;
                    }
                }
                //else {
                    // Unselect everything
                    self.view_game.unselect_piece();
                    self.view_game.unhighlight_all_squares();
                //}
            }
        }
    }
  
    // A square with a piece was selected.
    fn square_selected(&mut self, coord: &Coord) {
        if self.state != HumanTurn { return; }

        if self.view_game.is_move_to_coord(&coord) {
            // Move
            if let Some(move_id) = self.view_game.selected_piece_id() {
                self.perform_move(move_id, &coord);
                self.state = NextPlayer;
            }
        }
        //else {
            // Unselect everything
            self.view_game.unselect_piece();
            self.view_game.unhighlight_all_squares();
        //}
    }

    // A reserve square was selected.
    fn reserve_selected(&mut self, player: usize) {
        if self.state != HumanTurn { return; }
        if player != self.game.current_player { return; }
        println!("empty reserve square");
    }

    fn perform_move(&mut self, id: usize, to: &Coord) {
        // View
        self.view_game.move_piece(id, to);
        // Game
        for action in self.game.actions_available() {
            if action.piece_id == id && action.to == *to {
                self.game.perform_action(&action, true);
                self.history.push(action.clone());
                break;
            }
        }
    }

    fn perform_move_with_capture(&mut self, move_id: usize, capture_id: usize, to: &Coord) {
        for action in &self.game.actions_available() {
            if action.piece_id == move_id && action.to == *to {
                // View
                self.view_game.capture_piece(
                    capture_id, 
                    self.game.current_player, 
                    action.reserve_index.unwrap());
                self.view_game.move_piece(move_id, to);
                // Game
                self.game.perform_action(&action, true);
                self.history.push(action.clone());
                break;
            }
        } 
    }

    fn format_ai_progress(&self, progress: &AIProgress) -> String {
        let nodes_string = progress.nodes.to_formatted_string(&Locale::en);
        let mut text = format!("nodes: {}", nodes_string);
        let ms = progress.duration.as_millis();
        let ms_string = progress.duration.as_millis().to_formatted_string(&Locale::en);
        if ms > 0 {
            let nps = (progress.nodes as f64 / ms as f64 * 1_000.0) as usize;
            let nps_string = nps.to_formatted_string(&Locale::en);
            text.push_str(&format!(" / ms: {} = nps: {}", ms_string, nps_string));
        } else {
            text.push_str(" / ms: 0 = nps: --");
        }
        let score_string = (progress.score as isize).to_formatted_string(&Locale::en);
        text.push_str(&format!(". score: {}", score_string));
        text.push_str(". pv: ");
        for i in 0..progress.pv.len() {
            let coord = &progress.pv[i].to;
            text.push_str(&format!("{},{}", coord.0, coord.1));
            if i < progress.pv.len() - 1 {
                text.push_str(", ");
            }
        }
        text
    }

    fn next_player(&mut self) {
        match self.game.update_state() {
            GameState::Draw => {
                self.state = Draw;
            },
            GameState::WinPlayer0 => {
                self.state = Player0Won;
            },
            GameState::WinPlayer1 => {
                self.state = Player1Won;
            },
            _ => {
                let p = self.game.current_player;
                if self.player_kinds[p] == Human {
                    self.state = HumanTurn;
                } else {
                    self.state = AITurnBegin;
                    self.pv_text = String::new();
                }
            },
        }
    }

    fn begin_ai_turn(&mut self) {
        self.state = AIThinking;

        // These variables are captured by the thread.
        let ai_kind = self.player_kinds[self.game.current_player];
        let game_clone = self.game.clone();
        let message_sender = MessageSender::new(self.tx.clone(), None);

        std::thread::spawn(move || {
            AI::think(ai_kind, game_clone, message_sender);
        });
    }
}