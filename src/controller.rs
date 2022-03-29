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
            self.view_game.add_piece(piece).await; 
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
                Message::PieceSelected(id) => {
                    self.piece_selected(id);
                },
                Message::SquareSelected(index) => {
                    self.square_selected(index);
                },
                Message::ReserveSelected(player) => {
                    self.reserve_selected(player);
                },
                Message::AIUpdate(progress) => {
                    //if self.state == AIThinking {
                        self.pv_text = self.format_ai_progress(&progress);
                    //}
                }
                Message::SearchCompleted(progress) => {
                    let node = progress.best_node.unwrap();
                    self.use_node(node);
                    self.pv_text = self.format_ai_progress(&progress);
                    self.state = NextPlayer;
                },
                Message::ShouldExit => self.state = Exit,
            }
        }
    }

    fn piece_selected(&mut self, id: usize) {
        if self.state != HumanTurn { return; }
        // Own piece?
        if self.game.player_for_piece_id(id) == self.game.current_player {
            // Select it.
            self.view_game.select_piece(id);
            // Highlight move-to squares.
            let move_indices = self.game.move_indices_for_piece(id);
            self.view_game.set_move_indicies(move_indices);
        } else {
            // Opponent's piece. Is it on a move-to square?
            let location_index = self.game.location_index_for(id);
            if self.view_game.is_move_index(location_index) {
                // Capture
                if let Some(piece_id) = self.view_game.selected_piece_id() {
                    self.perform_move(piece_id, location_index);
                }
                
                self.state = NextPlayer;
            } 
            // Unselect everything
            self.view_game.unselect_piece();
            self.view_game.unhighlight_all_squares();
            // else {
            //     // Unselect everything
            //     self.view_game.unselect_piece();
            //     self.view_game.unhighlight_all_squares();
            // }
        }
    }
  
    // A square with a piece was selected.
    fn square_selected(&mut self, index: usize) {
        if self.state != HumanTurn { return; }

        if self.view_game.is_move_index(index) {
            // Move
            if let Some(piece_id) = self.view_game.selected_piece_id() {
                self.perform_move(piece_id, index);
                self.state = NextPlayer;
            }
        }
        // Regardless, unselect everything.
        self.view_game.unselect_piece();
        self.view_game.unhighlight_all_squares();
    }

    // A reserve square was selected.
    fn reserve_selected(&mut self, player: usize) {
        if self.state != HumanTurn { return; }
        if player != self.game.current_player { return; }
        println!("empty reserve square");
    }

    /// Find the child node matching the piece id
    fn find_node(&mut self, id: usize, location_index: usize) -> Option<Game> {
        let nodes = self.game.child_nodes(self.game.current_player);
        for node in nodes {
            let piece = node.piece_for(id);
            if piece.location_index == location_index {
                return Some(node);
            }
        }
        None
    }

    fn perform_move(&mut self, id: usize, location_index: usize) {
        let node_option = self.find_node(id, location_index);
        match node_option {
            Some(node) => {
                self.use_node(node); 
            },
            None => panic!("Cannot find node in perform_move!")
        }
    }

    fn use_node(&mut self, node: Game) {
        self.view_game.update_with_game(&node);
        self.node_history.push(node.clone());
        self.game = node;
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
            let the_move = progress.pv[i];
            let piece = self.game.piece_for(the_move.0);
            let piece_str = piece.string_rep();
            let cap_str = match the_move.2 {
                true => "x",
                false => "",
            };
            text.push_str(&format!("{}{}{}", piece_str, cap_str, the_move.1));
            if i < progress.pv.len() - 1 {
                text.push_str(", ");
            }
        }
        text
    }

    fn next_player(&mut self) {
        //self.game.debug();
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