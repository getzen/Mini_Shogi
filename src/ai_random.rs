// AI Random

use crate::ai::{AIProgress, Think};
use crate::Game;
use crate::message_sender::{Message, MessageSender};

pub struct AIRandom {
    game: Game,
    message_sender: MessageSender
}

impl AIRandom {
    pub fn new(game: Game, message_sender: MessageSender) -> Self {
        Self {
            game, message_sender,
        }
    }
}

impl Think for AIRandom {
    fn think(&mut self) -> AIProgress {
        let mut child_nodes = self.game.child_nodes(self.game.current_player);
        if child_nodes.is_empty() {
            panic!("AIRandom.think: no actions available!");
        }
        let node = child_nodes.swap_remove(fastrand::usize(0..child_nodes.len()));
        
        let mut progress= AIProgress::new();
        progress.nodes = child_nodes.len() + 1;
        progress.pv = vec![node.last_move.unwrap()];
        progress.best_node = Some(node);
        progress.is_complete = true;
        let return_progress = progress.clone();
        self.message_sender.send(Message::AIUpdate(progress));
        return_progress
    }
}


