// Controller
// Handles the app flow and is the intermediary between the view and model.

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use macroquad::prelude::get_frame_time;
use num_format::{Locale, ToFormattedString};

use crate::ai::{AI, AIProgress};
use crate::ai_sender::{AIMessage, AISender};
use crate::game::*;
use crate::game::{Game, GameState};
use crate::controller::AppState::*;
use crate::controller::PlayerKind::*;
use crate::view_game::{ViewGame, ViewGameMessage};
use crate::view_intro::{ViewIntro, ViewIntroMessage};

#[derive(Clone, Copy)]
pub struct Player {
    pub id: usize,
    pub kind: PlayerKind,
    pub search_depth: usize,
    pub search_rounds: usize,
}

#[derive(PartialEq, Clone, Copy)]
pub enum AppState {
    Intro,
    HumanTurn,
    AITurnBegin,
    AIThinking,
    WaitingOnAnimation,
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
    players: Vec<Player>,
    game: Game,
    view_intro: ViewIntro,
    view_game: ViewGame,
    pub state: AppState,
    node_history: Vec<Game>,
    view_intro_rx: Receiver<ViewIntroMessage>,
    view_game_rx: Receiver<ViewGameMessage>,
    ai_tx: Sender<AIMessage>,
    ai_rx: Receiver<AIMessage>,
    pv_text: String,
}

impl Controller {
    pub async fn new() -> Self {
        let (view_intro_tx, view_intro_rx) = mpsc::channel();
        let (view_game_tx, view_game_rx) = mpsc::channel();
        let (ai_tx, ai_rx) = mpsc::channel();

        Self {
            players: Vec::new(), // supplied by view_intro
            game: Game::new(),
            view_intro: ViewIntro::new(view_intro_tx).await,
            view_game: ViewGame::new(view_game_tx, COLS, ROWS).await,
            state: Intro,
            node_history: Vec::new(),
            view_intro_rx,
            view_game_rx,
            ai_tx, ai_rx,
            pv_text: String::from(""),
        }
    }

    pub async fn prepare(&mut self) {
        self.game.prepare();
        self.view_intro.prepare();
        self.view_game.prepare().await;

        // Add the game's pieces to the view.
        for piece in &self.game.pieces {
            self.view_game.add_piece(piece); 
        }
    }

    /// The main control loop.
    pub async fn go(&mut self) {
        loop {
            // Event and state management
            match self.state {
                Intro => {
                    self.view_intro.handle_events();
                    self.view_intro.check_messages();
                    self.check_messages().await;
                },
                HumanTurn | AIThinking | WaitingOnAnimation | Player0Won | Player1Won | Draw => {
                    self.view_game.handle_events();
                    self.check_messages().await;
                },
                AITurnBegin => {
                    self.begin_ai_turn();
                },
                NextPlayer => {
                    self.next_player();
                },
                Exit => {
                    break;
                },
            }
            // Animation updates
            match  self.state {
                WaitingOnAnimation => {
                    let time_delta = Duration::from_secs_f32(get_frame_time());
                    let active = self.view_game.update(time_delta);
                    if !active && self.state == WaitingOnAnimation {
                        self.state = NextPlayer;
                    }
                },
                _ => {},
            }
            // Drawing
            match self.state {
                Intro => {
                     self.view_intro.draw();
                     self.view_intro.end_frame().await;
                },
                HumanTurn | AIThinking | Player0Won | Player1Won | Draw | WaitingOnAnimation => {
                    self.view_game.draw_board();
                    self.view_game.draw_ui(&self.state, &self.pv_text);
                    self.view_game.end_frame().await;
                },
                _ => {},
            }
        }
    }

    async fn check_messages(&mut self) {
        // From ViewIntro
        let received = self.view_intro_rx.try_recv();
        if received.is_ok() {
            match received.unwrap() {
                ViewIntroMessage::ShouldStart(players) => {
                    //dbg!(players[1].search_depth);
                    self.players = players;
                    self.next_player();
                },
                ViewIntroMessage::ShouldExit => self.state = Exit,
            }
        }
        // From ViewGame
        let received = self.view_game_rx.try_recv();
        if received.is_ok() {
            match received.unwrap() {
                ViewGameMessage::PieceSelected(id) => {
                    self.piece_selected(id);
                },
                ViewGameMessage::SquareSelected(index) => {
                    self.square_selected(index);
                },
                ViewGameMessage::ReserveSelected(player) => {
                    self.reserve_selected(player);
                },
                ViewGameMessage::ShouldExit => {
                    self.state = Exit;
                },
            }
        }
        // From AI
        let received = self.ai_rx.try_recv();
        if received.is_ok() {
            match received.unwrap() {
                AIMessage::AIUpdate(progress) => {
                    //if self.state == AIThinking {
                        self.pv_text = self.format_ai_progress(&progress);
                    //}
                }
                AIMessage::SearchCompleted(progress) => {
                    let node = progress.best_node.unwrap();
                    self.use_node(node);
                    self.pv_text = self.format_ai_progress(&progress);
                    self.state = WaitingOnAnimation;
                },
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
                self.state = WaitingOnAnimation;
            } 
            // Unselect everything
            self.view_game.unselect_piece();
            self.view_game.unhighlight_all_squares();
        }
    }
  
    // A square with a piece was selected.
    fn square_selected(&mut self, index: usize) {
        if self.state != HumanTurn { return; }

        if self.view_game.is_move_index(index) {
            // Move
            if let Some(piece_id) = self.view_game.selected_piece_id() {
                self.perform_move(piece_id, index);
                self.state = WaitingOnAnimation;
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
        self.node_history.push(node);
        self.game = node;
    }

    fn format_ai_progress(&self, progress: &AIProgress) -> String {
        // let nodes_string = progress.nodes.to_formatted_string(&Locale::en);
        // let mut text = format!("nodes: {}", nodes_string);
        let percent_string = (progress.percent_complete * 100.0) as usize;
        let mut text = format!("{}%", percent_string);

        let ms = progress.duration.as_millis();
        let ms_string = progress.duration.as_millis().to_formatted_string(&Locale::en);
        if ms > 0 {
            let nps = (progress.nodes as f64 / ms as f64 * 1_000.0) as usize;
            let nps_string = nps.to_formatted_string(&Locale::en);
            text.push_str(&format!(" | ms: {} | nps: {}", ms_string, nps_string));
        } else {
            text.push_str(" | ms: 0 | nps: --");
        }
        let score_string = (progress.score as isize).to_formatted_string(&Locale::en);
        text.push_str(&format!(" | score: {}", score_string));
        text.push_str(" | pv: ");
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
                if self.players[p].kind == Human {
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
        let player = self.players[self.game.current_player];
        let game_copy = self.game;
        let message_sender = AISender::new(self.ai_tx.clone(), None);

        std::thread::spawn(move || {
            AI::think(player, game_copy, message_sender);
        });
    }
}