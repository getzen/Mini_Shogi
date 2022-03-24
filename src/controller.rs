// Controller
// Handles the app flow and is the intermediary between the view and model.

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use macroquad::prelude::get_frame_time;
use num_format::{Locale, ToFormattedString};

use crate::Action;
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
    action_history: Vec<Action>, // for display and player-controlled reversing, not used by AI
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
            action_history: Vec::new(),
            rx, tx,
            pv_text: String::from(""),
        }
    }

    pub async fn prepare(&mut self) {
        self.player_kinds.push(PlayerKind::Human);
        self.player_kinds.push(PlayerKind::Human);
        self.game.prepare();
        self.view_intro.prepare();
        self.view_game.prepare().await;

        // Add the game's pieces to the view.
        for (index, piece_id) in self.game.grid.iter().enumerate() {
            if *piece_id == NONE { continue; }
            let piece = self.game.pieces[*piece_id];
            let coord = Game::index_to_coord(index);
            self.view_game.add_piece_to(&coord, piece.kind, piece.player).await;
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
                Message::IntroEnded => { self.next_player(); },
                Message::SquareSelected(coord) => {
                    self.square_selected(&coord);
                },
                Message::ReserveSelected(coord) => {
                    self.reserve_selected(&coord);
                },
                Message::AIUpdate(progress) => {
                    if self.state == AIThinking {
                        self.pv_text = self.format_ai_progress(&progress);
                    }
                }
                Message::SearchCompleted(progress) => {
                    let action = progress.pv.first().unwrap();
                    
                    //self.view_game.add_piece_to(&action.coord, action.piece_kind, self.game.current_player).await;
                    self.game.perform_action(action, true);
                    self.action_history.push(action.clone());
                    self.pv_text = self.format_ai_progress(&progress);

                    self.state = NextPlayer;
                },
                Message::ShouldExit => self.state = Exit,
            }
        }
    }

    fn reserve_selected(&mut self, piece_id: usize) {
        
    }

    // A square with a piece was selected.
    fn square_selected(&mut self, coord: &Coord) {
        // Ignore if not human turn.
        if self.state != HumanTurn { return; }

        // If piece selected is some.
        if let Some(from) = self.view_game.selected_piece_coord() {
            println!("piece is selected");
            // If square is move-to.
            if self.view_game.is_move_to_coord(coord) {
                println!("square is move-to");
                // If square has opponent.
                if self.game.is_player_at(1 - self.game.current_player, coord) {
                    // Capture
                    println!("capture");
                    self.perform_capture(&from, coord);
                }
                else {
                    // Move
                    println!("move");
                    self.perform_move(&from, coord);
                }
                self.view_game.unselect_piece();
                self.view_game.unhighlight_all_squares();
            }
            else {
                // Not a move-to square.
                println!("square is NOT move-to");
                self.view_game.unselect_piece();
                self.view_game.unhighlight_all_squares();

                // Is it another player piece?
                if self.game.is_player_at(self.game.current_player, coord) {
                    // Select piece and move-to's
                    self.view_game.select_piece(coord);
    
                    let mut coords = Vec::new();
                    for action in self.game.actions_available() {
                        if action.from == Some(*coord) {
                            coords.push(action.to);
                        }
                    }
                    self.view_game.set_move_to_coords(coords);
                    return;
                }
            }
        }
        else { // No piece selected.
            // If player piece at coord.
            if self.game.is_player_at(self.game.current_player, coord) {
                // Select piece and move-to's
                self.view_game.select_piece(coord);

                let mut coords = Vec::new();
                for action in self.game.actions_available() {
                    if action.from == Some(*coord) {
                        coords.push(action.to);
                    }
                }
                self.view_game.set_move_to_coords(coords);
                return;
            }
            else { // No piece selected and no player piece at coord.
                return;
            }
        }
    }

    fn perform_move(&mut self, from: &Coord, to: &Coord) {
        // view
        self.view_game.move_piece(from, to);
        // game
        for action in self.game.actions_available() {
            if let Some(action_from) = action.from {
                if action_from == *from && action.to == *to {
                    self.game.perform_action(&action, true);
                    self.action_history.push(action.clone());
                    break;
                }
            }
        }
    }

    fn perform_capture(&mut self, from: &Coord, to: &Coord) {
        // view
        self.view_game.capture_piece(&to, self.game.current_player);
        self.view_game.move_piece(from, to);

        // game
        for action in self.game.actions_available() {
            if let Some(action_from) = action.from {
                if action_from == *from && action.to == *to {
                    self.game.perform_action(&action, true);
                    self.action_history.push(action.clone());
                    break;
                }
            }
        }
                    //self.game.perform_action(&action, true);
                    //self.action_history.push(action.clone());
    }

    // // An empty square was selected.
    // fn square_selected(&mut self, square_coord: &Coord) {
    //     // Ignore if not human turn.
    //     if self.state != HumanTurn { return; }
    //     // Return if no piece is selected.
    //     let piece_coord = self.view_game.selected_piece_coord();
    //     if piece_coord.is_none() {
    //         println!("no piece_coord!");
    //         return; 
    //     }

    //     // Find the matching action.
    //     for action in &self.game.actions_available() {
    //         let to = action.to;
    //         if to != *square_coord {
    //             println!("no 'to' coord!");
    //             continue;
    //         }

    //         match action.kind {
    //             MoveNoCapture => {
    //                 println!("move no capture");
    //                 if action.from.is_none() { continue; }
    //                 let from = action.from.unwrap();
    //                 if from != *square_coord { continue; }
    //                 self.view_game.move_piece(&from, &to)
    //             }
    //             MoveWithCapture => {
    //                 println!("move with capture!");
    //                 if action.from.is_none() { continue; }
    //                 let from = action.from.unwrap();
    //                 if from != *square_coord { continue; }
    //                 self.view_game.move_piece(&from, &to);
    //             }
    //             _ => {
    //                 println!("Other action!");
    //             }
    //         }

    //         // Highlight the 'from' and 'to' squares to show the move.
    //         let from = action.from.unwrap();
    //         self.view_game.highlight_squares(vec![from, to]);
    //         //self.view_game.toggle_piece_highlighting(&from);

    //         println!("finding action");

    //         self.game.perform_action(&action, true);
    //         self.action_history.push(action.clone());

    //     }
    // }

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

    async fn coord_selected_OLD(&mut self, coord: Coord) {
        if self.state == HumanTurn {
            let actions = self.game.actions_available();
            let mut some_action: Vec<&Action> = actions.iter().filter(|a| a.to == coord).collect();
            let action = some_action.swap_remove(0);

            //self.view_game.add_piece_to(&coord, action.to, self.game.current_player).await;

            self.game.perform_action(action, true);
            self.action_history.push(action.clone());
        }
    }
}