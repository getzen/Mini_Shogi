// AI Monte Carlo
// This is a "pure" Monte Carlo, which is basic and unsophisticated.
// AIMonteCarloTree is supposedly stronger, but I can't *quite* get it to work.
// It also uses a lot of memory.

use crate::{Game, GameState};
use crate::ai::{AIProgress, Think};
use crate::message_sender::Message;
use crate::message_sender::MessageSender;

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

        let actions_available = self.game.actions_available();
        // Create the move to beat. With that score, any move will do.
        let mut best_action = actions_available.first().unwrap().clone();
        let mut best_score = f64::MIN;
        
        // Examine and score each move.
        for action in actions_available {
            let mut action_score = 0.0;
            
            for _ in 0..rounds {
                let mut clone = self.game.clone();
                clone.perform_action(&action, true);

                // Play out the game with randon actions.                
                while clone.update_state() == &GameState::Ongoing {
                    let mut available = clone.actions_available();
                    let rand_index = fastrand::usize(0..available.len());
                    let rand_action = available.swap_remove(rand_index);
                    clone.perform_action(&rand_action, true);
                    progress.nodes += 1;
                }

                // Below, losses are weighted heavier than wins. In tic-tac-toe, this was
                // needed to prevent moves that allowed the opponent to win in the very
                // next move. Seems hack-ish though.
                const WIN_VAL: f64 = 1.0;
                const LOSS_VAL: f64 = -7.0;
                
                match clone.state {
                    GameState::Draw => {
                        continue;
                    },
                    GameState::WinPlayer0 => {
                        if player == 0 {
                            action_score += WIN_VAL;
                        } else {
                            action_score += LOSS_VAL;
                        }
                    },
                    GameState::WinPlayer1 => {
                        if player == 1 {
                            action_score += WIN_VAL;
                        } else {
                            action_score += LOSS_VAL;
                        }
                    },
                    GameState::Ongoing => {
                        panic!("Game not completed!");
                    }
                }
                self.message_sender.send(Message::AIUpdate(progress.clone()));
            }

            // Find the move with the highest score.
            if action_score > best_score {
                best_score = action_score;
                best_action = action;
                progress.score = best_score;
            }
            progress.pv = vec![best_action.clone()];
            progress.duration = now.elapsed();
            
        }
        progress.score = best_score;
        progress.pv = vec![best_action];
        progress.duration = now.elapsed();
        progress
    }
}