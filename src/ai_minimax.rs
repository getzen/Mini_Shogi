// AI Minimax

use std::time::Instant;

use crate::Action;
use crate::ai::{AIProgress, Think};
use crate::controller::Message;
use crate::Game;
use crate::GameState;
use crate::message_sender::MessageSender;

pub struct AIMinimax {
    game: Game, // a clone of the original
    depth: usize,
    // Due to the recursive nature of Minimax, we need more persistent fields so we're
    // not passing too many arguments to alpha_beta.
    search_player: usize, // need to remember this before things get hairy
    now: Instant, // tracks elapsed time
    progress: AIProgress,
    message_sender: MessageSender,
}

impl Think for AIMinimax {
    fn think(&mut self) -> AIProgress {
        self.now = std::time::Instant::now();

        // Optimization: consider switching to fixed-size array with index tracker.
        let mut pv = Vec::<Action>::new();

        let score = self.alpha_beta(self.depth, true, f64::MIN, f64::MAX, &mut pv);
        self.progress.duration = self.now.elapsed();

        // Use the final version of the pv assembled by alpha_beta.
        self.progress.pv = pv;
        self.progress.score = score;

        print!("score:{}", score);
        for p in &self.progress.pv {
            print!(" ({},{})", p.coord.0, p.coord.1);
        }
        println!();
        self.progress.clone()
    }
}

impl AIMinimax {
    pub fn new(game: Game, depth: usize, message_sender: MessageSender) -> Self {
        let p = game.current_player;
        Self {
            game, depth,
            search_player: p,
            now: std::time::Instant::now(),
            progress: AIProgress::new(),
            message_sender,
        }
    }

    fn alpha_beta(&mut self, depth: usize, maximizing: bool, mut alpha: f64, mut beta: f64, pv: &mut Vec<Action>) -> f64 {
        if *self.game.update_state() != GameState::Ongoing || depth == 0 {
            pv.clear();
            return self.evaluate(self.depth - depth);
        }
        let mut best_score: f64;
        let mut child_pv = Vec::<Action>::new();
        let actions_available = self.game.actions_available();

        // Maximizing
        if maximizing {
            best_score = f64::MIN;
            for action in &actions_available {
                self.game.perform_action(&action, true);
                let child_score = self.alpha_beta(depth-1, false, alpha, beta, &mut child_pv);
                self.game.perform_action(&action.undo(), true);

                self.progress.nodes += 1;

                if child_score > best_score {
                    best_score = child_score;

                    pv.clear();
                    pv.push(action.clone());
                    pv.append(&mut child_pv);

                    self.progress.pv = pv.clone();
                    self.progress.duration = self.now.elapsed();
                    self.message_sender.send(Message::AIUpdate(self.progress.clone()));
                }
                alpha = alpha.max(best_score);
                if beta <= alpha {
                    break;
                }
            }
        }
        // Minimizing
        else {
            best_score = f64::MAX;
            for action in &actions_available {
                self.game.perform_action(&action, true);
                let child_score = self.alpha_beta(depth-1, true, alpha, beta, &mut child_pv);
                self.game.perform_action(&action.undo(), true);

                self.progress.nodes += 1;

                if child_score < best_score {
                    best_score = child_score;

                    pv.clear();
                    pv.push(action.clone());
                    pv.append(&mut child_pv);

                    self.progress.pv = pv.clone();
                    self.progress.duration = self.now.elapsed();
                    self.message_sender.send(Message::AIUpdate(self.progress.clone()));
                }
                beta = beta.min(best_score);
                if beta <= alpha {
                    break;
                }
            }
        }
        best_score
    }

    /// Scores the game from the point of view of search_player.
    /// Depth is used here to make the eval favor winning sooner (low depth) or
    /// losing later (high depth).
    fn evaluate(&self, depth: usize) -> f64 {
        const WIN_LOSS_VAL: f64 = 100.0;
        match self.game.state {
            GameState::Draw => 0.0,
            GameState::WinPlayer0 => {
                if self.search_player == 0 {
                    WIN_LOSS_VAL - depth as f64
                } else {
                    -WIN_LOSS_VAL + depth as f64
                }
            },
            GameState::WinPlayer1 => {
                if self.search_player == 1 {
                    WIN_LOSS_VAL - depth as f64
                } else {
                    -WIN_LOSS_VAL + depth as f64
                }
            },
            GameState::Ongoing => {
                // Customize for each game.
                0.0
            }
        }
    }

}