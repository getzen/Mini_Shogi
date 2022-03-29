// AI Monte Carlo
// This is a "pure" Monte Carlo, which is basic and unsophisticated.
// AIMonteCarloTree is supposedly stronger, but I can't *quite* get it to work.
// It also uses a lot of memory.

use crate::{Game, GameState};
use crate::ai::{AIProgress, Think};
use crate::message_sender::{Message, MessageSender};

pub struct AIMonteCarlo {
    game: Game,
    rounds: usize,
    message_sender: MessageSender,
}

impl Think for AIMonteCarlo {
    fn think(&mut self) -> AIProgress {
        self.monte_carlo(self.rounds)
    }
}

impl AIMonteCarlo {
    pub fn new(game: Game, rounds: usize, message_sender: MessageSender) -> Self {
        Self {
            game, rounds, message_sender,
        }
    }

    fn monte_carlo(&mut self, rounds: usize) -> AIProgress {
        let now = std::time::Instant::now();
        let mut progress= AIProgress::new();
        let player = self.game.current_player;

        let child_nodes = self.game.child_nodes(player);
        // Create the move to beat. With that score, any move will do.
        let mut best_node = child_nodes.first().unwrap().clone();
        let mut best_score = f64::MIN;
        
        // Examine and score each move.
        for node in child_nodes {
            let mut node_score = 0.0;
            
            for _ in 0..rounds {
                let mut child = node.clone();

                // Play out the game by choosing random child nodes.                
                while child.update_state() == &GameState::Ongoing {
                    let mut sub_children = child.child_nodes(child.current_player);
                    let rand_index = fastrand::usize(0..sub_children.len());
                    child = sub_children.swap_remove(rand_index);
                    progress.nodes += 1;
                }

                // Below, losses are weighted heavier than wins. In tic-tac-toe, this was
                // needed to prevent moves that allowed the opponent to win in the very
                // next move. Seems hack-ish though.
                const WIN_VAL: f64 = 1.0;
                const LOSS_VAL: f64 = -7.0;
                
                match child.state {
                    GameState::Draw => {
                        continue;
                    },
                    GameState::WinPlayer0 => {
                        if player == 0 {
                            node_score += WIN_VAL;
                        } else {
                            node_score += LOSS_VAL;
                        }
                    },
                    GameState::WinPlayer1 => {
                        if player == 1 {
                            node_score += WIN_VAL;
                        } else {
                            node_score += LOSS_VAL;
                        }
                    },
                    GameState::Ongoing => {
                        panic!("Game not completed!");
                    }
                }
            }
            // Find the move with the highest score.
            if node_score > best_score {
                best_score = node_score;
                best_node = node;
                progress.score = best_score;
            }
            progress.pv = vec![best_node.last_move.unwrap()];
            progress.duration = now.elapsed();
            self.message_sender.send(Message::AIUpdate(progress.clone()));       
        }
        progress.best_node = Some(best_node);
        progress.duration = now.elapsed();
        progress
    }
}