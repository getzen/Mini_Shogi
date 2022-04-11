// AI
// The controller for AI thinking.

use std::time::Duration;

use crate::ai_random::AIRandom;
use crate::ai_minimax::AIMinimax;
use crate::ai_monte_carlo::AIMonteCarlo;
//use crate::ai_monte_carlo_tree::AIMonteCarloTree;
use crate::ai_sender::{AISender, AIMessage};

use crate::controller::Player;
use crate::controller::PlayerKind::*;
use crate::game::{Game, Move};

pub trait Think {
    fn think(&mut self) -> AIProgress;
}

#[derive(Debug, Clone)]
pub struct AIProgress {
    pub is_complete: bool,
    pub nodes: usize,
    pub pv: Vec<Move>,
    pub duration: Duration,
    pub score: f64,
    pub best_node: Option<Game>,
}

impl AIProgress {
    pub fn new() -> Self {
        Self {
            is_complete: false,
            nodes: 0,
            pv: Vec::new(),
            duration: Duration::new(0, 0),
            score: 0.0,
            best_node: None,
        }
    }
}

pub struct AI {}

impl AI {
    pub fn think(player: Player, game: Game, mut message_sender: AISender) {
        let mut sender_clone = message_sender.clone();
        let mut kind = player.kind;
        if kind == AIMinimax && player.search_depth == 0 {
            kind = AIRandom;
        }
        if kind == AIMonteCarlo && player.search_rounds == 0 {
            kind == AIRandom;
        }
        let progress: AIProgress = match kind {
            AIRandom => {
                let mut ai = AIRandom::new(game, sender_clone);
                ai.think()
            },
            AIMinimax => {
                sender_clone.min_time_between = Some(Duration::from_millis(100));
                let mut ai = AIMinimax::new(game, player.search_depth, sender_clone);
                ai.think()
            },
            AIMonteCarlo => {
                sender_clone.min_time_between = Some(Duration::from_millis(100));
                let mut ai = AIMonteCarlo::new(game, player.search_rounds, sender_clone);
                ai.think()
            },
            // AIMonteCarloTree => {
            //     sender_clone.min_time_between = Some(Duration::from_millis(100));
            //     let mut ai = AIMonteCarloTree::new(game, Duration::from_millis(1000), sender_clone);
            //     ai.think()
            // },
            _ => {panic!("AI::think panic!")},
        };
        message_sender.send(AIMessage::SearchCompleted(progress));
    }
}