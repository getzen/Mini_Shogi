// Main

use macroquad::prelude::*;

mod ai;
mod ai_minimax;
//mod ai_monte_carlo;
//mod ai_monte_carlo_tree;
//mod ai_random;
mod ai_sender;
mod asset_loader;
mod controller;
mod game;
mod piece;
mod view;

use crate::controller::Controller;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Mini Shogi"),
        window_width: 800,
        window_height: 800,
        high_dpi: true,
        //fullscreen: bool,
        //sample_count: 0,
        window_resizable: false,
        //icon: Option<Icon>,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    println!("dpi_scale: {}", crate::view::dpi_scale());
    let mut controller = Controller::new().await;
    controller.prepare().await;
    controller.go().await;
}
