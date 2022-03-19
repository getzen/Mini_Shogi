// Controller
// Handles the app flow and is the intermediary between the view and model.

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use num_format::{Locale, ToFormattedString};

use crate::Action;
use crate::ai::AI;
use crate::ai::AIProgress;
use crate::message_sender::MessageSender;
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

// Messages sent by the view or the AI to this controller.
pub enum Message {
    IntroEnded,
    CoordSelected(Coord),
    AIUpdate(AIProgress),
    SearchCompleted(AIProgress),
    ShouldExit,
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
                Message::CoordSelected(coord) => {
                    self.coord_selected(coord).await;
                    self.state = NextPlayer;
                },
                Message::AIUpdate(progress) => {
                    if self.state == AIThinking {
                        self.pv_text = self.format_ai_progress(&progress);
                    }
                }
                Message::SearchCompleted(progress) => {
                    let action = progress.pv.first().unwrap();
                    self.view_game.add_piece_to(&action.coord, action.piece_kind, self.game.current_player).await;
                    self.game.perform_action(action, true);
                    self.action_history.push(action.clone());
                    self.pv_text = self.format_ai_progress(&progress);

                    self.state = NextPlayer;
                },
                Message::ShouldExit => self.state = Exit,
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
            let coord = &progress.pv[i].coord;
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

    async fn coord_selected(&mut self, coord: Coord) {
        if self.state == HumanTurn {
            let actions = self.game.actions_available();
            let mut some_action: Vec<&Action> = actions.iter().filter(|a| a.coord == coord).collect();
            let action = some_action.swap_remove(0);

            self.view_game.add_piece_to(&coord, action.piece_kind, self.game.current_player).await;

            self.game.perform_action(action, true);
            self.action_history.push(action.clone());
        }
    }
}