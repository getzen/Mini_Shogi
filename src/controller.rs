// Controller
// Handles the app flow and is the intermediary between the view and model.

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use macroquad::prelude::*;
use num_format::{Locale, ToFormattedString};

use crate::ai::{AI, AIProgress};
use crate::ai_sender::{AIMessage, AISender};
use crate::asset_loader::AssetLoader;
use crate::game::*;
use crate::game::{Game, GameState};
use crate::controller::AppState::*;
use crate::controller::PlayerKind::*;
use crate::view::button::Button;
use crate::view::view_game::{ViewGame, ViewGameMessage};
use crate::view::view_intro::ViewIntro;
use crate::view::view_settings::{ViewSettings, ViewSettingsMessage};
use crate::view::view_rules::ViewRules;
use crate::view::view_rules::ViewRulesMessage;
use crate::view::button_bar::ButtonBar;

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
    Settings,
    Rules,
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
    AI,
}

// The button bar at top
const BAR_ABOUT_ID: usize = 0;
const BAR_RULES_ID: usize = 1;
const BAR_SETTINGS_ID: usize = 2;
const BAR_QUIT_ID: usize = 3;

pub struct Controller {
    players: Vec<Player>,
    game: Game,

    button_bar: ButtonBar, // the command bar at top

    view_intro: ViewIntro,
    view_settings: ViewSettings,
    view_rules: ViewRules,
    view_game: ViewGame,
    pub state: AppState,
    previous_state: Option<AppState>,
    node_history: Vec<Game>,
    view_intro_rx: Receiver<ViewSettingsMessage>,
    view_rules_rx: Receiver<ViewRulesMessage>,
    view_game_rx: Receiver<ViewGameMessage>,
    ai_tx: Sender<AIMessage>,
    ai_rx: Receiver<AIMessage>,
    pv_text: String,
}

impl Controller {
    pub async fn new() -> Self {
        let (view_intro_tx, view_intro_rx) = mpsc::channel();
        let (view_rules_tx, view_rules_rx) = mpsc::channel();
        let (view_game_tx, view_game_rx) = mpsc::channel();
        let (ai_tx, ai_rx) = mpsc::channel();

        Self {
            players: Vec::new(),
            game: Game::new(),
            button_bar: ButtonBar::new((0., 0.), false),
            view_settings: ViewSettings::new(view_intro_tx).await,
            view_rules: ViewRules::new(view_rules_tx).await,
            previous_state: None,
            view_intro: ViewIntro::new().await,
            view_game: ViewGame::new(view_game_tx, COLS, ROWS).await,
            state: NextPlayer,
            node_history: Vec::new(),
            view_intro_rx,
            view_rules_rx,
            view_game_rx,
            ai_tx, ai_rx,
            pv_text: String::from(""),
        }
    }

    pub async fn prepare(&mut self) {
        // Construct ButtonBar (menu bar)
        let mut texture = AssetLoader::get_texture("bar_about");
        let mut button = Button::new((0.,0.), texture, Some(BAR_ABOUT_ID));
        self.button_bar.add_button(button);

        texture = AssetLoader::get_texture("bar_rules");
        button = Button::new((0.,0.), texture, Some(BAR_RULES_ID));
        self.button_bar.add_button(button);

        texture = AssetLoader::get_texture("bar_settings");
        button = Button::new((0.,0.), texture, Some(BAR_SETTINGS_ID));
        self.button_bar.add_button(button);

        texture = AssetLoader::get_texture("bar_quit");
        button = Button::new((0.,0.), texture, Some(BAR_QUIT_ID));
        self.button_bar.add_button(button);

        // Must do after buttons are added.
        self.button_bar.set_color(Color::from_rgba(125, 125, 125, 125));
        self.button_bar.set_selected_color(Color::from_rgba(246, 194, 81, 125));

        self.players.push( Player {id: 0, kind: Human, search_depth: 3, search_rounds: 500} );
        self.players.push( Player {id: 1, kind: AI, search_depth: 3, search_rounds: 500} );
        self.game.prepare();
        self.view_settings.prepare(self.players.clone());
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

            // Check own events first.
            if let Some(button_id) = self.button_bar.process_events() {
                match button_id {
                    BAR_ABOUT_ID => {},
                    BAR_RULES_ID => self.state = Rules,
                    BAR_SETTINGS_ID => self.state = Settings,
                    BAR_QUIT_ID => self.state = Exit,
                    _ => panic!(),
                }
            }
            // View events
            match self.state {
                Intro => {
                },
                Settings => {
                    self.view_settings.process_events();
                    self.check_messages().await;
                },
                Rules => {
                    self.view_rules.process_events();
                    self.check_messages().await;
                }
                HumanTurn | AIThinking | WaitingOnAnimation | Player0Won | Player1Won | Draw => {
                    self.view_game.process_events();
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
            let time_delta = Duration::from_secs_f32(get_frame_time());
            if self.view_intro.visible {
                self.view_intro.update(time_delta);
            }
            if self.state == WaitingOnAnimation {
                let active = self.view_game.update(time_delta);
                if !active && self.state == WaitingOnAnimation {
                    self.state = NextPlayer;
                }
            }
            // Drawing
            self.view_game.draw_board();
            self.view_game.draw_ui(&self.state, &self.pv_text);
            match self.state {
                Settings => {
                     self.view_settings.draw();
                },
                Rules => {
                    self.view_rules.draw();
                }
                HumanTurn | AIThinking | Player0Won | Player1Won | Draw | WaitingOnAnimation => {
                    
                },
                _ => {},
            }
            // ButtonBar (menu)
            self.button_bar.draw();

            // Intro draws last since it is on top of other views.
            if self.view_intro.visible {
                self.view_intro.draw();
            }

            // Call next_frame for non-transitional states.
            match self.state {
                NextPlayer | AITurnBegin => {},
                _ => next_frame().await,
            }
        }
    }

    async fn check_messages(&mut self) {
        // From ViewIntro
        let received = self.view_intro_rx.try_recv();
        if received.is_ok() {
            match received.unwrap() {
                ViewSettingsMessage::ShouldStart(players) => {
                    //dbg!(players[1].search_depth);
                    self.players = players;
                    self.next_player();
                },
            }
        }

        // From ViewRules
        let received = self.view_rules_rx.try_recv();
        if received.is_ok() {
            match received.unwrap() {
                ViewRulesMessage::ShouldClose => {
                    self.state = self.previous_state.unwrap();
                },
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