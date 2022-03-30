// AI Minimax

use std::time::Instant;

use crate::ai::{AIProgress, Think};
use crate::Game;
use crate::GameState;
use crate::game::Move;
use crate::message_sender::{Message, MessageSender};

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
        let mut pv = Vec::new();

        let score = self.alpha_beta(self.game, self.depth, true, f64::MIN, f64::MAX, &mut pv);
        self.progress.duration = self.now.elapsed();

        // Use the final version of the pv assembled by alpha_beta.
        self.progress.pv = pv;
        self.progress.score = score;
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

    fn alpha_beta(&mut self, mut node: Game, depth: usize, maximizing: bool, mut alpha: f64, mut beta: f64, pv: &mut Vec<Move>) -> f64 {
        if *node.update_state() != GameState::Ongoing || depth == 0 {
            pv.clear();
            return self.evaluate(&node, self.depth - depth);
        }
        
        let mut child_pv = Vec::new();
        let child_nodes = node.child_nodes(node.current_player);

        // Maximizing
        if maximizing {
            for node in &child_nodes {
                let child_score = self.alpha_beta(*node, depth-1, false, alpha, beta, &mut child_pv);
                self.progress.nodes += 1;

                if child_score > alpha {
                    alpha = child_score;
                    pv.clear();
                    pv.push(node.last_move.unwrap());
                    pv.append(&mut child_pv);

                    self.progress.pv = pv.clone();
                    if depth == self.depth {
                        self.progress.best_node = Some(*node);
                    }
                    self.progress.duration = self.now.elapsed();
                    self.message_sender.send(Message::AIUpdate(self.progress.clone()));
                }
                
                if beta <= alpha {
                    break;
                }
            }
            alpha
        }
        // Minimizing
        else {
            for node in &child_nodes {
                let child_score = self.alpha_beta(*node, depth-1, true, alpha, beta, &mut child_pv);
                self.progress.nodes += 1;

                if child_score < beta {
                    beta = child_score;
                    pv.clear();
                    pv.push(node.last_move.unwrap());
                    pv.append(&mut child_pv);
                }
                
                if beta <= alpha {
                    break;
                }
            }
            beta
        }
    }

    /// Scores the game from the point of view of search_player.
    /// Depth is used here to make the eval favor winning sooner (low depth) or
    /// losing later (high depth).
    fn evaluate(&self, node: &Game, depth: usize) -> f64 {
        const WIN_LOSS_VAL: f64 = 100.0;
        match node.state {
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