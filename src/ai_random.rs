// AI Random

use crate::ai::{AIProgress, Think};
use crate::controller::Message;
use crate::Game;
use crate::message_sender::MessageSender;

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
        let mut available = self.game.actions_available();
        if available.is_empty() {
            panic!("AIRandom.think: no actions available!");
        }
        let action = available.swap_remove(fastrand::usize(0..available.len()));
        
        let mut progress= AIProgress::new();
        progress.nodes = available.len() + 1;
        progress.pv.push(action);
        progress.is_complete = true;
        let return_progress = progress.clone();
        self.message_sender.send(Message::AIUpdate(progress));
        return_progress
    }
}


