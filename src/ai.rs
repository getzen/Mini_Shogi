// AI
// The controller for AI thinking.

use std::time::Duration;

use crate::Action;
use crate::ai_random::AIRandom;
use crate::ai_minimax::AIMinimax;
use crate::ai_monte_carlo::AIMonteCarlo;
//use crate::ai_monte_carlo_tree::AIMonteCarloTree;

use crate::message_sender::Message;
use crate::controller::PlayerKind;
use crate::controller::PlayerKind::*;

use crate::Game;
use crate::message_sender::MessageSender;

pub trait Think {
    fn think(&mut self) -> AIProgress;
}

#[derive(Debug, Clone)]
pub struct AIProgress {
    pub is_complete: bool,
    pub nodes: usize,
    pub pv: Vec<Action>,
    pub duration: Duration,
    pub score: f64,
}

impl AIProgress {
    pub fn new() -> Self {
        Self {
            is_complete: false,
            nodes: 0,
            pv: Vec::<Action>::new(),
            duration: Duration::new(0, 0),
            score: 0.0,
        }
    }
}

pub struct AI {}

impl AI {
    pub fn think(ai_kind: PlayerKind, game: Game, mut message_sender: MessageSender) {
        let mut sender_clone = message_sender.clone();
        let progress: AIProgress = match ai_kind {
            AIRandom => {
                let mut ai = AIRandom::new(game, sender_clone);
                ai.think()
            },
            AIMinimax => {
                sender_clone.min_time_between = Some(Duration::from_millis(100));
                let mut ai = AIMinimax::new(game, 1, sender_clone);
                ai.think()
            },
            AIMonteCarlo => {
                sender_clone.min_time_between = Some(Duration::from_millis(100));
                let mut ai = AIMonteCarlo::new(game, 2000, sender_clone);
                ai.think()
            },
            // AIMonteCarloTree => {
            //     sender_clone.min_time_between = Some(Duration::from_millis(100));
            //     let mut ai = AIMonteCarloTree::new(game, Duration::from_millis(1000), sender_clone);
            //     ai.think()
            // },
            _ => {panic!("AI::think panic!")},
        };
        message_sender.send(Message::SearchCompleted(progress));
    }
}