// Main

use macroquad::prelude::*;

mod action;
use action::Action;
use action::ActionKind;
mod ai;
mod ai_minimax;
mod ai_monte_carlo;
mod ai_monte_carlo_tree;
mod ai_random;
mod controller;
use crate::controller::Controller;
mod game;
use game::{Game, GameState};
mod lerp;
mod message_sender;
use message_sender::{Message, MessageSender};
mod piece;
use piece::Piece;
mod sprite;
mod text;
mod view_game;
mod view_intro;

fn conf() -> Conf {
    Conf {
        window_title: String::from("My Game"),
        window_width: 800,
        window_height: 800,
        high_dpi: true,
        //fullscreen: bool,
        //sample_count: i32,
        //window_resizable: bool,
        //icon: Option<Icon>,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut controller = Controller::new().await;
    controller.prepare().await;
    controller.go().await;
}
